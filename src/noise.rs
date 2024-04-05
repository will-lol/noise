use std::sync::{Arc, Mutex};

use cpal::{traits::{DeviceTrait, StreamTrait}, SampleFormat};
use rand::Rng;

use crate::constants::{FREQUENCIES, Q_FACTOR};
use crate::{constants, filter::biquad::StreamBiquadFilter};
use crate::filter::coefs::Coefficients;
use crate::filter::filter::StreamFilter;

pub struct NoiseMaker {
    filters: [Arc<Mutex<StreamBiquadFilter>>; FREQUENCIES.len()],
    config: cpal::StreamConfig,
    stream: cpal::Stream,
}

impl NoiseMaker {
    pub fn new(device: &cpal::Device) -> anyhow::Result<Self> {
        let configs: Vec<_> = device
            .supported_output_configs()
            .expect("Error while querying configs.")
            .collect();

        let config = configs
            .iter()
            .filter(|x| x.channels() == 2)
            .find(|x| x.sample_format() == SampleFormat::F32)
            .unwrap_or_else(|| configs.get(0).expect(""))
            .clone()
            .with_max_sample_rate();

        let channels = config.channels();
        let sample_rate = config.sample_rate().0 as f32;

        let filters: [Arc<Mutex<StreamBiquadFilter>>; 15] = [
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[0], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[1], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[2], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[3], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[4], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[5], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[6], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[7], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[8], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[9], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[10], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[11], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[12], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[13], 0.0, Q_FACTOR)))),
            Arc::new(Mutex::new(StreamBiquadFilter::new(channels, &Coefficients::new_peaking_eq(sample_rate, FREQUENCIES[14], 0.0, Q_FACTOR)))),
        ];

        let sample_format = config.sample_format();
        let stream_config: cpal::StreamConfig = config.into();

        let stream = match sample_format {
            cpal::SampleFormat::I8 => Self::build_stream::<i8>(&stream_config, &device, &filters),
            cpal::SampleFormat::I16 => Self::build_stream::<i16>(&stream_config, &device, &filters),
            cpal::SampleFormat::I32 => Self::build_stream::<i32>(&stream_config, &device, &filters),
            cpal::SampleFormat::I64 => Self::build_stream::<i64>(&stream_config, &device, &filters),
            cpal::SampleFormat::U8 => Self::build_stream::<u8>(&stream_config, &device, &filters),
            cpal::SampleFormat::U16 => Self::build_stream::<u16>(&stream_config, &device, &filters),
            cpal::SampleFormat::U32 => Self::build_stream::<u32>(&stream_config, &device, &filters),
            cpal::SampleFormat::U64 => Self::build_stream::<u64>(&stream_config, &device, &filters),
            cpal::SampleFormat::F32 => Self::build_stream::<f32>(&stream_config, &device, &filters),
            cpal::SampleFormat::F64 => Self::build_stream::<f64>(&stream_config, &device, &filters),
            _ => unreachable!("Unsupported sample format"),
        }?;

        return Ok(Self {
            stream, filters, config: stream_config
        });
    }

    fn build_stream<T>(config: &cpal::StreamConfig, device: &cpal::Device, filters: &[Arc<Mutex<StreamBiquadFilter>>; constants::FREQUENCIES.len()]) -> anyhow::Result<cpal::Stream>
    where
        T: cpal::SizedSample + cpal::FromSample<f32> + 'static,
        rand::distributions::Standard: rand::distributions::Distribution<T>,
        f32: cpal::FromSample<T>,
    {
        let filters_stream = filters.clone();
        return Ok(device.build_output_stream(
            &config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                let mut rng = rand::thread_rng();
                data.fill_with(|| rng.gen::<T>());
                filters_stream.iter().for_each(|f| {
                    f.lock().unwrap().process(data);
                })
                
            },
            move |err| println!("{:?}", err),
            None,
        )?);
    }

    pub fn set_filter_gain(&self, i: usize, db: f32) {
        return self.filters[i].lock().unwrap().set_coefs(Coefficients::new_peaking_eq(self.config.sample_rate.0 as f32, FREQUENCIES[i], db, Q_FACTOR));
    }

    pub fn play(&self) -> anyhow::Result<()> {
        self.stream.play()?;
        return Ok(());
    }
}
