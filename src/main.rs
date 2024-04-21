use std::{
    process::exit,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use discord_rich_presence::{
    activity::{self, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

const SHELLS: [&str; 3] = ["bash", "zsh", "fish"];

struct Terminal {
    state: String,
    large_image: &'static str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DiscordIpcClient::new("1231559577607344138")?;
    client.connect()?;

    loop {
        let time = Timestamps::new().start(current_time());
        match get_data() {
            Ok(terminal) => {
                let activity = activity::Activity::new()
                    .timestamps(time)
                    .state(&terminal.state)
                    .assets(Assets::new().large_image(terminal.large_image));
                client.set_activity(activity)?;
                thread::sleep(Duration::from_secs(30))
            }
            Err(_) => {
                eprintln!("Couldn't retrieve terminal data");
                exit(1);
            }
        }
    }
}

fn get_data() -> Result<Terminal, ()> {
    let env_shell = env!("SHELL");
    for shell in SHELLS {
        if env_shell == format!("/bin/{shell}") {
            let state = format!("Using {}", shell);
            return Ok(Terminal {
                state,
                large_image: get_image(shell)
                    .expect("Couldn't retrieve image, maybe you are offline?"),
            });
        }
    }
    Err(())
}

fn get_image(shell: &str) -> Option<&str> {
    match shell {
        "bash" => Some("https://raw.githubusercontent.com/anotherlusitano/terminal-discord-rpc/main/icons/bash.png"),
        "fish" => Some("https://raw.githubusercontent.com/anotherlusitano/terminal-discord-rpc/main/icons/fish.png"),
        "zsh" => Some("https://raw.githubusercontent.com/anotherlusitano/terminal-discord-rpc/main/icons/zsh.png"),
        _ => None
    }
}

fn current_time() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to obtain current time")
        .as_secs() as i64
}
