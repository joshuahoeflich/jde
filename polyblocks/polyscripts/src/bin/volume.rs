extern crate pulsectl;
extern crate tokio;

use polylib::{render_pbwrite_error, write_polyblocks};
use polyscripts::get_app_context;
use pulsectl::controllers::types::DeviceInfo;
use pulsectl::controllers::DeviceControl;
use pulsectl::controllers::SinkController;
use std::process;

#[derive(Debug)]
enum VolumeCommand {
    Increase,
    Decrease,
    Mute,
    None,
}

#[derive(Debug)]
enum PulseError {
    MainDevice,
    Mute,
    NoPercentage,
}

#[derive(Debug)]
struct Volume {
    level: String,
    muted: bool,
}

impl Volume {
    fn print(&self) -> String {
        if self.muted {
            format!(" {}", self.level)
        } else {
            format!(" {}", self.level)
        }
    }
}

fn render_pulse_error(err: PulseError) {
    match err {
        PulseError::MainDevice => eprintln!("Could not find main device"),
        PulseError::Mute => eprintln!("Could not handle mute"),
        PulseError::NoPercentage => eprintln!("Could not get volume percentage"),
    }
}

fn get_percentage(volume_string: String) -> Result<String, PulseError> {
    for line in volume_string.split_whitespace() {
        if line.contains('%') {
            return Ok(line.to_owned());
        }
    }
    Err(PulseError::NoPercentage)
}

fn get_main_device(handler: &mut SinkController) -> Result<DeviceInfo, PulseError> {
    handler
        .get_default_device()
        .map_err(|_| PulseError::MainDevice)
}

fn toggle_mute(handler: &mut SinkController, main_device: DeviceInfo) -> Result<(), PulseError> {
    let operation = handler.handler.introspect.set_sink_mute_by_index(
        main_device.index,
        !main_device.mute,
        Some(Box::new(|_| {})),
    );
    handler
        .handler
        .wait_for_operation(operation)
        .map_err(|_| PulseError::Mute)?;
    Ok(())
}

fn get_volume(volume_command: VolumeCommand) -> Result<Volume, PulseError> {
    let mut handler = SinkController::create();
    let main_device = get_main_device(&mut handler)?;
    match volume_command {
        VolumeCommand::Increase => {
            handler.increase_device_volume_by_percent(main_device.index, 0.1)
        }
        VolumeCommand::Decrease => {
            handler.decrease_device_volume_by_percent(main_device.index, 0.1)
        }
        VolumeCommand::Mute => toggle_mute(&mut handler, main_device)?,
        _ => (),
    };
    let main_device = get_main_device(&mut handler)?;
    let vol = Volume {
        level: get_percentage(main_device.volume.print())?,
        muted: main_device.mute,
    };
    Ok(vol)
}

fn get_vol_command(option: Option<String>) -> VolumeCommand {
    match option.as_ref().map(String::as_ref) {
        Some(command_type) => match command_type {
            "increase" => VolumeCommand::Increase,
            "decrease" => VolumeCommand::Decrease,
            "mute" => VolumeCommand::Mute,
            _ => VolumeCommand::None,
        },
        None => VolumeCommand::None,
    }
}

#[tokio::main]
async fn main() {
    let context = get_app_context(
        "Volume",
        "Set the volume of my computer and update the status bar.",
    );
    let vol = get_volume(get_vol_command(context.command)).unwrap_or_else(|err| {
        render_pulse_error(err);
        process::exit(1);
    });
    let output = vol.print();
    match write_polyblocks(&context.socket_addr, &context.block, &output).await {
        Ok(()) => process::exit(0),
        Err(err) => {
            render_pbwrite_error(err);
            process::exit(1);
        }
    }
}
