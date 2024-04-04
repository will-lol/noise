use cpal::{traits::DeviceTrait, SampleFormat};

use crate::filter;

pub struct NoiseMaker {
    filters: [filter::biquad::StreamBiquadFilter; 7],
    config: cpal::StreamConfig,
    device: &cpal::Device,
    sample_format: cpal::SampleFormat,
}

impl NoiseMaker {
    pub fn new(device: &cpal::Device) -> Self {
        let configs: Vec<_> = device
            .supported_output_configs()
            .expect("Error while querying configs.")
            .collect();

        let config = configs
            .iter()
            .filter(|x| x.channels() == 8)
            .find(|x| x.sample_format() == SampleFormat::F32)
            .unwrap_or_else(|| configs.get(0).expect(""))
            .clone()
            .with_max_sample_rate();

        let sample_format = config.sample_format();

        let config: cpal::StreamConfig = config.into();

        return Self {
            config, sample_format, device, 
        }
        
    }
}
