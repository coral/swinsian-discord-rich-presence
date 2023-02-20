mod error;
mod swinsian;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

enum AppState {
    Active,
    Cleared,
}

#[allow(unreachable_code)]
fn main() -> Result<(), error::SwinsianError> {
    println!("starting");

    let mut client = DiscordIpcClient::new("1076384656850698240")?;
    client.connect()?;

    loop {
        if client.connect().is_ok() {
            break;
        }
        sleep(Duration::from_secs(30));
        continue;
    }

    let mut appstate = AppState::Active;
    let mut last_updated = Instant::now();

    loop {
        let data = swinsian::get()?;

        match data.state {
            swinsian::State::Playing => {
                appstate = AppState::Active;
                update_presence(data, &mut client, &mut last_updated)
            }
            _ => clear(&mut client, &mut last_updated, &mut appstate),
        }?;

        sleep(Duration::from_secs(5));
    }

    Ok(())
}

fn update_presence(
    data: swinsian::Swinsian,
    client: &mut impl DiscordIpc,
    last_updated: &mut Instant,
) -> Result<(), error::SwinsianError> {
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

    if Instant::now().duration_since(*last_updated).as_secs() >= 4 {
        if client.set_activity(payload).is_err() {
            client.reconnect().ok();
        } else {
            *last_updated = Instant::now();
        }
    }

    Ok(())
}

fn clear(
    client: &mut impl DiscordIpc,
    last_updated: &mut Instant,
    appstate: &mut AppState,
) -> Result<(), error::SwinsianError> {
    match appstate {
        AppState::Active => {
            if Instant::now().duration_since(*last_updated).as_secs() >= 4 {
                if client.clear_activity().is_err() {
                    client.reconnect().ok();
                } else {
                    *last_updated = Instant::now();
                    *appstate = AppState::Cleared
                }
            }
        }
        _ => {}
    }

    Ok(())
}
