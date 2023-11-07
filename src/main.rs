use duct::cmd;
use log::trace;
use reqwest;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub user_message: UserMessage,
    pub history: Vec<History>,
    pub style: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMessage {
    pub role: String,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub role: String,
    pub content: String,
}

fn truncate(s: &str, max_chars: usize) -> &str {
    s.trim()
        .replace("\n", " ")
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

fn get_error() -> String {
    let stdout = cmd!("cargo", "build")
        .unchecked()
        .stderr_to_stdout()
        .read()
        .unwrap()
        .split("\n")
        .filter(|s| s.starts_with("error"))
        .collect();
    return stdout;
}

fn main() {
    env_logger::init();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONNECTION,
        reqwest::header::HeaderValue::from_static("keep-alive"),
    );
    trace!("Headers: {:?}", headers);

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .default_headers(headers)
        .build()
        .unwrap();

    let msg = format!("Plz help with my rust error: {}", get_error());

    let root = Root {
        user_message: UserMessage {
            role: String::from("user"),
            content: truncate(&msg, 250).to_string(),
        },
        history: vec![History {
            role: String::from("assistant"),
            content: String::from(
                "Hello there, I am a roast bot that nitpicks all your Rust code compile errors",
            ),
        }],
        style: String::from("default"),
    };

    trace!("{:?}", root);

    let res = client
        .post("https://roastedby.ai/api/generate")
        .json(&root)
        .send()
        .expect("To send");

    trace!("Response: {:?}", res);

    let restext = &res.text().expect("DECODING");
    let jsontext: serde_json::Value =
        serde_json::from_str(restext).expect("JSON was not well-formatted");
    let fmttext = &jsontext["content"].to_string();
    trace!("fmttext: {}", fmttext);

    println!("{}", fmttext);
}
