window.onload = () => {
    var checkbox = document.getElementById("wrap-text-checkbox");
    var text_area = document.getElementById("upload-display-text")

    checkbox.addEventListener('change', (event) => {
        if (event.currentTarget.checked) {
            text_area.classList.add("wrap-text");
        } else {
            text_area.classList.remove("wrap-text");
        }
    });
}
