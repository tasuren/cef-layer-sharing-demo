use demoapp_macos_bindings::{CALayerHost, ContextId};
use gpui::Window;
use objc2::{AnyThread, MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSView, NSWindowOrderingMode};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

pub fn get_view(window: &Window) -> Option<Retained<NSView>> {
    let window_handle = HasWindowHandle::window_handle(window).unwrap();
    if let RawWindowHandle::AppKit(handle) = window_handle.as_raw() {
        let ptr = unsafe { handle.ns_view.cast().as_mut() };
        return Some(unsafe { Retained::retain(ptr) }.unwrap());
    }

    None
}

pub fn inject_ca_layer_host(context_id: ContextId, window: &Window) {
    let gpui_view = get_view(window).unwrap();
    let content_view = unsafe { gpui_view.superview() }.unwrap(); // It should be `contentView` of `NSWindow`.

    // Create new view for Chromium layer. We want to show GPUI content above.
    let mtm = MainThreadMarker::new().unwrap();
    let chromium_view = NSView::new(mtm);

    chromium_view.setFrame(gpui_view.frame());
    chromium_view.setWantsLayer(true);

    // Set up chromium layer.
    let chromium_layer = CALayerHost::init(CALayerHost::alloc());
    unsafe { chromium_layer.setContextId(context_id) };

    chromium_layer.setFrame(gpui_view.frame());
    chromium_layer.setGeometryFlipped(true);

    chromium_view.layer().unwrap().addSublayer(&chromium_layer);

    // Place chromium view over GPUI view.
    content_view.addSubview_positioned_relativeTo(
        &chromium_view,
        NSWindowOrderingMode::Below,
        Some(&gpui_view),
    );
}
