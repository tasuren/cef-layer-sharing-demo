# CEF Layer Sharing Demo

This is a demo for sharing web layer between processes on macOS based on
[ChatGPT Atlas's OWL Architecture](https://openai.com/en-US/index/building-chatgpt-atlas/)
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
$ cargo run -p demoapp
```

## Crates

- `demoapp`: Main application
- `demoapp-bundler`: Bundler to create the CEF application bundle.
- `demoapp-cef`: CEF application
- `demoapp-helper`: CEF application helper for macOS CEF bundle
- `demoapp-ipc`: Common IPC wrapper
- `demoapp-macos-bindings`: Exports macOS private API

## Acknowledge

- ChatGPT Atlas architecture: https://openai.com/index/building-chatgpt-atlas/
- Explanation of `CALayerHost`: https://teamdev.com/jxbrowser/blog/cross-process-rendering-using-calayer/

## License

This project is licensed under the Unlicense.
