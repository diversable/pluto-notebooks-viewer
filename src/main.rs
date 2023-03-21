use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        platform::run_return::EventLoopExtRunReturn,
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

use std::process::Command;
use std::thread;
use std::time;

fn main() -> wry::Result<()> {
    let julia_handler = thread::spawn(|| {
        Command::new("julia")
            // .env("JULIA_DIR", "~/.juliaup/bin/")
            // .env("LD_LIBRARY_PATH", "~/.julia/juliaup/julia-1.8.5+0.x64.linux.gnu/lib/")
            // .env("JULIA_NUM_THREADS", "16")
            .arg("-E")
            .arg("using Pluto; Pluto.run(; launch_browser=false, require_secret_for_access=false )")
            .status()
            // .output()
            .expect("could not start Pluto server... Make sure Pluto is properly installed..");
    });

    let _start_pluto = thread::sleep(time::Duration::from_secs(5));

    let mut base_url = String::from("http://127.0.0.1:1234/");

    // Start the UI
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pluto Notebooks")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?.with_url(&base_url)?.build()?;

    // use the `run_return` method instead of simply `run` so that you can re-join the julia handler thread / clean up before the program exits..
    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Welcome to Pluto Notebooks!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });

    julia_handler.join().unwrap();
    Ok(())
}
