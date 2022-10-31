# TODO

## What needs to be done

- more unit tests
- integration tests

### Deployment

- Dockerfile
  - Example compose.yaml

### Frontend

- Clean up classes in CSS (the `upload-display-whatever` ones)
- Need a paste page (to easily paste & upload plaintext/code)
- API documentation page
- Set proper headers for `/static/` files
  - maybe read file metadata to find modified date
- Add embeds for Twitter/Discord
- Create a favicon
- Make bobascheme-light
- Make the uploaded files list on upload page persist between reloads
- Fix the horrors of the expiry dropdown logic in upload/paste pages

### Maybe

- i18n (maybe using https://crates.io/crates/fluent)
- animations
  - such as when a file upload is created and the box appears on the upload page
  - uploading progress bar

### Logging

- Add https://docs.rs/console-subscriber/latest/console_subscriber/
- Instrument main `bobashare` library
- Make sure there's enough instrumentation in bobashare-web

### Backend

- If a file is missing a metadata entry, warning should be sent
- Check for extraneous files, maybe in CLI
- Add IP banning (requires saving in metadata)
- Figure out file locking and concurrent safety
- Add `#[non_exhaustive]` to every Error enum (and maybe other enums)
- Derive `Debug` and `Clone` for as many types as possible

- Maybe during startup, loop through all uploads and delete expired ones
- https://github.com/pyrossh/rust-embed/issues/192

### Admin

- TODO: everything

## Additional notes

- `size` doesn't need to be included on `/api/v1/info/:id` endpoint since it can be found by `HEAD /raw/:id`
