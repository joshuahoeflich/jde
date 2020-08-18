extern crate tokio;

use polylib::{render_pbwrite_error, write_polyblocks};
use polyscripts::{get_app_context, remove_whitespace};
use std::path::PathBuf;
use std::process;
use tokio::fs::read_to_string;

const CUR_BRIGHTNESS_PATH: &'static str = "/sys/class/backlight/intel_backlight/brightness";
const MAX_BRIGHTNESS_PATH: &'static str = "/sys/class/backlight/intel_backlight/max_brightness";

#[derive(Debug)]
enum BrightnessFailure {
    BadFile,
    BadNumber,
    BadWrite,
}

enum BrightnessCommand {
    Increase,
    Decrease,
    None,
}

struct SystemBrightness {
    cur_brightness: f32,
    max_brightness: f32,
}

impl ToString for SystemBrightness {
    #[inline]
    fn to_string(&self) -> String {
        format!(
            "  {:.0}%",
            (self.cur_brightness / self.max_brightness) * 100.0
        )
    }
}

fn render_brightness_error(err: BrightnessFailure) {
    match err {
        BrightnessFailure::BadFile => eprintln!("Could not find required file"),
        BrightnessFailure::BadNumber => eprintln!("Could not get float out of file"),
        BrightnessFailure::BadWrite => eprintln!("Could not update brightness"),
    }
}

fn float_from_filestring(
    maybe_file: Result<String, std::io::Error>,
) -> Result<f32, BrightnessFailure> {
    let float = maybe_file.map_err(|_| BrightnessFailure::BadFile)?;
    remove_whitespace(float)
        .parse::<f32>()
        .map_err(|_| BrightnessFailure::BadNumber)
}

async fn get_system_brightness() -> Result<SystemBrightness, BrightnessFailure> {
    let (brightness, max_brightness) = tokio::join!(
        read_to_string(PathBuf::from(CUR_BRIGHTNESS_PATH)),
        read_to_string(PathBuf::from(MAX_BRIGHTNESS_PATH))
    );
    let cur_brightness = float_from_filestring(brightness)?;
    let max_brightness = float_from_filestring(max_brightness)?;
    Ok(SystemBrightness {
        cur_brightness,
        max_brightness,
    })
}

fn get_brightness_command(maybe_command: Option<String>) -> BrightnessCommand {
    match maybe_command.as_ref().map(String::as_ref) {
        Some(command_type) => match command_type {
            "increase" => BrightnessCommand::Increase,
            "decrease" => BrightnessCommand::Decrease,
            _ => BrightnessCommand::None,
        },
        None => BrightnessCommand::None,
    }
}

async fn update_brightness(new_brightness: f32) -> Result<(), BrightnessFailure> {
    let brightness_string = format!("{}\n", new_brightness.floor());
    tokio::fs::write(CUR_BRIGHTNESS_PATH, &brightness_string)
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            BrightnessFailure::BadWrite
        })?;
    Ok(())
}

fn min(a: f32, b: f32) -> f32 {
    if a < b {
        return a;
    }
    b
}

async fn change_brightness(
    brightness_command: BrightnessCommand,
) -> Result<SystemBrightness, BrightnessFailure> {
    let mut brightness = get_system_brightness().await?;
    let bright_ref = &mut brightness;
    match brightness_command {
        BrightnessCommand::Increase => {
            if bright_ref.cur_brightness >= bright_ref.max_brightness {
                ()
            } else {
                bright_ref.cur_brightness =
                    min(bright_ref.cur_brightness * 1.10, bright_ref.max_brightness);
                update_brightness(bright_ref.cur_brightness).await?;
            }
        }
        BrightnessCommand::Decrease => {
            if bright_ref.cur_brightness <= 0.0 {
                ()
            } else {
                bright_ref.cur_brightness = bright_ref.cur_brightness * 0.9;
                update_brightness(bright_ref.cur_brightness).await?;
            }
        }
        _ => (),
    };
    Ok(brightness)
}

async fn get_brightness(
    brightness_command: BrightnessCommand,
) -> Result<String, BrightnessFailure> {
    let system_brightness = change_brightness(brightness_command).await?;
    Ok(system_brightness.to_string())
}

#[tokio::main]
async fn main() {
    let context = get_app_context(
        "brightness",
        "Display brightness information about my screen on my status bar.",
    );
    let output = get_brightness(get_brightness_command(context.command))
        .await
        .unwrap_or_else(|err| {
            render_brightness_error(err);
            process::exit(1);
        });
    match write_polyblocks(&context.socket_addr, &context.block, &output).await {
        Ok(()) => process::exit(0),
        Err(e) => {
            render_pbwrite_error(e);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{float_from_filestring, BrightnessFailure};
    #[test]
    fn test_io_err() {
        match float_from_filestring(Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "bad",
        ))) {
            Err(BrightnessFailure::BadFile) => (),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_parse_error() {
        match float_from_filestring(Ok("no\nnumber\nhere".to_string())) {
            Err(BrightnessFailure::BadNumber) => (),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_good_file() {
        assert_eq!(32.0, float_from_filestring(Ok("32\n".to_string())).unwrap())
    }
}
