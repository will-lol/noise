use std::sync::{Arc, Mutex};

use crate::filter;

pub struct App {
    pub currently_changing: u8,
    pub vals: [i8; 7],
    pub filter: Arc<Mutex<filter::biquad::StreamBiquadFilter>>,
}

impl App {
    pub fn new(filter: Arc<Mutex<filter::biquad::StreamBiquadFilter>>) -> App {
        App {
            currently_changing: 0,
            vals: [0; 7],
            filter,
        }
    }
}
