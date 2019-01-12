use std::io::{Write, Result};
use std::fs::OpenOptions;

pub fn store_token(token: &str) {

    let krakenrc_file_path = dirs::home_dir().unwrap().join(".krakenrc");
    let file = OpenOptions::new().write(true).create(true).truncate(true).open(krakenrc_file_path).unwrap();

    if let Err(e) = writeln!(&file, "{}", token) {
        eprintln!("Couldn't write to file: {}", e);
    }

}

pub fn retrieve_token() -> Result<String> {

    let krakenrc_file_path = dirs::home_dir().unwrap().join(".krakenrc");

    match std::fs::read_to_string(krakenrc_file_path) {
        Ok(mut token) => {
            token.pop(); // remove the newline character
            Ok(token)
        },
        Err(error) => {
            Err(error)
        }
    }

}

#[derive(Deserialize, Debug)]
pub struct TokenJSON {
    pub auth: bool,
    pub token: String
}

#[derive(Deserialize, Debug)]
pub struct JobJSON {
    pub _id: String,
    pub machine: String,
    pub name: String,
    pub description: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LogJSON {
    pub job_id: String,
    pub line: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorJSON {
    pub auth: Option<bool>,
    pub message: String,
}

// Define a type so we can return multiple types of errors
pub enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
    KrakenServerError(ErrorJSON),
    Other(String),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}
