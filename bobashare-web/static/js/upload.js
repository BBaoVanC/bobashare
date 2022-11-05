window.onload = () => {
    const uploadScriptElem = document.getElementById("upload-script-element");
    const uploadEndpoint = uploadScriptElem.getAttribute("data-upload-api-endpoint");
    const deleteEndpoint = uploadScriptElem.getAttribute("data-delete-api-endpoint");
    const infoEndpoint = uploadScriptElem.getAttribute("data-info-api-endpoint");

    const form = document.getElementById("upload-form");
    const filesDiv = document.getElementById("uploaded-files");

    const fileProgressTemplate = document.getElementById("upload-progress-template");
    const fileSuccessTemplate = document.getElementById("upload-success-template");
    const fileDeletedTemplate = document.getElementById("upload-deleted-template");
    const fileCancelledTemplate = document.getElementById("upload-cancelled-template");
    const fileFailTemplate = document.getElementById("upload-fail-template");
    const filePlaceholderTemplate = document.getElementById("upload-placeholder-template");

    const fileInput = document.getElementById("upload-file");
    const expiryNumInput = document.getElementById("upload-expiry-number");
    const expiryUnitInput = document.getElementById("upload-expiry-unit");

    for (let i = 0; i < localStorage.length; i++) {
        const key = localStorage.key(i);
        const deleteKey = localStorage.getItem(key);
        const id = key.replace(/-delete-key$/, "");
        const tmpl = filePlaceholderTemplate.content.cloneNode(true);
        tmpl.querySelector(".upload-id").innerText = id;
        filesDiv.appendChild(tmpl);
        const placeholderElem = filesDiv.lastElementChild;

        const req = new XMLHttpRequest();
        req.open("GET", infoEndpoint + id);
        req.responseType = "json";
        req.onreadystatechange = () => {
            if (req.readyState != XMLHttpRequest.DONE) {
                return;
            }
            if (req.status == 404) {
                localStorage.removeItem(key);
                placeholderElem.remove();
                return;
            }
            if (req.status >= 200 && req.status < 300) {
                const tmpl = fileSuccessTemplate.content.cloneNode(true);
                const filename = req.response.filename;
                tmpl.querySelector(".upload-filename").innerText = filename;
                tmpl.querySelector(".upload-filename").href = req.response.url;
                tmpl.querySelector(".upload-delete").onclick = () => {
                    // TODO: deduplicate this code
                    const req = new XMLHttpRequest();
                    req.open("DELETE", deleteEndpoint + id);
                    req.onreadystatechange = () => {
                        if (req.readyState !== XMLHttpRequest.DONE) {
                            return;
                        }
                        if (req.status >= 200 && req.status < 300) {
                            const deletedTmpl = fileDeletedTemplate.content.cloneNode(true);
                            deletedTmpl.querySelector(".upload-filename").innerText = filename;
                            console.log("successElem", successElem);
                            filesDiv.replaceChild(deletedTmpl, successElem);
                            localStorage.removeItem(key);
                        } else {
                            console.error(`delete of ${id} failed`, req);
                            alert(`delete of ${id} failed: ` + req.responseText);
                        }
                    }
                    req.send(deleteKey);
                }
                filesDiv.replaceChild(tmpl, placeholderElem);
                // TODO: this isnt working
                const successElem = filesDiv.lastElementChild;
            }
        }
        req.send();
    }

    form.onsubmit = event => {
        event.preventDefault();
        const file = fileInput.files[0];

        const tmpl = fileProgressTemplate.content.cloneNode(true);
        tmpl.querySelector(".upload-filename").innerText = file.name;
        const cancelElem = tmpl.querySelector(".upload-cancel");

        const progressElem = tmpl.querySelector(".upload-progress");
        const progressBarElem = tmpl.querySelector(".upload-progress-bar");
        progressElem.innerText = 0;
        progressBarElem.style.width = "0%";

        filesDiv.appendChild(tmpl);
        const uploadElem = filesDiv.lastElementChild;

        const req = new XMLHttpRequest();
        req.open("PUT", uploadEndpoint + file.name);
        req.setRequestHeader("Content-Type", file.type || "application/octet-stream");
        req.setRequestHeader("Bobashare-Expiry", expiryNumInput.value + expiryUnitInput.value);
        req.responseType = "json";
        req.upload.onprogress = event => {
            // const progress = (event.loaded / event.total) * 100;
            // progressElem.innerText = Math.round(progress) + "%";
            // progressBarElem.style.width = progress + "%";
            const progress = Math.round((event.loaded / event.total) * 100) + "%";
            progressElem.innerText = progress;
            progressBarElem.style.width = progress;
        }
        cancelElem.onclick = () => {
            // remove the readystatechange callback because for some reason it
            // gets called even when abort() is called
            // seems to be both Chrome and Firefox breaking spec:
            // > No readystatechange event is dispatched.
            // https://xhr.spec.whatwg.org/#the-abort()-method
            // other workarounds are less fun
            req.onreadystatechange = null;
            req.abort();
        }
        cancelElem.onkeypress = event => {
            if (event.key === "Enter") cancelElem.click();
        }
        req.onabort = () => {
            const cancelTmpl = fileCancelledTemplate.content.cloneNode(true);
            cancelTmpl.querySelector(".upload-filename").innerText = file.name;
            filesDiv.replaceChild(cancelTmpl, uploadElem);
        }
        req.onreadystatechange = () => {
            if (req.readyState !== XMLHttpRequest.DONE) {
                return;
            }

            if (req.status >= 200 && req.status < 300) {
                const successTmpl = fileSuccessTemplate.content.cloneNode(true);
                successTmpl.querySelector(".upload-filename").innerText = file.name;
                successTmpl.querySelector(".upload-filename").href = req.response.url;
                const deleteLink = successTmpl.querySelector(".upload-delete");

                filesDiv.replaceChild(successTmpl, uploadElem);
                const successElem = filesDiv.lastElementChild;

                const id = req.response.id;
                const deleteKey = req.response.delete_key;
                // this can be used on display page to delete/recreate file
                // it's not used here since all the files disappear on reload
                // TODO: it would be nice to make this key expire after the file expires
                localStorage.setItem(id + "-delete-key", deleteKey);

                deleteLink.onclick = () => {
                    const req = new XMLHttpRequest();
                    req.open("DELETE", deleteEndpoint + id);
                    req.onreadystatechange = () => {
                        if (req.readyState !== XMLHttpRequest.DONE) {
                            return;
                        }
                        if (req.status >= 200 && req.status < 300) {
                            const deletedTmpl = fileDeletedTemplate.content.cloneNode(true);
                            deletedTmpl.querySelector(".upload-filename").innerText = file.name;
                            filesDiv.replaceChild(deletedTmpl, successElem);
                        } else {
                            console.error(`delete of ${id} failed`, req);
                            alert(`delete of ${id} failed: ` + req.responseText);
                        }
                    }
                    req.send(deleteKey);
                }
                deleteLink.onkeypress = event => {
                    if (event.key === "Enter") deleteLink.click();
                }
            } else {
                const failTmpl = fileFailTemplate.content.cloneNode(true);
                failTmpl.querySelector(".upload-filename").innerText = file.name;
                filesDiv.replaceChild(failTmpl, uploadElem);
                console.error(`upload of ${file.name} failed`, req);
                alert(`upload of ${file.name} failed: ` + req.response.message);
            }
        }
        req.send(file);
        fileInput.value = null;
    }
}
