



mod app;
mod filter;
mod constants;
mod noise;
mod slider;
mod ui;
use app::App;
use constants::{MAXIMUM_DB, MINIMUM_DB};
use cpal::traits::HostTrait;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use noise::NoiseMaker;
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;
        if let Event::Key(key) = event::read()? {
            // only register presses
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    break;
                },
                KeyCode::Char('h') | KeyCode::Left | KeyCode::BackTab => {
                    if app.currently_changing != 0 {
                        app.currently_changing = app.currently_changing - 1;
                    } else {
                        app.currently_changing = app.vals.len() - 1;
                    }
                },
                KeyCode::Char('j') | KeyCode::Down => {
                    let mut val = app.vals[app.currently_changing] - constants::STEP;
                    if val < MINIMUM_DB {
                        val = MINIMUM_DB;
                    }
                    app.vals[app.currently_changing] = val;
                    app.noise.set_filter_gain(app.currently_changing, app.vals[app.currently_changing]);
                },
                KeyCode::Char('k') | KeyCode::Up => {
                    let mut val = app.vals[app.currently_changing] + constants::STEP;
                    if val > MAXIMUM_DB {
                        val = MAXIMUM_DB;
                    }
                    app.vals[app.currently_changing] = val;
                    app.noise.set_filter_gain(app.currently_changing, app.vals[app.currently_changing]);
                },
                KeyCode::Char('l') | KeyCode::Right | KeyCode::Tab => {
                    if (app.currently_changing as usize) != app.vals.len() - 1 {
                        app.currently_changing = app.currently_changing + 1;
                    } else {
                        app.currently_changing = 0;
                    }
                }
                _ => {},
            }
        }
    }
    return Ok(());

}

fn main() -> anyhow::Result<()> {
    let _l = simple_logging::log_to_file("test.log", log::LevelFilter::Debug);
    let device = cpal::default_host()
        .default_output_device()
        .expect("No default output device detected");
    let noise = NoiseMaker::new(&device)?;

    // pre run
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run
    noise.play()?;
    let mut app = app::App::new(noise);
    let _res = run_app(&mut terminal, &mut app);

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
