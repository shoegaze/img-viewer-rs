mod app;
mod gfx;
mod settings;
mod ui;
mod util;
mod window;

use clap::Parser;
use winit::event_loop::{ControlFlow, EventLoop};

use std::error::Error;

use shared::cli::args::Args;

use crate::app::App;
use crate::settings::AppSettings;
use crate::util::path::open_image_from_path;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse().validated()?;

    let settings = AppSettings::default();
    let mut app = App::from_settings(settings);

    for path in args.paths {
        let Err(error) = open_image_from_path(&mut app, path) else {
            continue;
        };

        eprintln!("Couldn't open image: '{}'", error);
    }

    // Execute app
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop
        .run_app(&mut app)
        .expect("EventLoop error occurred");

    Ok(())
}
