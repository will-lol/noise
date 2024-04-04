pub trait Filter<T: cpal::Sample> {
    fn run(&mut self, input: T) -> T;
}

pub trait StreamFilter<T: cpal::Sample> {
    fn process(&mut self, data: &mut [T]);
    fn set_coefs(&mut self, coefs: super::coefs::Coefficients);
}
