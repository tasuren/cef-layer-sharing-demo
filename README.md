# CEF Layer Sharing Demo

This is a demo for sharing web layer between processes on macOS based on
[ChatGPT Atlas's OWL Architecture](https://openai.com/index/building-chatgpt-atlas/)
with CEF.

The main application uses winit to create the window.
Then the main application opens the CEF application as a child process
and display its content via private API `CALayerHost`.

Currently, it can't receive user interactions on the main application.

## Demo usage

```shell
# Compile and bundle the CEF application.
$ cargo run -p demoapp-bundler

# Run the main application.
$ cargo run -p demoapp-winit # with winit crate
$ cargo run -p demoapp-wry   # with GPUI crate
$ cargo run -p demoapp-gpui  # with tao and wry crate
```

## Crates

- `demoapp-winit`: **Main process** (winit window version)
- `demoapp-wry`: **Main process** (tao window version with wry)
- `demoapp-gpui`: **Main process** (gpui window version)
- `demoapp-cef`: **Child process** (CEF application)
- `demoapp-ipc`: Common IPC wrapper
- `demoapp-macos-bindings`: Provides macOS private API `CALayerHost`
- `demoapp-bundler`: Bundler to create the CEF application bundle
- `demoapp-helper`: CEF application helper for macOS CEF application bundle

## Acknowledgments

- ChatGPT Atlas architecture: https://openai.com/index/building-chatgpt-atlas/
- Explanation of `CALayerHost`: https://teamdev.com/jxbrowser/blog/cross-process-rendering-using-calayer/

## License

This project is licensed under the Unlicense.
