use std::env;
use std::fmt::format;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use futures::{stream, StreamExt};
use reqwest::{Client, Error};
use bytes::Bytes;
use tokio;

#[tokio::main]
async fn main() {
    let mut args : Vec<String> = env::args().collect();
    if !args[1].ends_with("/") {
        args[1] = format!("{}/", args[1]);
    }
    let folder = Path::new(&args[1]);
    std::fs::create_dir_all(folder).unwrap();
    let session = format!("MoodleSession={}", args[2]);
    let client = Client::new();

    let bodies = stream::iter(&args[3..])
        .map(|url| {
            let client = &client;
            let session_clone = session.clone();
            async move {
                let resp = client.get(url).header("Cookie",&session_clone).send().await?;
                let text= resp.text().await?;
                let begin = text.find("Click ");
                if begin.is_none() {
                    panic!("Invalid session token (probably)! You'll want to check that.");
                }
                let begin = begin.unwrap()+15;
                let end = begin+(text[begin..]).find("\"").unwrap();
                let name_begin = end+text[end..].find(">").unwrap()+1;
                let name_end = name_begin+text[name_begin..].find("<").unwrap();
                let resp2 = client.get(&text[begin..end]).header("Cookie", &session_clone).send().await?;
                Ok((resp2.bytes().await, text[name_begin..name_end].to_string()))
            }
        })
        .buffer_unordered(10);

    bodies
        .for_each(|x : Result<(Result<Bytes, Error>, String), Error>| async {
            let (b, name) = x.unwrap();
            let b = b.unwrap();
            File::create(folder.join(Path::new(&name))).unwrap().write_all(&b[..]);
        })
        .await;
}