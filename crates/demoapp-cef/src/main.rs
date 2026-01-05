use std::sync::{Arc, Mutex};

use cef::{args::Args, rc::*, *};
use demoapp_ipc::{ChildProcessIpc, InitialInfo, Payload};
use objc2::rc::Retained;
use objc2_app_kit::NSView;

mod ca_layer_exporter;

wrap_app! {
    struct DemoApp {
        ipc: ChildProcessIpc,
        window: Arc<Mutex<Option<Window>>>,
    }

    impl App {
        fn browser_process_handler(&self) -> Option<BrowserProcessHandler> {
            Some(DemoBrowserProcessHandler::new(
                self.ipc.clone(),
                self.window.clone(),
            ))
        }

        fn on_before_command_line_processing(
            &self,
            _process_type: Option<&cef::CefString>,
            command_line: Option<&mut cef::CommandLine>,
        ) {
            if let Some(command_line) = command_line {
                command_line.append_switch(Some(&"use-mock-keychain".into()));
            }
        }
    }
}

wrap_browser_process_handler! {
    struct DemoBrowserProcessHandler {
        ipc: ChildProcessIpc,
        window: Arc<Mutex<Option<Window>>>,
    }

    impl BrowserProcessHandler {
        // The real lifespan of cef starts from `on_context_initialized`, so all the cef objects should be manipulated after that.
        fn on_context_initialized(&self) {
            println!("cef context intiialized");
            let mut client = DemoClient::new(self.window.clone(), self.ipc.clone());
            let url = CefString::from("https://www.google.com");

            let browser_view = browser_view_create(
                Some(&mut client),
                Some(&url),
                Some(&Default::default()),
                Option::<&mut DictionaryValue>::None,
                Option::<&mut RequestContext>::None,
                Option::<&mut BrowserViewDelegate>::None,
            )
            .expect("Failed to create browser view");

            let mut delegate = DemoWindowDelegate::new(browser_view);
            if let Ok(mut window_lock) = self.window.lock() {
                let window = window_create_top_level(Some(&mut delegate)).expect("Failed to create window");
                *window_lock = Some(window);
            }
        }
    }
}

wrap_load_handler! {
    struct DemoLoadHandler {
        window: Arc<Mutex<Option<Window>>>,
        ipc: ChildProcessIpc
    }

    impl LoadHandler {
        fn on_load_end(
            &self,
            browser: Option<&mut Browser>,
            _frame: Option<&mut Frame>,
            _http_status_code: std::ffi::c_int
        ) {
            let _browser_host = browser.unwrap().host().unwrap();
            let window = self.window.lock().unwrap();
            let Some(window) = window.as_ref() else {
                eprintln!("Window is not set.");
                return;
            };

            // Send CAContext ID to parent process.
            let ns_view = get_ns_view(window.window_handle());
            let context_id = ca_layer_exporter::get_ca_context_id(&ns_view)
                    .expect("There is not `CALayerHost` on the Chromium window");
            let bounds = window.bounds();

            let payload = Payload::Initialize(InitialInfo {
                context_id,
                window_width: bounds.width,
                window_height: bounds.height
            });
            self.ipc.send(payload).expect("Failed to send CAContext ID");
        }
    }
}

wrap_client! {
    struct DemoClient {
        window: Arc<Mutex<Option<Window>>>,
        ipc: ChildProcessIpc

    }

    impl Client {
        fn load_handler(&self) -> Option<LoadHandler> {
            Some(DemoLoadHandler::new(self.window.clone(), self.ipc.clone()))
        }
    }
}

wrap_window_delegate! {
    struct DemoWindowDelegate {
        browser_view: BrowserView,
    }

    impl ViewDelegate {}

    impl PanelDelegate {}

    impl WindowDelegate {
        fn on_window_created(&self, window: Option<&mut Window>) {
            if let Some(window) = window {
                let view = self.browser_view.clone();
                window.add_child_view(Some(&mut (&view).into()));
                window.show();

                // Hide CEF window to show only the main window.
                // It seems `CAContext` goes dropped and `CALayerHost` get removed
                // from the layer tree when we use `window.hide()`,
                // so we make window transparent as workaround.
                // It may make the web page always working even the main window is hidden,
                // thus we should handle visibility of winit window and CEF window in production.
                let ns_window = get_ns_view(window.window_handle()).window().unwrap();
                ns_window.setAlphaValue(0.);
                ns_window.setIgnoresMouseEvents(true);
            }
        }

        fn on_window_destroyed(&self, _window: Option<&mut Window>) {
            quit_message_loop();
        }

        fn with_standard_window_buttons(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }

        fn can_resize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }

        fn can_maximize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }

        fn can_minimize(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }

        fn can_close(&self, _window: Option<&mut Window>) -> ::std::os::raw::c_int {
            1
        }
    }
}

