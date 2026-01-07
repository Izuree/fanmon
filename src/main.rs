use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};
use std::env;
use unicode_width::UnicodeWidthStr;


static FAN_FRAMES: [&str; 4] = [
r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠙⣷⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⣸⠇⠀⠀⠀⠀⠀⠀
⠀⢀⣴⠞⠛⠛⠛⠶⣤⡶⢛⣛⡛⢾⣏⠀⠀⠀⠀⠀⠀⠀
⢀⣾⣁⣀⣀⣀⣀⣰⡏⣼⠟⠉⠻⣦⢹⣄⣀⣀⣀⣀⣀⣀
⠀⠉⠉⠉⠉⠉⠉⠹⣇⠻⣦⣀⣴⠟⣸⠏⠉⠉⠉⠉⢉⡿
⠀⠀⠀⠀⠀⠀⠀⠀⣹⠷⣬⣭⡵⠾⠛⠶⢤⣤⣤⠶⠟⠁
⠀⠀⠀⠀⠀⠀⠀⢠⡏⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢷⣄⡀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠙⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"#,

r#"

⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡿⠻⣦⠀⠀⠀⠀
⠀⠀⢀⣤⣤⣶⣦⣤⡀⠀⠀⠀⠀⢠⣿⠁⠀⢸⡇⠀⠀⠀
⠀⠀⣼⡋⠁⠀⠈⠉⢳⣆⠀⠀⠀⣼⠃⠀⠀⣾⠇⠀⠀⠀
⠀⠀⠈⠛⠳⢶⣤⣀⣠⡾⢛⣉⣛⠻⣦⣤⡾⠃⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢹⡏⣼⠟⠉⠻⣦⠨⣧⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢘⣧⠻⣦⣀⣴⠟⣰⣯⣄⡀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢀⡾⠟⠘⢻⣬⣭⡵⢾⡛⠀⠈⠛⠻⠶⣤⣀
⠀⠀⠀⠀⠀⣿⠁⠀⢀⣼⠃⠀⠀⠈⢻⣦⣄⣀⣠⣤⣾⠁
⠀⠀⠀⠀⠀⣿⡀⠀⣼⠇⠀⠀⠀⠀⠀⠈⠉⠛⠋⠉⠀⠀
⠀⠀⠀⠀⠀⠈⢷⣴⠏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠈⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"#,

r#"

⠀⠀⠀⠀⠀⣀⣤⣤⣤⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠻⣏⠉⠉⠻⣶⡀⠀⠀⠀⠀⠀⠀⢀⡀⠀
⠀⠀⠀⠀⠀⠀⠙⢷⣄⠀⠈⣿⠀⠀⠀⠀⣠⡴⠏⢿⡀
⠀⠀⠀⠀⠀⠀⠀⠀⢹⣶⢛⣛⣛⢶⣤⡞⠉⠀⢀⣾⠁
⠀⠀⠀⠀⠀⠀⠀⢹⡏⣼⠟⠉⠻⣦⢻⣆⣠⣤⡞⠃⠀
⠀⠀⠀⠀⣴⠾⠛⣿⡀⠻⣦⣀⣴⠟⢸⡏⠉⠉⠀⠀⠀
⠀⠀⠀⣰⡟⠀⠀⣠⡟⢷⣬⣭⣭⡴⣟⠀⠀⠀⠀⠀⠀
⠀⠀⠀⢿⣄⣤⠾⠋⠀⠀⠀⢹⡇⠀⠈⢷⡄⠀⠀⠀⠀
⠀⠀⠀⠈⠛⠁⠀⠀⠀⠀⠀⠀⢻⣤⡀⠀⢹⣦⡀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠳⠶⠶⠟⠁⠀⠀

"#,

r#"
⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢻⡏⠉⠛⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠈⣷⠀⠀⢹⡆⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣇⢀⣨⡇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⣀⣀⡀⣠⡾⢛⣛⣛⠳⣤⢀⣀⣤⡤⠶⢻⡇
⠀⠀⢠⣴⡟⠛⠉⢻⡏⣼⠟⠉⠻⣦⠛⡟⠉⠁⠀⢀⣼⠃
⠀⢀⣾⠃⠀⢀⣀⣼⣄⠻⣦⣀⣴⠟⣰⣧⣄⣤⣴⡾⠋⠀
⠀⠘⡷⡶⠟⠋⠉⠁⠙⠷⣬⣭⣭⡶⠋⠀⠉⠉⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⡟⠀⠘⣇⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣧⠀⠀⢹⡆⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣦⣄⣈⣷⡀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀
"#,
];
fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}


