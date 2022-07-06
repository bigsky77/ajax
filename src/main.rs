mod env;
mod player;

use color_eyre::Report;
use eyre::ErrReport;
use structopt::StructOpt;
use dialoguer::{Input, MultiSelect, theme::ColorfulTheme};

#[derive(StructOpt)]
pub struct Cmd {
    cmd: String,
}

#[tokio::main]
async fn main() -> Result<(), ErrReport> {
    set_up()?;

    let env = env::Env::new()?;
    let player = player::Player::new();

    game(&env, &player).await?;

    println!("Game Over!");

    Ok(())
}

pub fn set_up() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    Ok(())
}

pub async fn game(env: &env::Env, player: &player::Player) -> Result<(), ErrReport> {
    println!("Hello, {} welcome to the most exciting game of your life!", player.name);

    println!("You are a standing on a vast desert, with two suns in the sky");

    let action = &[
        "walk",
        "meditate",
        "summon a god",
    ];

    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&action[..])
        .interact()
        .unwrap();

    Ok(())
}