// FIXME: Rewrite this demo based on cef/tests/cefsimple
fn main() {
    #[cfg(target_os = "macos")]
    let _loader = {
        let loader = library_loader::LibraryLoader::new(&std::env::current_exe().unwrap(), false);
        assert!(loader.load());
        loader
    };

    #[cfg(target_os = "macos")]
    {
        use objc2::{
            ClassType, MainThreadMarker, msg_send,
            rc::Retained,
            runtime::{AnyObject, NSObjectProtocol},
        };
        use objc2_app_kit::NSApp;

        use application::SimpleApplication;

        let mtm = MainThreadMarker::new().unwrap();

        unsafe {
            // Initialize the SimpleApplication instance.
            // SAFETY: mtm ensures that here is the main thread.
            let _: Retained<AnyObject> = msg_send![SimpleApplication::class(), sharedApplication];
        }

        // If there was an invocation to NSApp prior to here,
        // then the NSApp will not be a SimpleApplication.
        // The following assertion ensures that this doesn't happen.
        assert!(NSApp(mtm).isKindOfClass(SimpleApplication::class()));
    }

    let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);

    let args = Args::new();
    let cmd = args.as_cmd_line().unwrap();

    let switch = CefString::from("type");
    let is_browser_process = cmd.has_switch(Some(&switch)) != 1;

    let window = Arc::new(Mutex::new(None));

    // Connect to the IPC.
    let ipc_name = std::env::args().next_back().expect("IPC name is not set");
    let ipc = ChildProcessIpc::connect(ipc_name).expect("Failed to connect to the IPC");

    let mut app = DemoApp::new(ipc, window.clone());
    let ret = execute_process(
        Some(args.as_main_args()),
        Some(&mut app),
        std::ptr::null_mut(),
    );

    if is_browser_process {
        println!("launch browser process");
        assert!(ret == -1, "cannot execute browser process");
    } else {
        let process_type = CefString::from(&cmd.switch_value(Some(&switch)));
        println!("launch process {process_type}");
        assert!(ret >= 0, "cannot execute non-browser process");
        // non-browser process does not initialize cef
        return;
    }
    let settings = Settings::default();
    assert_eq!(
        initialize(
            Some(args.as_main_args()),
            Some(&settings),
            Some(&mut app),
            std::ptr::null_mut(),
        ),
        1
    );

    run_message_loop();

    let window = window.lock().expect("Failed to lock window");
    let window = window.as_ref().expect("Window is None");
    assert!(window.has_one_ref());

    shutdown();
}

pub(crate) fn get_ns_view(window_handle: *mut std::ffi::c_void) -> Retained<NSView> {
    let ptr = window_handle as *mut NSView;
    unsafe { Retained::retain(ptr).unwrap() }
}

mod application {
    use std::cell::Cell;

    use cef::application_mac::{CefAppProtocol, CrAppControlProtocol, CrAppProtocol};
    use objc2::{DefinedClass, define_class, runtime::Bool};
    use objc2_app_kit::NSApplication;

    /// Instance variables of `SimpleApplication`.
    pub struct SimpleApplicationIvars {
        handling_send_event: Cell<Bool>,
    }

    define_class!(
        /// A `NSApplication` subclass that implements the required CEF protocols.
        ///
        /// This class provides the necessary `CefAppProtocol` conformance to
        /// ensure that events are handled correctly by the Chromium framework on macOS.
        #[unsafe(super(NSApplication))]
        #[ivars = SimpleApplicationIvars]
        pub struct SimpleApplication;

        unsafe impl CrAppControlProtocol for SimpleApplication {
            #[unsafe(method(setHandlingSendEvent:))]
            unsafe fn set_handling_send_event(&self, handling_send_event: Bool) {
                self.ivars().handling_send_event.set(handling_send_event);
            }
        }

        unsafe impl CrAppProtocol for SimpleApplication {
            #[unsafe(method(isHandlingSendEvent))]
            unsafe fn is_handling_send_event(&self) -> Bool {
                self.ivars().handling_send_event.get()
            }
        }

        unsafe impl CefAppProtocol for SimpleApplication {}
    );
}
