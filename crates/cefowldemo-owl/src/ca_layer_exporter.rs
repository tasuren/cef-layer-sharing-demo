use std::ffi::c_void;

use cefowldemo_macos_bindings::CALayerHost;
use objc2::rc::Retained;
use objc2_app_kit::NSView;
use objc2_quartz_core::CALayer;

fn find_ca_layer_host(ca_layer: &CALayer) -> Option<Retained<CALayerHost>> {
    let sublayers = unsafe { ca_layer.sublayers() }?;

    for sublayer in sublayers.iter() {
        let sublayer = match sublayer.downcast() {
            Ok(ca_layer_host) => return Some(ca_layer_host),
            Err(sublayer) => sublayer,
        };

        let result = find_ca_layer_host(sublayer.as_ref());
        if result.is_some() {
            return result;
        }
    }

    None
}

pub fn export_ca_layer(window_handle: *mut c_void) {
    let ptr = window_handle as *mut NSView;
    let ns_view = unsafe { Retained::retain(ptr).unwrap() };

    let ca_layer = ns_view
        .layer()
        .expect("There is no `CALayer` on the browser window.");

    if let Some(ca_layer_host) = find_ca_layer_host(ca_layer.as_ref()) {
        let context_id = ca_layer_host.contextId();
        println!("contextId: {context_id}");
    } else {
        eprintln!("`CALayerHost` is not found.");
    };
}
