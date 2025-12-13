# **WIP** CEF Demo Using CALayerHost

## Demo usage

```shell
# Bundle the cef app (crates/cefowldemo-owl).
$ cargo run -p cefowldemo-bundler

# Boot the chromium (crates/cefowldemo-owl).
$ ./target/debug/cefowldemo.app/Contents/MacOS/cefowldemo
launch browser process
[1213/221140.967886:WARNING:cef/libcef/common/resource_util.cc:83] Please customize CefSettings.root_cache_path for your application. Use of the default value may lead to unintended process singleton behavior.
cef context intiialized
contextId: 2161988284
[73245:2301251:1213/221143.509976:ERROR:google_apis/gcm/engine/registration_request.cc:292] Registration response error message: QUOTA_EXCEEDED

# Note above "contextId: ..." number, then run the main app (crates/cefowldemo).
$ CONTEXT_ID=2161988284 cargo run -p cefowldemo
```

## License

This project is licensed under the Unlicense.
