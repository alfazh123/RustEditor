import init, { adjust_contrasts_image, adjust_exposure_image, adjust_saturation_image, adjust_temperature_image, adjust_tint_image, sharpen, switch_color } from './pkg/rust_editor';

init().then(() => {
    console.log('initiation wasm success');
})

// initial page

const inputTarget = document.getElementById("input-target");
const imgTarget = document.getElementById("target-element");
const inputRef = document.getElementById("input-ref");
const imgRef = document.getElementById("ref-element");
const imgRes = document.getElementById("res-element");

const transferBtn = document.getElementById("transfer-button");
const sharpRange = document.getElementById("sharp-adj");
const saturationRange = document.getElementById("saturation-adj");
const temperatureRange = document.getElementById("temperature-adj");
const tintRange = document.getElementById("tint-adj");
const contrastRange = document.getElementById("contrast-adj");
const exposureRange = document.getElementById("exposure-adj");

let currentImage = new Uint8Array();
let referenceImage = null;

inputTarget.addEventListener("change", async function() {
    if (inputTarget.files.length > 0) {
        let reader = new FileReader();
        reader.onload = function (e) {
            imgTarget.setAttribute('src', e.target.result);
        }
        reader.readAsDataURL(inputTarget.files[0]);
        currentImage = new Uint8Array(await inputTarget.files[0].arrayBuffer());
    }
})

inputRef.addEventListener("change", async function() {
    if (inputRef.files.length > 0) {
        let reader = new FileReader();
        reader.onload = function (e) {
            imgRef.setAttribute('src', e.target.result);
        }
        reader.readAsDataURL(inputRef.files[0]);
        referenceImage = new Uint8Array(await inputRef.files[0].arrayBuffer())
    }
})

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

saturationRange.addEventListener("input", async function(e) {
    executeFunc("saturation", e.target.value);
})

temperatureRange.addEventListener("input", async function(e) {
    executeFunc("temperature", e.target.value);
})

tintRange.addEventListener("input", async function(e) {
    executeFunc("tint", e.target.value);
})

exposureRange.addEventListener("input", async function(e) {
    executeFunc("tint", e.target.value);
})

contrastRange.addEventListener("input", async function(e) {
    executeFunc("contrast", e.target.value);
})

const showImage = (imageArr) => {
    const blob = new Blob([imageArr], { type: "image/png"});
    const url = URL.createObjectURL(blob);
    imgRes.setAttribute('src', url);
}

const executeFunc = (type, value) => {
    if (currentImage === null || currentImage.length <= 0) {
        alert("input image first");
    }

    let result = new Uint8Array();

    switch (type) {
        case "sharp":
            result = sharpen(currentImage, value);
            showImage(result);
            break;
        case "saturation":
            result = adjust_saturation_image(currentImage, value);
            showImage(result);
            break;
        case "temperature":
            result = adjust_temperature_image(currentImage, value);
            showImage(result);
            break;
        case "tint":
            result = adjust_tint_image(currentImage, value);
            showImage(result);
            break;
        case "exposure":
            result = adjust_exposure_image(currentImage, value);
            showImage(result);
            break;
        case "contrast":
            result = adjust_contrasts_image(currentImage, value);
            showImage(result);
            break;
        default:
            showImage(currentImage);
            break;
    }
}