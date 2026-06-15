# rust-editor

This page is provide documentation of rust-editor package built with Rust compiled to WASM (WebAssembly), and simple implementation of use.

## Key  Feature
This package is provide several image processing algorithm such as:
1. Transfer Color: Apply color scheme from "Image Reference" to "Image Target".
2. Sharpen: Enhance edge definition using function from image dependencies.
3. Color Adjustment: Fine-tune Saturation, Temperature, adn Tint of Image Target.
4. Light Adjustment: Contrast and Exposure Adjustments.

## Usage

First calling function from package.

```ts
import init, { switch_color, sharpen } from 'rust-editor';

function switchColorTheme(imgTarget: Uint8Array, imgReference: Uint8Array): Uint8Array {
    try {
		const result = switch_color(imgT, imgR);
		return result;
	} catch (error) {
		console.error("Error getting image dimensions:", error);
		return new Uint8Array();
	}
}

function sharpImage(img: Uint8Array, radius: number): Uint8Array {
    try {
        const result = sharpen(img, radius);
        return result;
    } catch {
        console.error("Error sharp image:", error);
		return new Uint8Array();
    }
}
```

then use that function to event listener in your component like code bellow.

```js
transferBtn.addEventListener("click", async function() {
    try {
        if (currentImage === null || referenceImage === null) {
            alert("input reference image first");
            return;
        }

        const switchImage = switch_color(currentImage, referenceImage)
        showImage(switchImage)
    } catch (e) {
        console.log("message", e);
    }
})

sharpRange.addEventListener("input", async function(e) {
    executeFunc("sharp", e.target.value);
})
```
## Example 

As example of this package usage you can look at this link [example](https://github.com/alfazh123/RustEditor/tree/main/site/main.js), or this link [image-editor](https://alfazh123.github.io/image-editor/).