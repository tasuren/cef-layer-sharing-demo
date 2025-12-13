use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        #[cfg(target_os = "macos")]
        {
            let context_id: u32 = std::env::var("CONTEXT_ID")
                .expect("Please select `CONTEXT_ID`.")
                .parse()
                .expect("`CONTEXT_ID` is not valid.");
            ca_layer_setup::set_ca_layer_host(context_id, &window);
        }

        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
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

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

#[cfg(target_os = "macos")]
mod ca_layer_setup {
    use cefowldemo_macos_bindings::{CALayerHost, ContextId};
    use objc2::{AnyThread, rc::Retained};
    use objc2_app_kit::NSView;
    use objc2_core_foundation::CGPoint;
    use objc2_quartz_core::CALayer;
    use winit::{
        raw_window_handle::{HasWindowHandle, RawWindowHandle},
        window::Window,
    };

    pub fn get_view(window: &Window) -> Retained<NSView> {
        let window_handle = window.window_handle().unwrap();
        if let RawWindowHandle::AppKit(handle) = window_handle.as_raw() {
            let ptr = unsafe { handle.ns_view.cast().as_mut() };
            return unsafe { Retained::retain(ptr) }.unwrap();
        }

        unreachable!();
    }

    pub fn set_ca_layer_host(context_id: ContextId, window: &Window) {
        let view = get_view(window);
        if !view.wantsLayer() {
            view.setWantsLayer(true);
        }

        let ca_layer_host = CALayerHost::init(CALayerHost::alloc());
        unsafe { ca_layer_host.setContextId(context_id) };

        view.layer()
            .unwrap()
            .addSublayer(ca_layer_host.downcast_ref().unwrap());

        let ca_layer: &CALayer = ca_layer_host.downcast_ref().unwrap();
        ca_layer.setPosition(CGPoint::new(500., 500.));

        std::mem::forget(ca_layer_host);
    }
}
