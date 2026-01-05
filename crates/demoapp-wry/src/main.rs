use std::path::PathBuf;

use demoapp_ipc::{InitialInfo, ParentProcessIpc, Payload};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::WindowBuilder,
};
use wry::WebViewBuilder;

use crate::ca_layer_injector::inject_ca_layer_host;

mod ca_layer_injector;

fn run_app(
    InitialInfo {
        context_id,
        window_width,
        window_height,
    }: InitialInfo,
) -> wry::Result<()> {
    // Set up window.
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(window_width, window_height))
        .with_transparent(true)
        .build(&event_loop)
        .unwrap();

    let builder = WebViewBuilder::new()
        .with_html(include_str!("index.html"))
        .with_transparent(true);

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let _webview = builder.build(&window)?;
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let _webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

    inject_ca_layer_host(context_id, &window);

    event_loop.run_return(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            *control_flow = ControlFlow::Exit;
        }
    });

    Ok(())
}

fn main() -> wry::Result<()> {
    // Prepare the IPC.
    let (ipc, ipc_name) = ParentProcessIpc::new().expect("Faield to create the IPC");

    // Open the chromium application.
    let chromium_app_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/debug")
        .join("demoapp-cef.app/Contents/MacOS/demoapp-cef");
    let mut chromium = std::process::Command::new(chromium_app_path)
        .arg(ipc_name)
        .spawn()
        .expect("Failed to spawn the chromium application");
    let (_ipc, initial_payload) = ipc.accept().expect("Failed to accept the IPC connection");

    let Payload::Initialize(info) = initial_payload else {
        panic!("Non-initialize payload is given");
    };

    let result = run_app(info);

    chromium.kill().unwrap();
    _ = chromium.wait();

    result
}
