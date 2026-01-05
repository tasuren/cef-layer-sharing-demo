use demoapp_macos_bindings::{CALayerHost, ContextId};
use objc2::{AnyThread, rc::Retained};
use objc2_app_kit::NSView;
use objc2_core_foundation::CGPoint;
use objc2_quartz_core::CALayer;
use winit::{
    raw_window_handle::{HasWindowHandle, RawWindowHandle},
    window::Window,
};

pub fn get_view(window: &Window) -> Option<Retained<NSView>> {
    let window_handle = window.window_handle().unwrap();
    if let RawWindowHandle::AppKit(handle) = window_handle.as_raw() {
        let ptr = unsafe { handle.ns_view.cast().as_mut() };
        return Some(unsafe { Retained::retain(ptr) }.unwrap());
    }

    None
}

pub fn inject_ca_layer_host(context_id: ContextId, window: &Window) {
    let view = get_view(window).unwrap();
    if !view.wantsLayer() {
        view.setWantsLayer(true);
    }

    let ca_layer_host = CALayerHost::init(CALayerHost::alloc());
    unsafe { ca_layer_host.setContextId(context_id) };

    view.layer()
        .unwrap()
        .addSublayer(ca_layer_host.downcast_ref().unwrap());

    let ca_layer: &CALayer = ca_layer_host.downcast_ref().unwrap();
    ca_layer.setPosition(CGPoint::new(0., 0.));
}
