# TODO

- make the error messages include the correct variables to be displayed well so we don't have to loop through the sources

## Testing

- more unit tests
- integration tests
- make tests to run JS/CSS linters and compatibility checkers
- run JS/HTML/CSS format checkers
- test locking

### Frontend

- Clean up classes in CSS (the `upload-display-whatever` ones)
  - especially the markdown display ones
- API documentation page
- Create a favicon
- Make bobascheme-light
- Support changing default expiry unit (and guess intelligently too if not set)
- Change docs to private and binary (not library)
- Paste image to upload
- Drop file to upload
- Automatically expire old uploads from local storage

- `build.rs` to generate syntax CSS, and also make sure to properly rebuild on the right dependency files (the syntax source)

#### Maybe

- Make the uploaded files list on upload page persist between reloads
- Replace the single-file upload form at the top with automatically creating a "pending" upload entry
  - this would add multi-file support
  - each pending upload would have its own submit and possibly even expiry selection
  - MAYBE MAYBE: start uploading first, select expiry and stuff later (while it's uploading)

- i18n (maybe using https://crates.io/crates/fluent)
- animations
  - such as when a file upload is created and the box appears on the upload page
  - uploading progress bar

### Revamp upload page

![light](https://cdn.discordapp.com/attachments/1018368926494769314/1036779113245057064/Desktop_-_1.png)
![dark](https://cdn.discordapp.com/attachments/1018368926494769314/1036779113597382779/Desktop_-_2.png)

- find better way to show selected upload filename
- use fontawesome icons (maybe svgs)
  - icon to show that successful upload filename is a link
- brighter/more contrast in-progress & successful upload background
- retry button for failed upload
- make percentage come after the bar (follow the right edge of it)

### Logging

- Add https://docs.rs/console-subscriber/latest/console_subscriber/
- Instrument main `bobashare` library
- Make sure there's enough instrumentation in bobashare-web

### Backend

- Add IP banning (requires saving in metadata)
- Add `#[non_exhaustive]` to every Error enum (and maybe other enums)
- Derive `Debug` and `Clone` for as many types as possible
- Make a system to delete expired uploads immediately as they expire instead of waiting until cleanup task
- Way to serve static files directly via webserver instead of through bobashare
- Maybe SIGINT shouldn't terminate all active uploads instantly
- Use an in-memory structure for upload locking instead of `.lock` file

- Possibly support multiple files in a single upload
  - or at the bare minimum, previewing individual files of a zip

- https://github.com/pyrossh/rust-embed/issues/192
- Grafana/Prometheus exporter

#### Renames to do for a backwards-incompatible update

- `backend_path` should be `storage_path`
- the `APP_` prefix for env vars should be `BOBASHARE_`

### Admin

- TODO: everything

## Additional notes

- `size` doesn't need to be included on `/api/v1/info/:id` endpoint since it can be found by `HEAD /raw/:id`
