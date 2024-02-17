use clap::Parser;
use env_logger;
use file_diff::diff;
use inotify::{EventMask, Inotify, WatchMask};
use nostr_sdk::prelude::*;
use std::collections::HashSet;
use std::fs::{copy, read_to_string};

/// Nostr + Post = Nost
#[derive(Parser, Debug)]
struct Args {
    /// Directory to watch
    #[arg(short, long, default_value_t = String::from("./data/"))]
    watch: String,
}

fn get_keys() -> Keys {
    Keys::from_sk_str(read_to_string("./config/nsec.txt").unwrap().as_str().trim()).unwrap()
}

fn get_relays() -> Vec<String> {
    let mut relays = Vec::new();
    for line in read_to_string("./config/relays.txt").unwrap().lines() {
        if line.starts_with("#") {
            continue;
        }
        relays.push(line.to_string());
    }
    relays
}

fn relays_to_add(relays: Vec<String>, new_relays: Vec<String>) -> Vec<String> {
    // let mut new_relays_set: HashSet<String> = new_relays.into_iter().collect();
    // let mut relays_set: HashSet<String> = relays.into_iter().collect();
    // let mut result: Vec<String> = Vec::new();
    // for relay in new_relays_set.difference(&relays_set).into_iter() {
    //     result.push(relay.to_string());
    // }

    let mut result: Vec<String> = Vec::new();
    for relay in new_relays
        .into_iter()
        .collect::<HashSet<String>>()
        .difference(&relays.into_iter().collect())
        .into_iter()
    {
        result.push(relay.to_string());
    }
    result
}

fn relays_to_remove(relays: Vec<String>, new_relays: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for relay in relays
        .into_iter()
        .collect::<HashSet<String>>()
        .difference(&new_relays.into_iter().collect())
        .into_iter()
    {
        result.push(relay.to_string());
    }
    result
}

async fn get_client(keys: Keys) -> Result<Client> {
    let relays = get_relays();

    let client = Client::new(&keys);

    for relay in relays {
        client.add_relay(relay, None).await?;
    }

    client.connect().await;

    return Ok(client);
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> Result<()> {
    env_logger::init();

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
                log::info!("Empty!");
                continue;
            }
            if diff("./data/.content-before.txt", "./data/.content-current.txt") {
                log::info!("Same!");
                continue;
            }
            log::info!("--content begin--\n{}\n--content end--", content);

            let keys = get_keys();
            let client = get_client(keys).await?;

            let event: Event = EventBuilder::new_text_note(content, &[]).to_event(&keys)?;
            log::info!("Event id: {}", event.id);

            if let Err(_) = tokio::time::timeout(std::time::Duration::from_secs(2), async {
                if let Err(_) = client.send_event(event).await {
                    log::error!("client.publish_text_note Error");
                }
            })
            .await
            {
                log::debug!("Timeout!")
            }
            log::debug!("After publish");

            copy("./data/.content-current.txt", "./data/.content-before.txt")?;
            log::debug!("After copy cur -> bef");
        }
    }
}
