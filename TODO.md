# TODO

## What needs to be done

### Frontend

- CSS (styling)
- Need an upload page
  - likely doesn't need any JS framework, unless there's something light
- Syntax highlighting
- Store delete keys in browser local storage so it's possible to delete uploads
  - this should be checked on the display page too
- API documentation page

### Logging

- Add https://docs.rs/console-subscriber/latest/console_subscriber/

### Backend

- If a file is missing a metadata entry, warning should be sent
- Check for extraneous files, maybe in CLI
- Add IP banning (requires saving in metadata)
- Figure out file locking and concurrent safety

## Additional notes

- `size` doesn't need to be included on `/api/v1/info/:id` endpoint since it can be found by `HEAD /raw/:id`
