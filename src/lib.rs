use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        menu::{MenuBar, MenuId, MenuItem, MenuItemAttributes, MenuType},
        platform::run_return::EventLoopExtRunReturn,
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};

use std::process::Command;
use std::thread;
use std::time;

use std::error::Error;

type PlutoResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> PlutoResult<()> {
    // TODO: check if Pluto is installed first before attempting to run it...
    let julia_handler = thread::spawn(|| {
        Command::new("julia")
            .arg("-E")
            .arg("using Pluto; Pluto.run(; launch_browser=false, require_secret_for_access=false )")
            .status()
            .expect("could not start Pluto server... Make sure Pluto is properly installed..");
    });

    // give the Pluto process time to start, so that the UI doesn't give an error when starting, thus necessitating a right-click+page-reload
    let _start_pluto = thread::sleep(time::Duration::from_secs(5));

    let base_url = String::from("http://127.0.0.1:1234/");

    let mut main_menu = MenuBar::new();

    let mut file_menu = MenuBar::new();

    // Bug!?! Cannot create new menu item - gives error:
    // """
    // thread 'main' panicked at 'GTK has not been initialized. Call `gtk::init` first.', ~/.cargo/registry/src/github.com-1ecc6299db9ec823/gtk-0.16.2/src/auto/menu_item.rs:56:9
    // """
    // let itm = file_menu.add_item(MenuItemAttributes::new("Exit"));
    // dbg!(itm);

    main_menu.add_submenu("File", true, file_menu);

    // Start the UI
    let mut event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Pluto Notebooks")
        .with_closable(true)
        // .with_window_icon(window_icon)
        // Start with window maximized
        .with_maximized(true)
        .with_menu(main_menu)
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_url(&base_url)?
        .with_devtools(true)
        .build()?;

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
