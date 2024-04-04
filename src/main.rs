use std::sync::{Arc, Mutex};

use anyhow::Result;
mod app;
mod filter;
mod noise;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, SampleFormat, SizedSample,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use filter::filter::StreamFilter;
use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};
use ratatui::{backend::CrosstermBackend, Terminal};

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
        .filter(|x| x.channels() == 8)
        .find(|x| x.sample_format() == SampleFormat::F32)
        .unwrap_or_else(|| configs.get(0).expect(""))
        .clone()
        .with_max_sample_rate();

    let conf: cpal::StreamConfig = config.into();

    let filter = match config.sample_format() {
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

    // pre run
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run
    let mut app = app::App::new(filter);
    let res = run_app(&mut terminal, &mut app);

    // post run
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    return Ok(());
}

fn noise<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> Result<Arc<Mutex<filter::biquad::StreamBiquadFilter>>, anyhow::Error>
where
    T: SizedSample + FromSample<f32> + 'static,
    Standard: Distribution<T>,
    f32: FromSample<T>,
{
    let coefs =
        filter::coefs::Coefficients::new_peaking_eq(config.sample_rate.0 as f32, 100.0, 30.0, 0.6);

    let filter = Arc::new(Mutex::new(filter::biquad::StreamBiquadFilter::new(
        config.channels,
        &coefs,
    )));
    let filter_cloned = filter.clone();

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let mut rng = thread_rng();
            data.fill_with(|| rng.gen::<T>());
            filter_cloned.lock().unwrap().process(data);
        },
        move |err| println!("{:?}", err),
        None,
    )?;

    let coefs2 =
        filter::coefs::Coefficients::new_peaking_eq(config.sample_rate.0 as f32, 500.0, 25.0, 0.6);

    std::thread::sleep(std::time::Duration::from_millis(500));
    filter.lock().unwrap().set_coefs(coefs2);

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_millis(1000000));
    Ok(filter)
}
