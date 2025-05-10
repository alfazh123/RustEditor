import init, { resize, get_size, resize_one_side, change_scale_image, crop } from './pkg/rust_img_editor.js';
import { switch_color } from './pkg/rust_img_editor.js';

const fileInput = document.getElementById('file-input');
const image = document.getElementById('image');
const fileInputRef = document.getElementById('reference');
const imageRef = document.getElementById('reference-image');

fileInputRef.setAttribute("draggable",false);
image.setAttribute("draggable",false);

const switchBtn = document.getElementById('switch-btn');

const width = document.getElementById('width');
const height = document.getElementById('height');

let imageRect = image.getBoundingClientRect();
let imageX =imageRect.left;
let imageY =imageRect.top;
const imageWrapper = document.getElementById('image-wrapper');
const cropbtn = document.getElementById('btn-crop');
const confirmCropBtn = document.getElementById('btn-confirm-crop');
let isDragging=false;
let startX,startY;
let selection=null;
let cropperShade=null;

let currentImage = null;
let currentImageF = null;

async function run() {
    await init();
    console.log('Rust img editor initialized');


    fileInput.addEventListener('change', async function() {
        console.log('change');
        if (fileInput.files.length > 0) {
            let reader = new FileReader();
            reader.onload = function (e) {
                image.setAttribute('src', e.target.result);
            }
            image.style.display = 'block';
            fileInput.style.display = 'none';
            reader.readAsDataURL(fileInput.files[0]);
            currentImage = new Uint8Array(await fileInput.files[0].arrayBuffer());
            currentImageF = new Uint8Array(await fileInput.files[0].arrayBuffer());
            const size = get_size(currentImage);
            setSize(size);
        }
    });

    fileInputRef.addEventListener('change', async function() {
        console.log('reference change');
        if (fileInputRef.files.length > 0) {
            let reader = new FileReader();
            reader.onload = function (e) {
                imageRef.setAttribute('src', e.target.result);
            }
            console.log('fileInputRef.files[0]', fileInputRef.files[0]);
            console.log(fileInputRef.files[0]);
            reader.readAsDataURL(fileInputRef.files[0]);
            switchBtn.style.display = 'block';
        }
    })

    switchBtn.addEventListener('click', async function() {
        console.log('switch');
        const fileBufferRef = await fileInputRef.files[0].arrayBuffer();
        const switchImageColor = switch_color(currentImage, new Uint8Array(fileBufferRef));
        currentImageF = switchImageColor;
        console.log(switchImageColor);
        showImage(switchImageColor);
        const size = get_size(new Uint8Array(switchImageColor));
        setSize(size);
        console.log(size);
    })

    width.addEventListener('change', function() {
        // resizeImage();
        const value = parseInt(width.value);
        resizeOneSide('width', value);
        // const value = parseInt(width.value);
        // scaleImage(value);
        // cropFunc();
    });

    height.addEventListener('change', function() {
        // resizeImage();
        const value = parseFloat(height.value);
        resizeOneSide('height', value);
        // cropFunc();
    });

    async function resizeImage() {
        const width = parseInt(document.getElementById('width').value);
        const height = parseInt(document.getElementById('height').value);
        console.log(width, height);
        const currentImage = fileInput.files[0];
        const fileBuffer = await currentImage.arrayBuffer();
        const resultFile = resize(new Uint8Array(fileBuffer), width, height);
        console.log(resultFile);
        showImage(resultFile);
    }

    async function resizeOneSide(key, value) {
        const resultFile = resize_one_side(currentImage, key, value);
        currentImage = resultFile;
        showImage(resultFile);
    }

    async function scaleImage(scale) {
        const currentImage = fileInput.files[0];
        const fileBuffer = await currentImage.arrayBuffer();
        const resultFile = change_scale_image(new Uint8Array(fileBuffer), scale);
        showImage(resultFile);
    }

    async function cropFunc() {
        const widthVal = parseInt(document.getElementById('width').value);
        const heightVal = parseInt(document.getElementById('height').value);
        const currentImage = fileInput.files[0];
        const fileBuffer = await currentImage.arrayBuffer();
        const resultFile = crop(new Uint8Array(fileBuffer), 0, 0, widthVal, heightVal);
        showImage(resultFile);
    }
}

function setSize(size) {
    width.value = size[0];
    height.value = size[1];
    console.log(size)
    console.log(width.value, height.value);
}

function showImage(imageData) {
    const blob = new Blob([imageData], { type: 'image/png' });
    const url = URL.createObjectURL(blob);
    image.setAttribute('src', url);
}


