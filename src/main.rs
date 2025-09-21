use color_eyre::Result;
use std::env;
use std::time::{Duration, Instant};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    cursor::Show,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    style::Color,
    Frame,
    Terminal,
};

use tui_rain::{CharacterSet, Rain, RainSpeed};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("Chalbik - Klingon Rain TUI");
        println!("\nUsage: chalbik [OPTIONS]");
        println!("\nOptions:");
        println!("  -t, --tail-color <COLOR>    Rain trail color (default: red)");
        println!("  -d, --head-color <COLOR>    Leading drop color (default: yellow)");
        println!("  -s, --speed <SPEED>         Speed: slow|fast (default: fast)");
        println!("  -l, --tail-length <LENGTH>  Trail lifespan in seconds (default: 10)");
        println!("  -h, --help                  Print help");
        println!("\nColors: black red green yellow blue magenta cyan white light_green etc.");
        println!("\nQuit: q or Esc");
        return Ok(());
    }

    let mut tail_color_arg = "red".to_string();
    let mut head_color_arg = "yellow".to_string();
    let mut speed_arg = "fast".to_string();
    let mut tail_length_arg = "10".to_string();

    let mut i = 1;
    while i < args.len() {
        let current = &args[i];
        match current.as_str() {
            "--tail-color" | "-t" => {
                i += 1;
                if i < args.len() {
                    tail_color_arg = args[i].clone();
                }
            }
            "--head-color" | "-d" => {
                i += 1;
                if i < args.len() {
                    head_color_arg = args[i].clone();
                }
            }
            "--speed" | "-s" => {
                i += 1;
                if i < args.len() {
                    speed_arg = args[i].clone();
                }
            }
            "--tail-length" | "-l" => {
                i += 1;
                if i < args.len() {
                    tail_length_arg = args[i].clone();
                }
            }
            "-h" | "--help" => {
                // Handled above
                return Ok(());
            }
            _ => {}
        }
        i += 1;
    }

    let tail_color = match tail_color_arg.as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "dark_gray" => Color::DarkGray,
        "light_red" => Color::LightRed,
        "light_green" => Color::LightGreen,
        "light_yellow" => Color::LightYellow,
        "light_blue" => Color::LightBlue,
        "light_magenta" => Color::LightMagenta,
        "light_cyan" => Color::LightCyan,
        "gray" | "light_gray" => Color::Gray,
        _ => Color::Red,
    };

    let head_color = match head_color_arg.as_str() {
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "dark_gray" => Color::DarkGray,
        "light_red" => Color::LightRed,
        "light_green" => Color::LightGreen,
        "light_yellow" => Color::LightYellow,
        "light_blue" => Color::LightBlue,
        "light_magenta" => Color::LightMagenta,
        "light_cyan" => Color::LightCyan,
        "gray" | "light_gray" => Color::Gray,
        _ => Color::Yellow,
    };

    let rain_speed = match speed_arg.as_str() {
        "slow" => RainSpeed::Slow,
        "fast" => RainSpeed::Fast,
        _ => RainSpeed::Fast,
    };

    let tail_lifespan_secs: u64 = tail_length_arg.parse().unwrap_or(10);
    let tail_lifespan = Duration::from_secs(tail_lifespan_secs);

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_start_time = Instant::now();
    loop {
        terminal.draw(|f| ui(f, app_start_time.elapsed(), tail_color, head_color, tail_lifespan, rain_speed))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, Show)?;
    Ok(())
}

fn ui(f: &mut Frame, elapsed: Duration, tail_color: Color, head_color: Color, tail_lifespan: Duration, rain_speed: RainSpeed) {
    let area = f.area();

    // Full pIqaD charset with bad ranges filtered
    let options: Vec<char> = (0xF8D0..=0xF8FF)
        .filter_map(|code| char::from_u32(code))
        .filter(|&c| {
            let code = c as u32;
            let i = code - 0xF8D0;
            !((i >= 0x1A && i <= 0x1F) || (i >= 0x2A && i <= 0x2D))
        })
        .collect();

    let charset = CharacterSet::Explicit {
        options,
    };

    let rain = Rain::new_matrix(elapsed)
        .with_character_set(charset)
        .with_color(tail_color)
        .with_head_color(head_color)
        .with_tail_lifespan(tail_lifespan)
        .with_rain_speed(rain_speed);

    f.render_widget(rain, area);
}