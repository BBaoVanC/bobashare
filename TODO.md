# TODO

- https://docs.rs/axum/0.6.0-rc.1/axum/index.html

- [ ] Add IP banning (requires saving in metadata)
- [ ] Figure out file locking and concurrent safety

## Library

### File backend

- If a file is missing a metadata entry, warning should be sent
- Check for extraneous files, maybe in CLI

#### Directory structure

- `abc123xyz/`
  - `abc123xyz`
  - `metadata.json`

## Web backend

### URLs

#### HTML -- pretty view

Single file:

- `/abc123xyz`

#### Plaintext/binary -- raw view

Single file:

- `/raw/abc123xyz` - should return correct (original) filename

### Static frontend

- [ ] `/upload`

### Full frontend single-page application (React?)
