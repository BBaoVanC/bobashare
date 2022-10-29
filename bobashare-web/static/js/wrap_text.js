window.onload = () => {
    const checkbox = document.getElementById("wrap-text-checkbox");
    const text_area = document.getElementById("upload-display-text")

    function updateWrapText() {
        if (checkbox.checked) {
            text_area.classList.add("wrap-text");
        } else {
            text_area.classList.remove("wrap-text");
        }
    }

    checkbox.addEventListener('change', () => {
        updateWrapText();
    });

    // update on page load
    updateWrapText();
}
