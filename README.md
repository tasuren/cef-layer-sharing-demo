# CEF with OWL Architecture Demo

It is the demo to try [ChatGPT Atlas's OWL Architecture](https://openai.com/en-US/index/building-chatgpt-atlas/)
with CEF on Rust.

The main application uses winit to create the window.
Then the main application opens the CEF application and display its content via `CALayerHost`.

Currently, it can't receive user interactions on the main application.

## Demo usage

```shell
# Compile and bundle the CEF application.
$ cargo run -p cefowldemo-bundler

# Run the main application.
$ cargo run -p cefowldemo
```

## Crates

- `cefowldemo`: Main application
- `cefowldemo-ipc`: Common IPC wrapper
- `cefowldemo-macos-bindings`: Exports macOS private API
- `cefowldemo-owl`: CEF application
- `cefowldemo-helper`: CEF application helper for macOS CEF bundle
- `cefowldemo-bundler`: Bundler to create the CEF application bundle.

## Acknowledge

- ChatGPT Atlas architecture: https://openai.com/en-US/index/building-chatgpt-atlas/
- Explanation of `CALayerHost`: https://teamdev.com/jxbrowser/blog/cross-process-rendering-using-calayer/

## License

This project is licensed under the Unlicense.
