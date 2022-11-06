window.onload = () => {
    const pasteScriptElem = document.getElementById("paste-script-element");
    const uploadEndpoint = pasteScriptElem.getAttribute("data-upload-api-endpoint");
    const deleteEndpoint = pasteScriptElem.getAttribute("data-delete-api-endpoint");

    const form = document.getElementById("paste-form");

    const textarea = document.getElementById("paste-textarea");
    const filenameInput = document.getElementById("paste-filename");
    const extensionInput = document.getElementById("paste-extension");
    const expiryNumInput = document.getElementById("paste-expiry-number");
    const expiryUnitInput = document.getElementById("paste-expiry-unit");
    const submitButton = document.getElementById("paste-submit");
    expiryUnitInput.onchange = () => {
        if (expiryUnitInput.value == "never") {
            expiryNumInput.style.display = "none";
        } else {
            expiryNumInput.style.display = "inline-block";
        }
    }
    /* if the user previously selected never before the event was registered */
    expiryUnitInput.onchange();

    form.onsubmit = event => {
        event.preventDefault()
        submitButton.disabled = true;

        const req = new XMLHttpRequest();
        req.open("PUT", uploadEndpoint + `${filenameInput.value}.${extensionInput.value}`);
        req.setRequestHeader("Content-Type", "text/plain");
        req.setRequestHeader("Bobashare-Expiry", expiryNumInput.value + expiryUnitInput.value);
        req.responseType = "json";
        req.onreadystatechange = () => {
            if (req.readyState != XMLHttpRequest.DONE) {
                return;
            }

            if (req.status >= 200 && req.status < 300) {
                submitButton.disabled = false;
                localStorage.setItem(req.response.id + "-delete-key", req.response.delete_key);
                window.location.href = req.response.url;
            } else {
                submitButton.disabled = false;
                console.error("paste failed", req);
                alert("paste failed: " + req.response.message);
            }
        }
        req.send(textarea.value);
    }
}
