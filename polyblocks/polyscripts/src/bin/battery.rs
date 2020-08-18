extern crate tokio;

use polylib::{render_pbwrite_error, write_polyblocks};
use polyscripts::{get_app_context, remove_whitespace};
use std::path::PathBuf;
use std::process;
use tokio::fs::read_to_string;

#[derive(Debug)]
enum BatteryFailure {
    BadCapacity,
    BadStatus,
    ParseError,
}

fn render_battery_error(err: BatteryFailure) {
    match err {
        BatteryFailure::BadCapacity => eprintln!("Could not read capacity to string"),
        BatteryFailure::BadStatus => eprintln!("Could not read status to string"),
        BatteryFailure::ParseError => eprintln!("Could not parse battery capacity"),
    }
}

fn get_battery_string(battery_capacity: i32, battery_status: String) -> String {
    if battery_status != "Discharging" {
        return format!(" {}%", battery_capacity);
    }
    if battery_capacity > 95 {
        return format!("{}%", battery_capacity);
    }
    if battery_capacity > 75 {
        return format!(" {}%", battery_capacity);
    }
    if battery_capacity > 50 {
        return format!(" {}%", battery_capacity);
    }
    if battery_capacity > 25 {
        return format!(" {}%", battery_capacity);
    }
    if battery_capacity > 0 {
        return format!(" {}%", battery_capacity);
    }
    return format!("? {}", battery_capacity);
}

#[test]
fn battery_charged() {
    assert_eq!(
        " 55%".to_string(),
        get_battery_string(55, "Charging".to_string())
    );
}

#[test]
fn battery_depleting() {
    assert_eq!(
        " 55%".to_string(),
        get_battery_string(55, "Discharging".to_string())
    );
}

async fn get_battery_status() -> Result<String, BatteryFailure> {
    let (battery_capacity, battery_status) = tokio::join!(
        read_to_string(PathBuf::from("/sys/class/power_supply/BAT0/capacity")),
        read_to_string(PathBuf::from("/sys/class/power_supply/BAT0/status"))
    );
    let battery_capacity = battery_capacity
        .map_err(|_| BatteryFailure::BadCapacity)
        .and_then(|capacity| {
            remove_whitespace(capacity)
                .parse::<i32>()
                .map_err(|_| BatteryFailure::ParseError)
        })?;
    let battery_status = battery_status.map_err(|_| BatteryFailure::BadStatus)?;
    Ok(get_battery_string(
        battery_capacity,
        remove_whitespace(battery_status),
    ))
}

#[tokio::main]
async fn main() {
    let context = get_app_context(
        "battery",
        "Put useful battery information on my status bar.",
    );
    let output = get_battery_status().await.unwrap_or_else(|err| {
        render_battery_error(err);
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
