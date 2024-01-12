use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use futures::{stream, StreamExt};
use reqwest::Client;
use bytes::Bytes;
use clap::Parser;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Folder to download files to")]
    path: PathBuf,
    #[arg(help = "Session token")]
    session: String,
    #[arg(help = "URLs to download")]
    urls: Vec<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let folder = args.path;
    std::fs::create_dir_all(&folder).unwrap();
    let session = format!("MoodleSession={}", args.session);
    let client = Client::new();

    let bodies = stream::iter(&args.urls)
        .map(|url| {
            let client = &client;
            let session_clone = session.clone();
            async move {
                let resp = client.get(url).header("Cookie",&session_clone).send().await?;
                let text= resp.text().await?;
                let begin = text.find("Click ");
                if begin.is_none() {
                    return Err(anyhow::Error::msg("Could not find the text \"Click\" in the response. This might be because the URL is not a double-link but a direct link, or because your session token is invalid."));
                }
                let went_wrong = || anyhow::Error::msg("Something went wrong while parsing the response");
                let begin = begin.unwrap()+15;
                let end = begin+(text[begin..]).find("\"").ok_or_else(went_wrong)?;
                let name_begin = end+text[end..].find(">").ok_or_else(went_wrong)?+1;
                let name_end = name_begin+text[name_begin..].find("<").ok_or_else(went_wrong)?;
                let resp2 = client.get(&text[begin..end]).header("Cookie", &session_clone).send().await?;
                Ok((resp2.bytes().await?, text[name_begin..name_end].to_string()))
            }
        })
        .buffer_unordered(10);

    bodies
        .for_each(|x : anyhow::Result<(Bytes, String)>| async {
            match x {
                Ok((b, name)) => {
                    File::create(folder.join(Path::new(&name))).unwrap().write_all(&b[..]);
                }
                Err(e) => {
                    println!("Couldn't download file: {}", e);
                }
            }
        })
        .await;
}