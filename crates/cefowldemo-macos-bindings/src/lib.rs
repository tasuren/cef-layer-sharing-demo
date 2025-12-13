#![allow(non_snake_case)]

use objc2::{
    extern_class, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_quartz_core::CALayer;

pub type ContextId = std::ffi::c_uint;

extern_class!(
    #[unsafe(super(CALayer))]
    pub struct CALayerHost;
);

impl CALayerHost {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

impl CALayerHost {
    extern_methods!(
        #[unsafe(method(contextId))]
        pub fn contextId(&self) -> ContextId;

        #[unsafe(method(setContextId:))]
        pub unsafe fn setContextId(&self, contextId: ContextId);
    );
}
