extern crate pulsectl;
extern crate tokio;

use clap::{App, Arg};
use polylib::{render_pbwrite_error, write_polyblocks};
use polyscripts::get_socket_addr;
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
        .get_device_by_index(0)
        .map_err(|_| PulseError::MainDevice)
}

fn toggle_mute(handler: &mut SinkController, main_device: DeviceInfo) -> Result<(), PulseError> {
    let operation = handler.handler.introspect.set_sink_mute_by_index(
        main_device.index,
        !main_device.mute,
        Some(Box::new(|_| {
            println!("I RAN");
        })),
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

struct VolumeContext {
    command: VolumeCommand,
    block: String,
    socket_addr: String,
}

fn get_volume_context() -> VolumeContext {
    let matches = App::new("volume")
        .name("volume")
        .version("1.0")
        .author("Joshua Hoeflich")
        .about("Update the volume and my status bar together.")
        .arg(
            Arg::with_name("id")
                .short("i")
                .long("id")
                .takes_value(true)
                .help("ID of the server to which you would like to connect. Defaults to 0."),
        )
        .arg(
            Arg::with_name("block")
                .short("b")
                .long("block")
                .required(true)
                .takes_value(true)
                .help("Block which you would like this command to update"),
        )
        .arg(
            Arg::with_name("command")
                .short("c")
                .long("command")
                .takes_value(true)
                .help("How you would like to modify the volume."),
        )
        .get_matches();
    let socket_addr = get_socket_addr(matches.value_of("id").unwrap_or("0"));
    let block = matches
        .value_of("block")
        .expect("Output is required")
        .to_owned();
    let command = match matches.value_of("command") {
        Some(command_type) => match command_type {
            "increase" => VolumeCommand::Increase,
            "decrease" => VolumeCommand::Decrease,
            "mute" => VolumeCommand::Mute,
            _ => VolumeCommand::None,
        },
        None => VolumeCommand::None,
    };
    VolumeContext {
        socket_addr,
        block,
        command,
    }
}

#[tokio::main]
async fn main() {
    let context = get_volume_context();
    let vol = get_volume(context.command).unwrap_or_else(|err| {
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
