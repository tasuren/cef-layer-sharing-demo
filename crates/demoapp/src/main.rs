use std::path::PathBuf;

use demoapp_ipc::{InitialInfo, ParentProcessIpc, Payload, Receiver};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod ca_layer_injector;

struct App {
    _ipc: Receiver,
    initial_info: InitialInfo,
    chromium: std::process::Child,
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes().with_inner_size(LogicalSize::new(
            self.initial_info.window_width,
            self.initial_info.window_height,
        ));
        let window = event_loop.create_window(attributes).unwrap();

        // Set `CALayer` of Chromium.
        ca_layer_injector::inject_ca_layer_host(self.initial_info.context_id, &window);

        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");

                self.chromium.kill().unwrap();
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
    // dispatched any events. This is ideal for games and similar applications.
    event_loop.set_control_flow(ControlFlow::Poll);

    // ControlFlow::Wait pauses the event loop if no events are available to process.
    // This is ideal for non-game applications that only update in response to user
    // input, and uses significantly less power/CPU time than ControlFlow::Poll.
    event_loop.set_control_flow(ControlFlow::Wait);

    // Prepare the IPC.
    let (ipc, ipc_name) = ParentProcessIpc::new().expect("Faield to create the IPC");

    // Open the chromium application.
    let chromium_app_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/debug")
        .join("demoapp-cef.app/Contents/MacOS/demoapp-cef");
    let chromium = std::process::Command::new(chromium_app_path)
        .arg(ipc_name)
        .stdout(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn the chromium application");
    let (ipc, initial_payload) = ipc.accept().expect("Failed to accept the IPC connection");

    let Payload::Initialize(initial_info) = initial_payload else {
        panic!("Non-initialize payload is given");
    };

    let mut app = App {
        _ipc: ipc,
        initial_info,
        chromium,
        window: None,
    };

    event_loop.run_app(&mut app).unwrap();
}