async function addSelectionBox(element) {

    element.addEventListener("mousedown",(event)=>{
        console.log('a',selection);
        if(selection){
            selection=null;
            const child = document.getElementById("selection");
            const cropperShade = document.getElementById("cropper-shade");
            element.removeChild(cropperShade);
            element.removeChild(child);
        }
        console.log('c', selection)
        isDragging=true;
        startX = event.clientX;
        startY = event.clientY;

        cropperShade = document.createElement("div");
        cropperShade.id = "cropper-shade";
        cropperShade.style.position = "absolute";
        cropperShade.style.left = 0;
        cropperShade.style.top = 0;
        cropperShade.style.width = `${width.value}px`;
        cropperShade.style.height = `${height.value}px`;
        cropperShade.style.backgroundColor = "rgba(0,0,0,0.5)";
        cropperShade.style.zIndex = "1";
        
        selection = document.createElement("div");
        selection.id = "selection";
        selection.style.position = "absolute";
        selection.style.left = `${startX}px`;
        selection.style.top = `${startY}px`;
        selection.style.with = "0px";
        selection.style.height = "0px";
        selection.style.background = "rgba(255,255,255,0)"; // Transparent background
        selection.style.border = "1px solid white";
        selection.style.borderRadius = "5px";
        selection.style.mixBlendMode = "destination-out";

        element.appendChild(selection);
        element.appendChild(cropperShade);
    });

    element.addEventListener("mousemove",(event)=>{
        if(isDragging) {
            const currentX = event.clientX;
            const currentY = event.clientY;
            const width = Math.abs(currentX-startX);
            const height = Math.abs(currentY-startY);
            const left = Math.min(startX-imageX,currentX-imageX);
            const top = Math.min(startY-imageY,currentY-imageY);
    
            // console.log("Dragging", width, height, left, top);
            selection.style.width = `${width}px`;
            selection.style.height = `${height}px`;
            selection.style.left = `${left}px`;
            selection.style.top = `${top}px`;
            // console.log('aaaa',selection.style.width, selection.style.height);
            selection.style.background ="rgba(255,255,255,0)";
            selection.style.border ="1px solid white";
            selection.style.borderRadius ="5px";
            element.style.backgroundColor = "rgba(0,0,0,0.5)";
            console.log(element.style.backgroundColor);

            cropperShade.style.clipPath = `rect(
                ${top}px
                ${left+width}px
                ${top+height}px
                ${left}px
            )`;
            
            // image.style.clipPath = `rect(
            //     ${top}px
            //     ${left+width}px
            //     ${top+height}px
            //     ${left}px
            // )`;
        }
    })

    element.addEventListener("mouseup",()=>{
        isDragging = false;

        if(selection) {
            const x = parseInt(selection.style.left);
            const y = parseInt(selection.style.top);
            const selectionWidth = parseInt(selection.style.width.split('xp')[0]);
            const selectionHeight = parseInt(selection.style.height.split('xp')[0]);
            let originalImage = new Image();
            originalImage.src = image.src;

            if (selectionWidth < 10 || selectionHeight < 10) {
                selection = null;
                const child = document.getElementById("selection");
                const cropperShade = document.getElementById("cropper-shade");
                element.removeChild(cropperShade);
                element.removeChild(child);
            }

        } 
    });

    confirmCropBtn.addEventListener("click", () => {
        console.log('b',selection)
        if(selection) {
            const x = parseInt(selection.style.left);
            const y = parseInt(selection.style.top);
            const selectionWidth = parseInt(document.getElementById('selection').style.width.split('px')[0]);
            const selectionHeight = parseInt(document.getElementById('selection').style.height.split('px')[0]);
            // console.log(x, y, selectionWidth, selectionHeight);
            // console.log('aaaa',selection.style.width, selection.style.height);

            let originalImage = new Image();
            originalImage.src = image.src;
            
            cropFunc(x, y, selectionWidth, selectionHeight);
            selection.remove();
            selection = null;
            cropperShade.remove();
            cropperShade = null;
            addSelectionBox(null)
        }
    })

    async function cropFunc(x, y, width, height) {
        console.log(x, y, width, height);
        const currentImage = fileInput.files[0];
        const fileBuffer = await currentImage.arrayBuffer();
        const resultFile = crop(new Uint8Array(fileBuffer), x, y, width, height);
        const size = get_size(new Uint8Array(resultFile));
        showImage(resultFile);
        setSize(size);

        cropbtn.style.display = 'block';
        confirmCropBtn.style.display = 'none';
    }
}

// async function main() {
//     console.log('main');
//     console.log('fileInput', fileInput);
//     await init();
//     console.log('Rust img editor initialized');

//     fileInput.addEventListener('change', async function() {
//         console.log('change');
//         if (fileInput.files.length > 0) {
//             let reader = new FileReader();
//             reader.onload = function (e) {
//                 image.setAttribute('src', e.target.result);
//             }
//             console.log('fileInput.files[0]', fileInput.files[0]);
//             console.log(image.src);
//             console.log(fileInput.files[0]);
//             reader.readAsDataURL(fileInput.files[0]);
//             const fileBuffer = await fileInput.files[0].arrayBuffer();
//             const size = get_size(new Uint8Array(fileBuffer));
//             console.log(size);
//             setSize(size);

//             console.log('aaa')
//             cropbtn.style.display = 'block';
//             cropbtn.addEventListener('click', function() {
//                 addSelectionBox(imageWrapper);
            
//                 cropbtn.style.display = 'none';
//                 confirmCropBtn.style.display = 'block';
//             })
//         }
//     });

// }

// main();
run();