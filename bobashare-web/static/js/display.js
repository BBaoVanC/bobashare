window.onload = () => {
    wrapTextHandler();
    markdownPreviewHandler();

    const deleteScriptElem = document.getElementById("delete-script-element");
    const uploadId = deleteScriptElem.getAttribute("data-upload-id");

    const titlebarRight = document.querySelector(".upload-titlebar-right");

    const deleteButtonTemplate = document.getElementById("delete-button-template");

    const deleteKey = localStorage.getItem(uploadId + "-delete-key");
    if (deleteKey) {
        const tmpl = deleteButtonTemplate.content.cloneNode(true);
        tmpl.querySelector(".delete-button").onclick = () => {
            const req = new XMLHttpRequest();
            req.open("DELETE", "/api/v1/delete/" + uploadId);
            req.responseType = "json";
            req.onload = () => {
                if (req.status >= 200 && req.status < 300) {
                    localStorage.removeItem(uploadId + "-delete-key");
                    window.location = "/";
                } else {
                    console.error("delete failed", req);
                    alert("Failed to delete upload: " + req.response.message);
                }
            }
            req.send(deleteKey);
        }
        titlebarRight.appendChild(tmpl);
    }
}

const wrapTextHandler = () => {
    const checkbox = document.getElementById("wrap-text-checkbox");
    if (checkbox == null) {
        return;
    }

    const textarea = document.getElementById("upload-display-text")
    const updateWrapText = () => {
        if (checkbox.checked) {
            textarea.classList.add("wrap-text");
        } else {
            textarea.classList.remove("wrap-text");
        }
    }
    checkbox.addEventListener('change', () => {
        updateWrapText();
    });

    // update on page load
    updateWrapText();
}

const markdownPreviewHandler = () => {
    const markdownContainer = document.querySelector(".upload-display-markdown-container");
    if (markdownContainer == null) {
        return;
    }

    const renderedTab = document.getElementById("markdown-tab-rendered");
    const sourceTab = document.getElementById("markdown-tab-source");

    renderedTab.onclick = () => {
        renderedTab.classList.add("active");
        sourceTab.classList.remove("active");
        markdownContainer.classList.add("rendered");
    }
    sourceTab.onclick = () => {
        sourceTab.classList.add("active");
        renderedTab.classList.remove("active");
        markdownContainer.classList.remove("rendered");
    }
}
