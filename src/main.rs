use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop, EventLoopClosed},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

use std::env;
use std::process::Command;
use std::{thread, time};

fn main() -> wry::Result<()> {
    // println!(
    //     "input the Pluto notebook secret as the first arg to this program, if necessary...\n\n"
    // );

    let julia_handler = thread::spawn(|| {
        Command::new("julia")
            // .env("JULIA_DIR", "~/.juliaup/bin/")
            // .env("LD_LIBRARY_PATH", "~/.julia/juliaup/julia-1.8.5+0.x64.linux.gnu/lib/")
            .env("JULIA_NUM_THREADS", "16")
            .arg("-E")
            .arg("using Pluto; Pluto.run()")
            .status()
            // .output()
            .expect("could not start Pluto server... Make sure Pluto is properly installed..");
    });

    // thread::sleep(time::Duration::from_secs(7));

    let mut base_url = String::from("http://127.0.0.1:1234/");

    let args: Vec<String> = env::args().collect();

    // the first arg is always the prog name
    // if the length of the args vector is
    if &args.len() > &1_usize {
        let secret = &args[1];

        let secret_query = "&secret=";

        base_url.push_str(secret_query);

        base_url.push_str(secret);
        dbg!(&base_url);
    }

    // Start the UI
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Pluto Notebooks")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?.with_url(&base_url)?.build()?;

    // julia_handler.join().unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Welcome to Pluto Notebooks!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Close Julia process!!");
                // DOES NOT WORK!!!
                // julia_handler
                //     .join()
                //     .expect("Couldn't finish Julia / Pluto");
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}
