# img-viewer.rs

No-fluff image viewing application written in Rust

## Build and Execution

To build the project, execute the following cargo command in your shell:

```sh
cargo build --package app-main
```

To build+run the project from cargo, enter the following and replace `{PATH_TO_IMAGE}` with the image you want to view:

```sh
cargo run --package app-main -- {PATH_TO_IMAGE}
```

Note: multiple file paths can be passed, and the app will try to open as many as it can.

## Supported Image Formats

The app uses [`image-rs`](https://github.com/image-rs) to decode images.
For an up-to-date list of supported image formats, please visit their README: [Supported Image Formats](https://github.com/image-rs/image?tab=readme-ov-file#supported-image-formats).

Here are some formats that are tested and confirmed to work:

- PNG
- JPEG/JPG

More will be added in the future, as well as planned video support (e.g. GIF)