fn flush() {
    io::stdout().flush().unwrap();
}

use std::process::Command;

#[derive(Debug)]
struct Config {
    speed_scale: f32,
    hide_text: bool,
    center: bool,
    color: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            speed_scale: 1.0,
            hide_text: false,
            center: false,
            color: None,
        }
    }
}

fn print_help() {
    println!("ASCII Fan Monitor - Terminal fan animation NBFC service listener");
    println!("Author: deutereum");
    println!("USAGE:");
    println!("    ascii-fan [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help         Show this help message");
    println!("    -s, --speed <NUM>  Speed scaling factor (default: 1.0)");
    println!("    -c, --color <COLOR> Set fan color (red, yellow, blue, green, cyan, purple)");
    println!("    --hide-text        Hide speed and status text");
    println!("    -C, --center       Center the fan animation");
}

fn get_color_code(color: &str) -> &'static str {
    match color.to_lowercase().as_str() {
        "red" => "\x1b[31m",
        "yellow" => "\x1b[33m",
        "blue" => "\x1b[34m",
        "green" => "\x1b[32m",
        "cyan" => "\x1b[36m",
        "purple" => "\x1b[35m",
        _ => "\x1b[37m", // white default
    }
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            "-s" | "--speed" => {
                if i + 1 >= args.len() {
                    return Err("Speed value required".to_string());
                }
                config.speed_scale = args[i + 1].parse()
                    .map_err(|_| "Invalid speed value".to_string())?;
                i += 1;
            }
            "-c" | "--color" => {
                if i + 1 >= args.len() {
                    return Err("Color value required".to_string());
                }
                let color = &args[i + 1];
                if !["red", "yellow", "blue", "green", "cyan", "purple"].contains(&color.to_lowercase().as_str()) {
                    return Err("Invalid color. Use: red, yellow, blue, green, cyan, purple".to_string());
                }
                config.color = Some(color.clone());
                i += 1;
            }
            "--hide-text" => {
                config.hide_text = true;
            }
            "-C" | "--center" => {
                config.center = true;
            }
            _ => return Err(format!("Unknown option: {}", args[i])),
        }
        i += 1;
    }

    Ok(config)
}

struct FanStatus {
    current_speed: f32,
    critical: bool,
}

fn get_fan_status() -> Option<FanStatus> {
    let output = Command::new("nbfc")
        .arg("status")
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);

    let mut current_speed: Option<f32> = None;
    let mut critical: Option<bool> = None;

    for line in text.lines() {
        if line.contains("Current Fan Speed") {
            let value = line.split(':').nth(1)?;
            current_speed = value.trim().parse::<f32>().ok();
        }

        if line.contains("Critical Mode Enabled") {
            let value = line.split(':').nth(1)?;
            critical = match value.trim() {
                "true" => Some(true),
                "false" => Some(false),
                _ => None,
            };
        }
    }

    Some(FanStatus {
        current_speed: current_speed?,
        critical: critical?,
    })
}

const BASE_SPEED_SCALE: f32 = 0.3;

