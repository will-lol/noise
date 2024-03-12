pub trait Filter<T: cpal::Sample> {
    fn apply(&mut self, arr: &mut [T]);
}
