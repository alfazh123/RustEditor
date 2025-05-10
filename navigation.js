const inputFile = document.getElementById("file-input");

// bottom nav
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

inputFile.addEventListener("change", (e) => {
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

// Event listeners for action wrappers
