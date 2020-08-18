extern crate tokio;

use polylib::{render_pbwrite_error, write_polyblocks};
use polyscripts::{get_app_context, remove_whitespace};
use std::path::PathBuf;
use std::process;
use tokio::fs::read_to_string;

#[derive(Debug)]
enum BrightnessFailure {
    BadFile,
    BadNumber,
}

fn render_brightness_error(err: BrightnessFailure) {
    match err {
        BrightnessFailure::BadFile => eprintln!("Could not find required file"),
        BrightnessFailure::BadNumber => eprintln!("Could not get uint out of file"),
    }
}

fn uint_from_filestring(
    maybe_brightness: Result<String, std::io::Error>,
) -> Result<u32, BrightnessFailure> {
    let brightness = maybe_brightness.map_err(|_| BrightnessFailure::BadFile)?;
    remove_whitespace(brightness)
        .parse::<u32>()
        .map_err(|_| BrightnessFailure::BadNumber)
}

#[test]
fn test_io_err() {
    match uint_from_filestring(Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "bad",
    ))) {
        Err(BrightnessFailure::BadFile) => (),
        _ => unreachable!(),
    }
}

#[test]
fn test_parse_error() {
    match uint_from_filestring(Ok("no\nnumber\nhere".to_string())) {
        Err(BrightnessFailure::BadNumber) => (),
        _ => unreachable!(),
    }
}

#[test]
fn test_good_file() {
    assert_eq!(32, uint_from_filestring(Ok("32\n".to_string())).unwrap())
}

fn get_brightness_string(brightness: u32, max_brightness: u32) -> String {
    format!("  {}%", (brightness / max_brightness) * 100)
}

async fn get_brightness() -> Result<String, BrightnessFailure> {
    let (brightness, max_brightness) = tokio::join!(
        read_to_string(PathBuf::from(
            "/sys/class/backlight/intel_backlight/brightness"
        )),
        read_to_string(PathBuf::from(
            "/sys/class/backlight/intel_backlight/max_brightness"
        ))
    );
    let brightness = uint_from_filestring(brightness)?;
    let max_brightness = uint_from_filestring(max_brightness)?;
    Ok(get_brightness_string(brightness, max_brightness))
}

#[tokio::main]
async fn main() {
    let context = get_app_context(
        "brightness",
        "Display brightness information about my screen on my status bar.",
    );
    let output = get_brightness().await.unwrap_or_else(|err| {
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
