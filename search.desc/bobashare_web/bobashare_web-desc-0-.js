searchState.loadedDescShard("bobashare_web", 0, "Webserver written with <code>axum</code> which provides a frontend and …\nA struct that contains all the state and config for …\n<code>ClassStyle</code> used for <code>syntect</code> highlighting\nPrefix for CSS classes used for <code>syntect</code> highlighting\nstring does not match duration format (try: 15d)\nOptions used for <code>pulldown_cmark</code> rendering\ncould not parse number in duration, is it too large?\nError encountered in converting string to duration values …\nPublic facing REST API for bobashare\nstorage backend\nbase URL (ex. <code>http://localhost:3000/</code>)\nTake the requested expiry, and make sure it’s within the …\nhow often between each cleanup\ndefault expiry time\nextra text to display in footer\nReturns the argument unchanged.\nReturns the argument unchanged.\nlength of randomly generated IDs\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nmaximum expiry time (<code>None</code> for no maximum)\nmaximum file size in bytes\nbase URL for downloading raw upload files (ex. …\nchannel to broadcast shutdown – will force all uploads …\nHandler to serve static files\nTake a string with a simple duration format (single number …\nFrontend views (as opposed to REST API)\nRoutes under <code>/api/</code>\nVersion 1 of the bobashare API, hosted at <code>/api/v1/</code>\nMethod to convert an <code>std::error::Error</code> into a <code>Response</code> …\nAPI to delete an upload\nAPI to get metadata about an upload\nConsume the error and convert it to a <code>Response</code> with the …\nRoutes under <code>/api/v1/</code>\nAPI to create an upload\nErrors that could occur when deleting an upload\nincorrect delete key\ninternal server error\nan upload at the specified id was not found\nDelete an upload\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nErrors when querying info about an upload\nSuccessful upload info API response\ninternal server error\nan upload at the specified id was not found\ndate the upload was created\ndirect URL to download the upload file\ndate the upload expires, or None if it never expires\nfilename of the uploaded file\nReturns the argument unchanged.\nReturns the argument unchanged.\nID of the upload\nGet information (metadata) about an upload\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMIME type of the file\nURL of the upload\nan upload already exists with the same id\nupload was cancelled\ninternal server error\nerror parsing <code>{name}</code> header\nfile is too large ({size} &gt; {max})\nErrors that could occur during upload\nThe JSON API response after uploading a file\nkey to delete the upload later before it’s expired\ndirect url to download the raw uploaded file\nexpiration date in RFC 3339 format, null if the upload …\nthe name of the file\nReturns the argument unchanged.\nReturns the argument unchanged.\nID of the upload (used in URL)\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nthe MIME type of the uploaded file\nCreate an upload\nurl to the upload\nRoutes to display or download an upload in a browser\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\ninternal server error\nan upload at the specified id was not found\nErrors when trying to view/download an upload\nDisplay an upload as HTML\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nDownload the raw upload file\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")