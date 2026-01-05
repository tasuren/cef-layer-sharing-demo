use demoapp_macos_bindings::{CALayerHost, ContextId};
use objc2::{AnyThread, MainThreadMarker, rc::Retained};
use objc2_app_kit::{NSView, NSWindowOrderingMode};
use tao::{platform::macos::WindowExtMacOS, window::Window};

pub fn get_view(window: &Window) -> Option<Retained<NSView>> {
    unsafe { Retained::retain(window.ns_view() as *mut NSView) }
}

pub fn inject_ca_layer_host(context_id: ContextId, window: &Window) {
    let wry_view = get_view(window).unwrap();
    let content_view = unsafe { wry_view.superview() }.unwrap(); // It should be `contentView` of `NSWindow`.

    // Create new view for Chromium layer. We want to show wry content above.
    let mtm = MainThreadMarker::new().unwrap();
    let chromium_view = NSView::new(mtm);

    chromium_view.setFrame(wry_view.frame());
    chromium_view.setWantsLayer(true);

    // Set up chromium layer.
    let chromium_layer = CALayerHost::init(CALayerHost::alloc());
    unsafe { chromium_layer.setContextId(context_id) };

    chromium_layer.setFrame(wry_view.frame());
    chromium_layer.setGeometryFlipped(true);

    chromium_view.layer().unwrap().addSublayer(&chromium_layer);

    // Place chromium view over wry view.
    content_view.addSubview_positioned_relativeTo(
        &chromium_view,
        NSWindowOrderingMode::Below,
        Some(&wry_view),
    );
}
