# TODO

- https://docs.rs/axum/0.6.0-rc.1/axum/index.html

- [ ] Rename `backend::serialization` to `backend::serde`?

## Library

### File backend

- [ ] CLI to interact/test the file backend library

- If a file is missing a metadata entry, warning should be sent

#### Directory structure

Single file:

File structure:

- `abc123xyz/`
  - `abc123xyz.txt`
  - `metadata.json`

Multiple files:

File structure:

- `abc123xyz/`
  - `frontend.js`
  - `code.rs`
  - `program.exe`
  - `metadata.json`

Should a zip be automatically created for multi-file uploads?

## Web backend

### URLs

#### HTML -- pretty view

Single file:

- `/abc123xyz`
- `/abc123xyz.txt` - low priority, may be removed

Multiple files:

- `/abc123xyz/` - directory listing
- `/abc123xyz` - redirect to directory listing
- `/abc123xyz/frontend.js` - to single file
- `/abc123xyz.zip` - zip archive - should be possible to add

#### Plaintext/binary -- raw view

Single file:

- `/raw/abc123xyz` - should return correct (original) filename
- `/raw/abc123xyz.txt` - low priority, may be removed

Multiple files:

- `/raw/abc123xyz/` - redirect to directory listing (`/abc123xyz/`)
- `/raw/abc123xyz/frontend.js` - raw plaintext file

### REST API

- [ ] PUT/POST `/api/v1/upload` - upload a file
- [ ] GET `/api/v1/delete/<id>` - delete file
- [ ] GET `/api/v1/info/<id>` - get the Upload object

### Static frontend

- [ ] `/upload`

### Full frontend single-page application (React?)
