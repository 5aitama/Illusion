<p align="center">
<img width=400 src="./res/Illusion-logo-white.png#gh-dark-mode-only">
<img width=400 src="./res/Illusion-logo-black.png#gh-light-mode-only">
</p>

<br />
<p align="center"><i>Cross platform graphcs engine</i></p>

<br />


## Table of contents

- [Description](#description)
- [Requirements](#requirements)
  - [For desktop](#for-desktop)
  - [For browser](#for-browser)
- [How to build](#how-to-build)
  - [For desktop](#for-desktop-1)
  - [For browser](#for-browser-1)
- [Compatibilities](#compatibilities)
  - [Platforms](#platforms)

# Description
**Illusion** is a graphics engine written in [Rust 🦀](https://www.rust-lang.org/) and built on top of [webgpu](https://github.com/gfx-rs/wgpu). It was designed to be easy to use as much a possible with high performance and cross-platform *(see [supported platform](#platforms) section for more infos)*.

# Requirements

## For desktop
- The latest version of [Rust 🦀](https://www.rust-lang.org/) installed on your computer.

## For browser
- The latest version of [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed on your computer.

# How to build

## For desktop
1. Clone this repo
    ```bash
    ▲ ~ git clone https://github.com/5aitama/Illusion.git
    ```

2. Build binaries for your machine
    ```bash
    ▲ ~ cargo build --release
    ```
    > The binaries are located in the folder at ``target/release/``)

3. Run it manually or with 
   ```bash
   ▲ ~ cargo run --release
   ```

## For browser
1. Clone this repo
    ```bash
    ▲ ~ git clone https://github.com/5aitama/Illusion.git
    ```

2. Compile to webassembly with [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
    ```bash
    ▲ ~ wasm-pack build -d web/lib --target web
    ```

3. Open the `index.html` that was located in `web/` folder and enjoy !
    > You need to serve the `web/` folder with a server otherwise you will have some error about **CORS**

# Compatibilities

## Platforms

This table show you the support status of **Illusion** on each operating system.

|               | Supported | Tested  |
| ------------- | --------- | ------- |
| **Windows**   |   ✅      |  ✅     |
| **macOS**     |   ✅      |  ✅     |
| **Linux**     |   ✅      |  ❌     |
| **iOS**       |   🛠️      |  ❌     |
| **Android**   |   🛠️      |  ❌     |
| **Web**       |   ✅      |  ✅     |