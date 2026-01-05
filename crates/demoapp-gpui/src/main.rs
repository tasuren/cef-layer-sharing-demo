use std::path::PathBuf;

use demoapp_ipc::{InitialInfo, ParentProcessIpc, Payload};
use gpui::{
    App, Application, Bounds, Context, Window, WindowBackgroundAppearance, WindowBounds,
    WindowOptions, div, prelude::*, px, size,
};

use crate::ca_layer_injector::inject_ca_layer_host;

mod ca_layer_injector;

struct HelloWorld {}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .text_color(gpui::white())
            .justify_center()
            .items_center()
            .child(
                div()
                    .bg(gpui::black().alpha(0.7))
                    .p_4()
                    .text_3xl()
                    .child("GPUI over CEF"),
            )
    }
}

fn main() {
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

    let Payload::Initialize(InitialInfo {
        context_id,
        window_width,
        window_height,
    }) = initial_payload
    else {
        panic!("Non-initialize payload is given");
    };

    Application::new().run(move |cx: &mut App| {
        cx.on_app_quit(move |_cx| {
            chromium.kill().unwrap();
            _ = chromium.wait();

            async {}
        })
        .detach();

        let bounds = Bounds::centered(
            None,
            size(px(window_width as _), px(window_height as _)),
            cx,
        );

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_background: WindowBackgroundAppearance::Transparent,
                ..Default::default()
            },
            |window, cx| {
                inject_ca_layer_host(context_id, window);

                cx.new(|_| HelloWorld {})
            },
        )
        .unwrap();
    });
}
