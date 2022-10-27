# TODO

## What needs to be done

### Frontend

- CSS (styling)
- Clean up classes in CSS (the `upload-display-whatever` ones)
- Need an upload page
  - likely doesn't need any JS framework, unless there's something light
- Need a paste page (to easily paste & upload plaintext/code)
- use `build.rs` to create syntax css file automatically?
- Store delete keys in browser local storage so it's possible to delete uploads
  - this should be checked on the display page too; add a delete button
- API documentation page
- i18n (maybe using https://crates.io/crates/fluent)
- Set proper headers for `/static/` files
- Add embeds for Twitter/Discord
- Create a favicon

### Logging

- Add https://docs.rs/console-subscriber/latest/console_subscriber/

### Backend

- If a file is missing a metadata entry, warning should be sent
- Check for extraneous files, maybe in CLI
- Add IP banning (requires saving in metadata)
- Figure out file locking and concurrent safety
- Add `#[non_exhaustive]` to every Error enum (and maybe other enums)
- Derive `Debug` and `Clone` for as many types as possible

### Admin

- TODO: everything

## Additional notes

- `size` doesn't need to be included on `/api/v1/info/:id` endpoint since it can be found by `HEAD /raw/:id`
