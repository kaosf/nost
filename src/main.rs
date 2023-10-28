use clap::Parser;
use file_diff::diff;
use inotify::{EventMask, Inotify, WatchMask};
use nostr_sdk::prelude::*;
use std::fs::{copy, read_to_string};

/// Nostr + Post = Nost
#[derive(Parser, Debug)]
struct Args {
    /// Directory to watch
    #[arg(short, long, default_value_t = String::from("./data/"))]
    watch: String,
}

async fn publish(content: &str) -> Result<()> {
    let mut relays = Vec::new();
    for line in read_to_string("./config/relays.txt").unwrap().lines() {
        if line.starts_with("#") {
            continue;
        }
        relays.push(line.to_string());
    }
    let keys =
        Keys::from_sk_str(read_to_string("./config/nsec.txt").unwrap().as_str().trim()).unwrap();

    let client = Client::new(&keys);

    for relay in relays {
        client.add_relay(relay, None).await?;
    }

    client.connect().await;

    println!("--content--{}--", content);
    client.publish_text_note(content, &[]).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // let args = Args::parse();
    // println!("{}", args.watch);

    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    inotify
        .watches()
        .add(
            // "./data/content.txt", // ディレクトリ見ないとダメ
            "./data/",
            // args.watch,
            // WatchMask::DELETE | WatchMask::CREATE | WatchMask::MODIFY,
            WatchMask::MODIFY,
        )
        .expect("Failed to add inotify watch");

    let mut buffer = [0u8; 4096];
    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if event.mask.contains(EventMask::ISDIR) {
                continue;
            }
            if let Some(event_name) = event.name {
                if event_name != "content.txt" {
                    continue;
                }
            }

            if !event.mask.contains(EventMask::MODIFY) {
                continue;
            }

            copy("./data/content.txt", "./data/.content-current.txt")?;

            let binding = read_to_string("./data/.content-current.txt").unwrap();
            let content = binding.as_str().trim();
            if content == "" {
                println!("Empty!");
                continue;
            }
            if diff("./data/.content-before.txt", "./data/.content-current.txt") {
                println!("Same!");
                continue;
            }

            publish(content).await?;
            // TODO: 2秒でタイムアウトさせたい

            copy("./data/.content-current.txt", "./data/.content-before.txt")?;
            println!("After copy cur -> bef")
        }
    }
}
