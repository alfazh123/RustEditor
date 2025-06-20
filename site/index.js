import init, * as wasm from "rust-img-editor";
// import init from "rust-img-editor";

// canvas 
// const fileInput = document.getElementById('file-input');
const canvas = document.getElementById('editing-canvas');
const ctx = canvas.getContext('2d');
let img = null;

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

let originalImage = null;
let currentImage = null;
let currentImageF = null;

// enhange item 
const sharpenRange = document.getElementById('sharpen-range');
const sharpValueElement = document.getElementById('value-sharp');
let sharpenValue = 0;

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
            image.style.display = 'flex';
            fileInput.style.display = 'none';
            reader.readAsDataURL(fileInput.files[0]);

            // for canvas
            // const file = fileInput.files[0];
            // img = new Image();
            // img.onload = function() {

            //     ctx.drawImage(img, 0, 0, img.width, img.height);
            //     console.log('img.width', img.width);
            // }
            // img.src = URL.createObjectURL(file);

            // set value image
            const initImage = wasm.fix_size_image(new Uint8Array(await fileInput.files[0].arrayBuffer()));
            originalImage = initImage;
            currentImage = initImage;
            currentImageF = initImage;
            const size = wasm.get_size(initImage);
            // originalImage = new Uint8Array(await fileInput.files[0].arrayBuffer());
            // currentImage = originalImage;
            // currentImageF = originalImage;
            // const size = wasm.get_size(new Uint8Array(await fileInput.files[0].arrayBuffer()));
            setSize(size);
            width.setAttribute('max', size[0]);
            height.setAttribute('max', size[1]);
        }
    });

    fileInputRef.addEventListener('change', async function() {
        console.log('reference change');
        if (fileInputRef.files.length > 0) {
            let reader = new FileReader();
            reader.onload = function (e) {
                imageRef.setAttribute('src', e.target.result);
            }
            imageRef.style.display = 'block';
            reader.readAsDataURL(fileInputRef.files[0]);
            console.log('fileInputRef.files[0]', fileInputRef.files[0]);
            console.log(fileInputRef.files[0]);
            switchBtn.style.display = 'block';
        }
    })

    switchBtn.addEventListener('click', async function() {
        console.log('switch');
        const fileBufferRef = await fileInputRef.files[0].arrayBuffer();
        const switchImageColor = wasm.switch_color(currentImage, new Uint8Array(fileBufferRef));
        currentImageF = switchImageColor;
        console.log(switchImageColor);
        showImage(switchImageColor);
        const size = wasm.get_size(new Uint8Array(switchImageColor));
        setSize(size);
        console.log(size);
    })

    width.addEventListener('input', function() {
        // resizeImage();
        if (width.value < 10) {
            width.value = 10;
        }
        const widthVal = parseInt(width.value);
        const heightVal = parseInt(height.value);
        resizeOneSide(widthVal, heightVal);
        console.log('width', width.value);
        console.log('height', height.value);
        // const value = parseInt(width.value);
        // scaleImage(value);
        // cropFunc();
    });

    height.addEventListener('input', function() {
        // resizeImage();
        if (height.value < 10) {
            height.value = 10
        }
        const widthVal = parseInt(width.value);
        const heightVal = parseInt(height.value);
        resizeOneSide(widthVal, heightVal);
        console.log('width', width.value);
        console.log('height', height.value);
        // cropFunc();
    });

    async function resizeImage() {
        const width = parseInt(document.getElementById('width').value);
        const height = parseInt(document.getElementById('height').value);
        console.log(width, height);
        const currentImage = fileInput.files[0];
        const fileBuffer = await currentImage.arrayBuffer();
        const resultFile = wasm.resize(new Uint8Array(fileBuffer), width, height);
        console.log(resultFile);
        showImage(resultFile);
    }

    async function resizeOneSide(width, height) {
        const resultFile = wasm.resize_exact(currentImage, width, height);
        currentImageF = resultFile;
        showImage(resultFile);
    }

    async function scaleImage(scale) {
        const currentImage = fileInput.files[0];
        const fileBuffer = await currentImage.arrayBuffer();
        const resultFile = wasm.change_scale_image(new Uint8Array(fileBuffer), scale);
        showImage(resultFile);
    }

    async function cropFunc() {
        const widthVal = parseInt(document.getElementById('width').value);
        const heightVal = parseInt(document.getElementById('height').value);
        const currentImage = fileInput.files[0];
        const fileBuffer = await currentImage.arrayBuffer();
        const resultFile = wasm.crop(new Uint8Array(fileBuffer), 0, 0, widthVal, heightVal);
        showImage(resultFile);
    }

    enhanceFunc();
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

    // for canvas
    // const blob = new Blob([imageData], { type: 'image/png' });
    // const url = URL.createObjectURL(blob);
    // const img = new Image();
    // img.onload = function() {
    //     ctx.drawImage(img, 0, 0, img.width, img.height);
    // }
    // img.src = url;
}


async function enhanceFunc() {
    sharpenRange.addEventListener('input', async function() {
        sharpenValue = parseInt(sharpenRange.value);
        console.log('sharpenValue', sharpenValue);
        sharpValueElement.innerText = `${sharpenValue > 0 ? "+":""}${sharpenValue}`;
        // const resultFile = wasm.sharpen(originalImage, sharpenValue);
        const resultFile = wasm.blur(originalImage, sharpenValue);
        showImage(resultFile);
    });
};

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

