# TODO

## Testing

- more unit tests
- integration tests
- make tests to run JS/CSS linters and compatibility checkers
- run JS/HTML/CSS format checkers

### Frontend

- Clean up classes in CSS (the `upload-display-whatever` ones)
  - especially the markdown display ones
- API documentation page
- Create a favicon
- Make bobascheme-light
- Support changing default expiry unit (and guess intelligently too if not set)
- Change docs to private and binary (not library)

#### Maybe

- Make the uploaded files list on upload page persist between reloads
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
- support multiple files
- make percentage come after the bar

### Logging

- Add https://docs.rs/console-subscriber/latest/console_subscriber/
- Instrument main `bobashare` library
- Make sure there's enough instrumentation in bobashare-web

### Backend

- If a file is missing a metadata entry, warning should be sent
- Check for extraneous files, maybe in CLI
- Add IP banning (requires saving in metadata)
- Figure out file locking and concurrency safety
- Add `#[non_exhaustive]` to every Error enum (and maybe other enums)
- Derive `Debug` and `Clone` for as many types as possible

- Maybe during startup, loop through all uploads and delete expired ones
- https://github.com/pyrossh/rust-embed/issues/192
- Grafana/Prometheus exporter

### Admin

- TODO: everything

## Additional notes

- `size` doesn't need to be included on `/api/v1/info/:id` endpoint since it can be found by `HEAD /raw/:id`
