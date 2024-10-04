use clap::Parser;
use env_logger;
use file_diff::diff;
use inotify::{EventMask, Inotify, WatchMask};
use nostr_sdk::prelude::*;
use std::{
    fs::{copy, read_to_string},
    path::PathBuf,
};

/// Nostr + Post = Nost
#[derive(Parser, Debug)]
struct Args {
    /// Directory for config
    #[arg(short, long, default_value_t = String::from("./config/"))]
    config: String,
    /// Directory to watch
    #[arg(short, long, default_value_t = String::from("./data/"))]
    watch: String,
}

fn get_keys(config_dir: &PathBuf) -> Keys {
    Keys::from_sk_str(
        read_to_string(config_dir.join("nsec.txt"))
            .unwrap()
            .as_str()
            .trim(),
    )
    .unwrap()
}

async fn get_client(config_dir: &PathBuf, keys: Keys) -> Result<Client> {
    let mut relays = Vec::new();
    for line in read_to_string(config_dir.join("relays.txt"))
        .unwrap()
        .lines()
    {
        if line.starts_with("#") {
            continue;
        }
        relays.push(line.to_string());
    }

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

    let args = Args::parse();
    let config_dir = PathBuf::from(args.config);
    log::info!("Config directory: {}", config_dir.display());
    let data_dir = PathBuf::from(args.watch);
    log::info!("Watch directory: {}", data_dir.display());

    let mut inotify = Inotify::init().expect("Failed to initialize inotify");

    inotify
        .watches()
        .add(&data_dir, WatchMask::CLOSE_WRITE)
        .expect("Failed to add inotify watch");

    let mut buffer = [0u8; 4096];
    let keys = get_keys(&config_dir);
    let client = get_client(&config_dir, keys).await?;
    let data_dir_content = data_dir.join("content.txt");
    let data_dir_hidden_content_current = data_dir.join(".content-current.txt");
    let data_dir_hidden_content_before = data_dir.join(".content-before.txt");
    let data_dir_hidden_content_before_str = data_dir_hidden_content_before.to_str().unwrap();
    let data_dir_hidden_content_current_str = data_dir_hidden_content_current.to_str().unwrap();
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
            } else {
                continue;
            }
            if !event.mask.contains(EventMask::CLOSE_WRITE) {
                continue;
            }

            copy(&data_dir_content, &data_dir_hidden_content_current)?;

            let binding = read_to_string(&data_dir_hidden_content_current).unwrap();
            let content = binding.as_str().trim();
            if content == "" {
                log::info!("Empty!");
                continue;
            }
            if diff(
                data_dir_hidden_content_before_str,
                data_dir_hidden_content_current_str,
            ) {
                log::info!("Same!");
                continue;
            }
            log::info!("--content begin--\n{}\n--content end--", content);

            let event: Event = EventBuilder::new_text_note(content, &[]).to_event(&keys)?;
            log::info!("Event id: {}", event.id);

            if let Err(_) = tokio::time::timeout(std::time::Duration::from_secs(2), async {
                if let Err(nostr_error) = client.send_event(event).await {
                    log::error!("client.send_event Error!\n{}", nostr_error);
                }
            })
            .await
            {
                log::debug!("Timeout!");
            }
            log::debug!("After publish");

            copy(
                &data_dir_hidden_content_current,
                &data_dir_hidden_content_before,
            )?;
            log::debug!("After copy cur -> bef");
        }
    }
}
