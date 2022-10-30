window.onload = () => {
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
