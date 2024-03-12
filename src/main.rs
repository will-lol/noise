use anyhow::Result;
mod filter;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, SampleFormat, SizedSample,
};
use filter::filter::Filter;
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device detected.");

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

    match config.sample_format() {
        cpal::SampleFormat::I8 => noise::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => noise::<i16>(&device, &config.into()),
        cpal::SampleFormat::I32 => noise::<i32>(&device, &config.into()),
        cpal::SampleFormat::I64 => noise::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => noise::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => noise::<u16>(&device, &config.into()),
        cpal::SampleFormat::U32 => noise::<u32>(&device, &config.into()),
        cpal::SampleFormat::U64 => noise::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => noise::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => noise::<f64>(&device, &config.into()),
        _ => unreachable!("Unsupported sample format"),
    }?;

    return Ok(());
}

fn noise<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32> + 'static,
    Standard: Distribution<T>, f32: FromSample<T>
{
    let mut peaking = filter::biquad::Biquad::new_peaking_eq(25000.0, 1000.0, -1000.0, 10.0);
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let mut rng = thread_rng();
            data.fill_with(|| rng.gen::<T>());
            peaking.apply(data);
        },
        move |err| println!("{:?}", err),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_millis(1000000));
    Ok(())
}

