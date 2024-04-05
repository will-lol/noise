pub trait Filter<T: cpal::Sample> {
    fn run(&mut self, input: T) -> T;
}

pub trait StreamFilter {
    fn process<T: cpal::Sample>(&mut self, data: &mut [T])
    where
        T: cpal::Sample + cpal::FromSample<f32>,
        f32: cpal::FromSample<T>;
    fn set_coefs(&mut self, coefs: super::coefs::Coefficients);
}
