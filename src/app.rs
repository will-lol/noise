use crate::{constants, noise};

pub struct App {
    pub currently_changing: usize,
    pub vals: [f32; constants::FREQUENCIES.len()],
    pub noise: noise::NoiseMaker,
}

impl App {
    pub fn new(noise: noise::NoiseMaker) -> App {
        App {
            currently_changing: 0,
            vals: [0.0; constants::FREQUENCIES.len()],
            noise,
        }
    }
}
