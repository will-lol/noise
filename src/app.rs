use crate::{constants, noise};

pub struct App {
    pub currently_changing: usize,
    pub vals: [f32; constants::FREQUENCIES.len()],
    pub noise: noise::NoiseMaker,
}

impl App {
    pub fn new(noise: noise::NoiseMaker, preset: [f32; constants::FREQUENCIES.len()]) -> App {
        App {
            currently_changing: 0,
            vals: preset,
            noise,
        }
    }
}
