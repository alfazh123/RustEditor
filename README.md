# rust-editor

This page is provide documentation of rust-editor package built with Rust compiled to WASM (WebAssembly), and simple implementation of use.

## Key  Feature
This package is provide several image processing algorithm such as:
1. Transfer Color: Apply color scheme from "Image Reference" to "Image Target".
2. Sharpen: Enhance edge definition using function from image dependencies.
3. Color Adjustment: Fine-tune Saturation, Temperature, adn Tint of Image Target.
4. Light Adjustment: Contrast and Exposure Adjustments.

## Build WASM package

Before building the package, you must first install `wasm-pack`.

```bash
cargo install wasm-pack
```

Then you can clone this repository and build package with running command bellow.

```bash
wasm-pack build --target web
```

```
.
└── RustEditor/
    ├── pkg/      <--------------------build result
    │   ├── package.json
    │   ├── README.md
    │   ├── rust_editor_bg.wasm
    │   ├── rust_editor_bg.wasm.d.ts
    │   └── rust_editor.d.ts
    ├── site/
    │   ├── index.html
    │   ├── main.js
    │   ├── style.css
    │   ├── assets/
    │   ├── README.md
    │   └── package.json
    ├── src/
    │   ├── color.rs
    │   ├── image_size.rs
    │   ├── lab_converter.rs
    │   ├── lib.rs
    │   ├── light.rs
    │   └── switch_color.rs
    ├── .gitignore
    ├── Cargo.lock
    ├── Cargo.toml
    └── LICENSE
```

Then look in your project directory you must find new directory with folder name `pkg`, on that folder you'll find some file like js, ts, and wasm file. file `.wasm` is result of compile Rust code using `wasm-pack`, file `.js` and `.ts` is bridge for frontEnd can comunicate with `.wasm` file.

you can look published this documentation on this [link](https://www.npmjs.com/package/rust-editor)

## Demo
This documentation is also provide demo, so you can try how to use and how the application work. You can jump to `/site` directory foolow command bellow.

```bash
cd side
npm install // install all depencencies
npx parcel index.html // run project using parceljs
```
