# bobashare

A simple, fast, and anonymous file sharing server.

## Usage (client-side)

There are currently a couple of clients for simple usage of the bobashare API,
namely uploading files:

- [`bobashareupload`](https://github.com/shepgoba/bobashareupload)
- [`bobash`](https://github.com/kkrruumm/bobash)

Otherwise, see the [API docs](#api).

## Architecture

This program is separated into three different crates. `bobashare` is the
backend logic and platform API, and `bobashare-web` provides a webserver
interface between the `bobashare` backend. And `bobashare-admin` is currently an
abandoned alternative interface which uses a CLI to edit the storage instead.

## Setup (server-side)

*This section is for running your own bobashare server. If you just want to
upload files to an existing instance, see the [Usage
(client-side)](#usage-client-side) section.*

I designed bobashare to be as simple and clean to run as possible. You can
simply run the corresponding `bobashare-web` executable for your platform,
available under [Releases](https://github.com/BBaoVanC/bobashare/releases). It
will create the configured storage directory if it doesn't exist and listen on
the configured address/port. No files on the system are touched outside of the
storage directory.

However, do know that I don't have premade systemd unit service files yet
(although writing one will be trivial with how simple it is to run
`bobashare-web`) so running it as a service may be easiest using Docker
(compose).

### Docker

The image is `ghcr.io/bbaovanc/bobashare:latest`. You can copy an example
`compose.yaml` from [compose.example.yaml](compose.example.yaml), just delete
the `build:` line and uncomment the `image:` line.

## Configuration

`bobashare-web` accepts configuration via [TOML file](bobashare-web/bobashare.example.toml)
and/or environment variables (with env vars taking priority). The environment
variables are the regular names, but in all caps and with the prefix `APP_` (so
`base_url` would become `APP_BASE_URL`).

If using a config file, you must specify the path of it using the
`--config`/`-c` flag.

List of config options:

- `listen_addr` - default `127.0.0.1:3000` - the address and port to listen on
- `backend_path` - default `storage/` (relative to current directory) - the
  directory to use for storing all bobashare data (uploads and metadata)
- `cleanup_interval` - default `1h` - how often to run a cleanup task, where e
  loop through every upload in the store to delete expired ones
- `base_url` - default `http://localhost:3000/` - the url that the bobashare
  instance is being hosted on, used for generating upload URLs and CSS/JS paths
- `id_length` - default `8` - how many characters should each upload id be
- `default_expiry` - default `24h` - the default expiry time for new uploads,
  used as the default option in the UI, and if not explicitly chosen by API
  request
- `max_expiry` - default `30d` - the maximum expiry of an upload, can be set to
  `never` to allow non-expiring uploads
- `max_file_size` - default `1073741824` (1 GiB) - maximum size of an upload
- `extra_footer_text` - default empty - extra text to add to the footer, see the
  "Limits" blurb at the bottom on https://share.boba.best
- `about_page` - default empty - path to a markdown file to render on the about
  page at `/about/`

Also see the `--help` page for different verbosity settings.

## Rustdoc (internal code docs)

- [bobashare](https://bbaovanc.github.io/bobashare/bobashare/index.html)
- [bobashare-web](https://bbaovanc.github.io/bobashare/bobashare_web/index.html)

## API

### `/api/v1/`

#### GET `info/:id`

Get information (metadata) about an upload

**Request:** `GET /api/v1/info/:id`

**Arguments:**

- `:id` - the ID of the upload to query

**Successful response:** 200 OK, with JSON body in [InfoResponse][inforesponse-struct] format

**Example:**

```bashsession
$ curl https://share.example.com/api/v1/info/dXk1ODH5 | python -m json.tool
{
    "id": "dXk1ODH5",
    "url": "https://share.example.com/dXk1ODH5",
    "direct_url": "https://share.example.com/raw/dXk1ODH5",
    "filename": "20230526_170432.jpg",
    "mimetype": "image/jpeg",
    "creation_date": "2023-10-14T03:26:06.961405419Z",
    "expiry_date": "2023-10-15T03:26:06.961405419Z"
}
```

---

#### PUT `upload/:filename`

Create an upload

**Request:** `PUT /api/v1/upload/:filename`

**Arguments:**

- `:filename` - the name of the file being uploaded (required)

**Request headers:**

- `Content-Type` **(required)** - the mime type (file format) of the file. Note that
  it will be ignored if the file is found to be UTF-8 plaintext.
- `Bobashare-Expiry` *(optional)* - duration until the upload should expire
  - specify `0` for no expiry
  - examples (see
    [`duration_str`](https://docs.rs/duration-str/latest/duration_str/) for more information):
    - `1d` -- 1 day
    - `1h` -- 1 hour
    - `1m` -- 1 minute
    - `1s` -- 1 second
- `Bobashare-Delete-Key` *(optional)* - custom key to use for deleting the file
  later; if not provided, one will be randomly generated

**Request body:** The contents of the file

**Successful response:**

- 201 Created
- `Location` header with the URL of the upload
- JSON body [UploadResponse][uploadresponse-struct]

**Example:**

Please note that in this example, the trailing slash on the URL, combined with
using the `-T` flag means that curl automatically adds the filename to the end
of the URL. To the server, it appears as a `PUT
/api/v1/upload/joel-holland-TRhGEGdw-YY-unsplash.jpg`.

```bashsession
$ file --mime-type joel-holland-TRhGEGdw-YY-unsplash.jpg
joel-holland-TRhGEGdw-YY-unsplash.jpg: image/jpeg
$ curl -H 'Content-Type: image/jpeg' -T joel-holland-TRhGEGdw-YY-unsplash.jpg https://share.example.com/api/v1/upload/ | python -m json.tool
{
    "id": "ireyFMwu",
    "url": "https://share.example.com/ireyFMwu",
    "direct_url": "https://share.example.com/raw/ireyFMwu",
    "filename": "joel-holland-TRhGEGdw-YY-unsplash.jpg",
    "mimetype": "image/jpeg",
    "expiry_date": "2023-10-15T05:11:37.486763335Z",
    "delete_key": "joNtQd7TVKdBvlOmocueM35qU3JOqFuc"
}
```

---

#### DELETE `delete/:id`

Delete an upload

**Request:** `DELETE /api/v1/delete/:id`

**Arguments:**

- `:id` - the ID of the upload to delete

**Request body:** Should contain the `delete_key`, which was given in
[UploadResponse][uploadresponse-struct] when creating the upload.

**Successful response:** 204 No Content

**Example:**

```bashsession
$ curl -X DELETE https://share.example.com/api/v1/delete/ireyFMwu -d 'joNtQd7TVKdBvlOmocueM35qU3JOqFuc'
```


[inforesponse-struct]: https://bbaovanc.github.io/bobashare/bobashare_web/api/v1/info/struct.InfoResponse.html
[uploadresponse-struct]: https://bbaovanc.github.io/bobashare/bobashare_web/api/v1/upload/struct.UploadResponse.html
