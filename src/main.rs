mod error;
mod swinsian;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
fn main() -> Result<(), error::SwinsianError> {
    println!("starting");
    //let req = swinsian::get();

    let mut client = DiscordIpcClient::new("1076384656850698240")?;
    client.connect()?;

    loop {
        if client.connect().is_ok() {
            break;
        }
        sleep(Duration::from_secs(1));
        continue;
    }

    // loop {
    //     let payload = activity::Activity::new().state("Hello world!");
    //     client.set_activity(payload)?;
    //     sleep(Duration::from_secs(5));
    // }

    let mut has_closed = false;
    let mut last_updated = Instant::now();
    loop {
        if Instant::now().duration_since(last_updated).as_secs() >= 1 {
            if update_presence(&mut client, &mut last_updated).is_ok() {
                has_closed = false;
                continue;
            }
        }
        sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn update_presence(
    client: &mut impl DiscordIpc,
    last_updated: &mut Instant,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = swinsian::get()?;

    let state: String = format!("{} - {}", data.artist, data.album)
        .chars()
        .take(128)
        .collect();
    let details: String = data.song.chars().take(128).collect();
    let large_text: String = format!("Listening with Swinsian");
    let assets = activity::Assets::new()
        .large_text(large_text.as_str())
        .large_image("sw2")
        .small_text("Playing");
    let mut payload = activity::Activity::new()
        .state(&state)
        .details(&details)
        .assets(assets.clone());

    if Instant::now().duration_since(*last_updated).as_secs() >= 5 {
        if client.set_activity(payload).is_err() {
            client.reconnect().ok();
        } else {
            *last_updated = Instant::now();
        }
    }

    Ok(())
}
