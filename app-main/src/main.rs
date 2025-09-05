mod app;
mod gfx;
mod settings;
mod ui;
mod util;
mod window;

use clap::Parser;
use winit::error::EventLoopError;
use winit::event_loop::{ControlFlow, EventLoop};

use std::error::Error;
use std::process::exit;

use shared::cli::args::Args;

use crate::app::App;
use crate::settings::AppSettings;
use crate::util::path::open_image_from_path;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse().validated();

    if args.paths.is_empty() {
        eprintln!("No valid images could be opened");
        exit(1);
    }

    // TODO: Load app settings from file
    let settings = AppSettings::default();
    let mut app = App::from_settings(settings);

    for path in args.paths {
        let open_image_result = open_image_from_path(&mut app, path);

        if let Err(error) = open_image_result {
            eprintln!("Couldn't open image: '{}'", error);
        };
    }

    {
        let app_result = run_app(&mut app);

        if let Err(error) = app_result {
            eprintln!("Event loop error occurred: {}", error);

            return Err(Box::from(error));
        }
    }

    Ok(())
}

fn run_app(app: &mut App) -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new()?;

    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(app)
}