run();

// navigation

function navigation() {

    const bottomNav = document.getElementById("bottom-nav");
    
    // Bottom nav menu
    const adjustNav = document.getElementById("adjust-nav");
    const cropNav = document.getElementById("crop-nav");
    const filterNav = document.getElementById("filter-nav");
    const enhanceNav = document.getElementById("enhance-nav");
    
    // Action wrappers
    const actionCropWrapper = document.getElementById("action-crop-wrapper");
    const actionAdjustWrapper = document.getElementById("action-adjust-wrapper");
    const actionFilterWrapper = document.getElementById("action-filter-wrapper");
    const actionEnhanceWrapper = document.getElementById("action-enhance-wrapper");
    
    bottomNav.style.display = "none";
    
    fileInput.addEventListener("change", (e) => {
        const file = e.target.files[0];
        if (file) {
            bottomNav.style.display = "flex"; // Show the bottom nav
        }
    });
    
    
    // Event listeners for bottom nav
    adjustNav.addEventListener("click", () => {
        if (actionAdjustWrapper.classList.contains("active")) {
            actionAdjustWrapper.classList.remove("active");
        } else {
            actionAdjustWrapper.classList.add("active");
            actionCropWrapper.classList.remove("active");
            actionFilterWrapper.classList.remove("active");
            actionEnhanceWrapper.classList.remove("active");
        }
    });
    
    cropNav.addEventListener("click", () => {
        if (actionCropWrapper.classList.contains("active")) {
            actionCropWrapper.classList.remove("active");
        } else {
            actionCropWrapper.classList.add("active");
            actionAdjustWrapper.classList.remove("active");
            actionFilterWrapper.classList.remove("active");
            actionEnhanceWrapper.classList.remove("active");
        }
    });
    
    filterNav.addEventListener("click", () => {
        if (actionFilterWrapper.classList.contains("active")) {
            actionFilterWrapper.classList.remove("active");
        } else {
            actionFilterWrapper.classList.add("active");
            actionCropWrapper.classList.remove("active");
            actionAdjustWrapper.classList.remove("active");
            actionEnhanceWrapper.classList.remove("active");
        }
    });
    
    enhanceNav.addEventListener("click", () => {
        if (actionEnhanceWrapper.classList.contains("active")) {
            actionEnhanceWrapper.classList.remove("active");
        } else {
            actionEnhanceWrapper.classList.add("active");
            actionCropWrapper.classList.remove("active");
            actionAdjustWrapper.classList.remove("active");
            actionFilterWrapper.classList.remove("active");
        }
    });
}

navigation();

// canvas pan and zoom

function canvasViewPort() {

    const viewportTransform = {
        x: 0,
        y: 0,
        scale: 1
    }
    
    // From here on, everything we'll write will go below 👇
    const drawRect = (x, y, width, height, color) => {
        ctx.fillStyle = color
        ctx.fillRect(x, y, width, height)
    }
    
    const render = () => {
      // New code 👇
        ctx.setTransform(1, 0, 0, 1, 0, 0);
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.setTransform(viewportTransform.scale, 0, 0, viewportTransform.scale, viewportTransform.x, viewportTransform.y);
        // New Code 👆
    
        // drawRect(0, 0, 100, 100, 'red');
        // drawRect(200, 200, 100, 100, 'blue');
        showImage(originalImage);
    }
    
    
    // We need to keep track of our previous mouse position for later
    let previousX = 0, previousY = 0;
    
    const updatePanning = (e) => {
        const localX = e.clientX;
        const localY = e.clientY;
    
        viewportTransform.x += localX - previousX;
        viewportTransform.y += localY - previousY;
    
        previousX = localX;
        previousY = localY;
    }
    
    const updateZooming = (e) => {
    
        const oldScale = viewportTransform.scale;
        const oldX = viewportTransform.x;
        const oldY = viewportTransform.y;
    
        const localX = e.clientX;
        const localY = e.clientY;
    
        const previousScale = viewportTransform.scale;
    
        const newScale = viewportTransform.scale += e.deltaY * -0.01;
    
        const newX = localX - (localX - oldX) * (newScale / previousScale);
        const newY = localY - (localY - oldY) * (newScale / previousScale);
    
        viewportTransform.x = newX;
        viewportTransform.y = newY;
        viewportTransform.scale = newScale;
    }
    
    const onMouseMove = (e) => {
        updatePanning(e)
    
        render()
    
        console.log(e)
    }
    
    const onMouseWheel = (e) => {
        updateZooming(e)
    
        render()
    
        console.log(e)
    }
    
    canvas.addEventListener("wheel", onMouseWheel);
    
    canvas.addEventListener("mousedown", (e) => {
        previousX = e.clientX;
        previousY = e.clientY;
    
        canvas.addEventListener("mousemove", onMouseMove);
    })
    
    canvas.addEventListener("mouseup", (e) => {
        canvas.removeEventListener("mousemove", onMouseMove);
    })
    
    render()
};

canvasViewPort();
