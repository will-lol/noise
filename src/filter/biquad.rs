use std::f32::consts::PI;

use cpal::{FromSample, Sample};

use super::filter::Filter;

pub struct Biquad {
    pub a0: f32,
    pub a1: f32,
    pub a2: f32,
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    xn1: f32,
    xn2: f32,
}

impl<T> Filter<T> for Biquad
where
    T: Sample + FromSample<f32>,
    f32: FromSample<T>,
{
    fn apply(&mut self, arr: &mut [T]) {
        for val in arr.iter_mut() {
            let v = val.to_sample::<f32>();
            let out = (self.b0 / self.a0) * v
                + (self.b1 / self.a0) * self.xn1
                + (self.b2 / self.a0) * self.xn2
                - (self.a1 / self.a0) * self.xn1
                - (self.a2 / self.a0) * self.xn2;
            *val = out.to_sample::<T>();
            self.xn2 = self.xn1;
            self.xn1 = v;
        }
    }
}

impl Biquad {
    pub fn new(a0: f32, a1: f32, a2: f32, b0: f32, b1: f32, b2: f32) -> Self {
        return Self {
            a0,
            a1,
            a2,
            b0,
            b1,
            b2,
            xn1: 0.0,
            xn2: 0.0,
        };
    }

    pub fn new_bandpass(sampling_frequency: f32, center_frequency: f32, q_factor: f32) -> Self {
        let w = 2.0 * PI * (center_frequency / sampling_frequency);
        let alpha = f32::sin(w) / 2.0 * q_factor;
        return Self::new(
            1.0 + alpha,
            -2.0 * f32::cos(w),
            1.0 - alpha,
            q_factor * alpha,
            0.0,
            q_factor * alpha * -1.0,
        );
    }

    pub fn new_peaking_eq(
        sampling_frequency: f32,
        center_frequency: f32,
        db: f32,
        q_factor: f32,
    ) -> Self {
        let w = 2.0 * PI * (center_frequency / sampling_frequency);
        let alpha = f32::sin(w) / 2.0 * q_factor;
        let a = f32::powf(10.0, db / 40.0);
        return Self::new(
            1.0 + alpha / a,
            -2.0 * f32::cos(w),
            1.0 - alpha / a,
            1.0 + alpha * a,
            -2.0 * f32::cos(w),
            1.0 - alpha * a,
        );
    }
}
