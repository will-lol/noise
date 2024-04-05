use cpal::{FromSample, Sample};

use super::filter::{Filter, StreamFilter};

pub struct Biquad {
    coefs: super::coefs::Coefficients,
    xn1: f32,
    xn2: f32,
    yn1: f32,
    yn2: f32,
    i: usize,
}

impl<T> Filter<T> for Biquad
where
    T: Sample + FromSample<f32>,
    f32: FromSample<T>,
{
    fn run(&mut self, input: T) -> T {
        let out: f32;
        let i = input.to_sample::<f32>();

        if self.i == 0 {
            out = self.coefs.b0 * i;
        } else if self.i == 1 {
            out = (self.coefs.b0 * i) + (self.coefs.b1 * self.xn1) - (self.coefs.a1 * self.yn1);
        } else {
            out = ((self.coefs.b0 * i) + (self.coefs.b1 * self.xn1) + (self.coefs.b2 * self.xn2))
                - ((self.coefs.a1 * self.yn1) + (self.coefs.a2 * self.yn2));
        }

        self.xn2 = self.xn1;
        self.xn1 = i;

        self.yn2 = self.yn1;
        self.yn1 = out;

        self.i += 1;

        return out.to_sample();
    }
}

impl Biquad {
    pub fn new(coefs: &super::coefs::Coefficients) -> Self {
        let mut coefs = *coefs;
        Self::normalise_coefs(&mut coefs);

        let filter = Self {
            coefs,
            yn1: 0.0,
            yn2: 0.0,
            xn1: 0.0,
            xn2: 0.0,
            i: 0,
        };

        return filter;
    }
    pub fn set_coefs(&mut self, coefs: super::coefs::Coefficients) {
        let mut coefs = coefs;
        Self::normalise_coefs(&mut coefs);
        self.coefs = coefs;
    }
    fn normalise_coefs(coefs: &mut super::coefs::Coefficients) {
        let scale = 1.0 / coefs.a0;
        coefs.a0 *= scale;
        coefs.a1 *= scale;
        coefs.a2 *= scale;
        coefs.b0 *= scale;
        coefs.b1 *= scale;
        coefs.b2 *= scale;
    }
}

pub struct StreamBiquadFilter {
    channels: u16,
    filters: Vec<super::biquad::Biquad>,
}

impl StreamBiquadFilter {
    pub fn new(channels: u16, biquad_coefs: &super::coefs::Coefficients) -> Self {
        let filters: Vec<super::biquad::Biquad> = (0..channels)
            .map(|_| super::biquad::Biquad::new(biquad_coefs))
            .collect();
        Self { channels, filters }
    }
}

impl StreamFilter for StreamBiquadFilter {
    fn process<T>(&mut self, data: &mut [T])
    where
        T: cpal::Sample + cpal::FromSample<f32>,
        f32: FromSample<T>,
    {
        for i in 1..=(self.channels) {
            data.iter_mut()
                .skip((self.channels - i) as usize)
                .step_by(self.channels as usize)
                .for_each(|x| *x = self.filters[(i - 1) as usize].run(*x));
        }
    }
    fn set_coefs(&mut self, coefs: super::coefs::Coefficients) {
        self.filters.iter_mut().for_each(|x| x.set_coefs(coefs));
    }
}