fn speed_to_delay(speed: f32, critical: bool, speed_scale: f32) -> u64 {
    let base_delay = if critical {
        40
    } else {
        match speed as u32 {
            0..=10 => 400,
            11..=25 => 250,
            26..=40 => 180,
            41..=60 => 120,
            61..=80 => 80,
            _ => 60,
        }
    };

    ((base_delay as f32) * BASE_SPEED_SCALE / speed_scale) as u64
}

fn get_terminal_size() -> (u16, u16) {
    if let Ok(output) = Command::new("tput").args(&["lines"]).output() {
        if let Ok(rows_str) = String::from_utf8(output.stdout) {
            if let Ok(rows) = rows_str.trim().parse::<u16>() {
                if let Ok(output) = Command::new("tput").args(&["cols"]).output() {
                    if let Ok(cols_str) = String::from_utf8(output.stdout) {
                        if let Ok(cols) = cols_str.trim().parse::<u16>() {
                            return (rows, cols);
                        }
                    }
                }
            }
        }
    }
    (24, 80)
}

fn print_centered_at_position(text: &str, term_rows: u16, term_cols: u16) {
    let lines: Vec<&str> = text.lines().collect();

    let content_height = lines.len();
    let content_width = lines
        .iter()
        .map(|l| UnicodeWidthStr::width(*l))
        .max()
        .unwrap_or(0);

    let fits_vertically = content_height <= term_rows as usize;
    let fits_horizontally = content_width <= term_cols as usize;

    if !fits_vertically || !fits_horizontally {
        print!("{}", text);
        return;
    }

    let start_row = (term_rows as usize - content_height) / 2 + 1;
    let start_col = (term_cols as usize - content_width) / 2 + 1;

    for (i, line) in lines.iter().enumerate() {
        print!("\x1b[{};{}H{}", start_row + i, start_col, line);
    }
}


fn main() {
    print!("\x1b[?1049h\x1b[H");
    print!("\x1b[?25l");
    flush();

    let config = match parse_args() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {}", err);
            eprintln!("Use -h or --help for usage information");
            std::process::exit(1);
        }
    };

    let mut frame_idx = 0;

    print!("\x1b[?25l");

    loop {
        clear_screen();

        let status = get_fan_status();

        let (delay, critical) = if let Some(ref s) = status {
            (
                speed_to_delay(s.current_speed, s.critical, config.speed_scale),
                s.critical,
            )
        } else {
            (200, false)
        };

        // Set color - critical overrides custom color
        if critical {
            print!("\x1b[31m");
        } else if let Some(ref color) = config.color {
            print!("{}", get_color_code(color));
        }

        if config.center {
            let (term_rows, term_cols) = get_terminal_size();
            
            let mut content = FAN_FRAMES[frame_idx].to_string();
            
            if !config.hide_text {
                if let Some(ref s) = status {
                    let display_speed = if s.current_speed < 6.0 { 0.0 } else { s.current_speed };
                    content.push_str(&format!("\nSpeed    : {:.2}%\nCritical : {}", display_speed, s.critical));
                } else {
                    content.push_str("\nNBFC service unavailable, is the service running?");
                }
            }
            print!("\x1b[0;0H");
            print_centered_at_position(&content, term_rows, term_cols);
        } else {
         print!("{}", FAN_FRAMES[frame_idx]);
    
         if !config.hide_text {
            if let Some(ref s) = status {
                let display_speed = if s.current_speed < 6.0 { 0.0 } else { s.current_speed };
                print!("\nSpeed    : {:.2}%", display_speed);
                print!("\nCritical : {}", s.critical);
             }
            }
        }


        // Reset color
        if critical || config.color.is_some() {
            print!("\x1b[0m");
        }

        if let Some(ref s) = status {
            if s.current_speed > 0.0 {
                frame_idx = (frame_idx + 1) % FAN_FRAMES.len();
            }
        } else {
            frame_idx = (frame_idx + 1) % FAN_FRAMES.len();
        }

        flush();
        sleep(Duration::from_millis(delay));
    }
}
