use chrono::{DateTime, TimeDelta, Utc};

use crate::storage::upload::Upload;

pub(crate) fn upload_id() -> String {
    String::from("abc123xyz/")
}

pub(crate) fn creation_date() -> DateTime<Utc> {
    "2022-08-29T01:02:19.824375631Z"
        .parse::<DateTime<Utc>>()
        .unwrap()
}
pub(crate) fn expiry_date() -> DateTime<Utc> {
    creation_date() + TimeDelta::try_days(4).unwrap()
}

pub(crate) fn example_upload_no_expiry() -> Upload {
    Upload {
        id: upload_id(),
        filename: String::from("code.py"),
        mimetype: "text/x-python".parse().unwrap(),
        creation_date: creation_date(),
        expiry_date: None,
        delete_key: String::from("jasdflyhdas87nmgc7gf7342gcir874g23"),
    }
}
pub(crate) fn example_upload_with_expiry() -> Upload {
    let mut upload = example_upload_no_expiry();
    upload.expiry_date = Some(expiry_date());
    upload
}

pub(crate) const EXAMPLE_UNKNOWN_VERSION: &str = r#"{"version":"-1"}"#;
pub(crate) const EXAMPLE_UPLOADV0_NO_EXPIRY_SERIALIZED: &str = r#"{"version":"0","filename":"code.py","mimetype":"text/x-python","creation_date":"2022-08-29T01:02:19.824375631Z","expiry_date":null,"delete_key":"jasdflyhdas87nmgc7gf7342gcir874g23"}"#;
pub(crate) const EXAMPLE_UPLOADV0_WITH_EXPIRY_SERIALIZED: &str = r#"{"version":"0","filename":"code.py","mimetype":"text/x-python","creation_date":"2022-08-29T01:02:19.824375631Z","expiry_date":"2022-09-02T01:02:19.824375631Z","delete_key":"jasdflyhdas87nmgc7gf7342gcir874g23"}"#;
