mod env;
mod player;

use color_eyre::Report;
use eyre::ErrReport;
use structopt::StructOpt;
use dialoguer::{Input, Select, MultiSelect, theme::ColorfulTheme};
use starknet::accounts::{Account, Call, SingleOwnerAccount};
use starknet::core::{chain_id, types::FieldElement, utils::get_selector_from_name};

#[derive(StructOpt)]
pub struct Cmd {
    cmd: String,
}

#[tokio::main]
async fn main() -> Result<(), ErrReport> {
    set_up()?;

    let env = env::Env::new()?;
    let player = player::Player::new();
        
    game(env, player).await?;

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

pub async fn game(env: env::Env, player: player::Player) -> Result<(), ErrReport> {
    let addr = Some(player::deploy_player(&player).await?).unwrap();
    
    println!("{:?}", addr.address);

    println!("Hello, {} welcome to the most exciting game of your life!", player.name);

    println!("You are a standing on a vast desert, with two suns in the sky");

    let actions = &[
        "walk",
        "meditate",
        "summon a god",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&actions[..])
        .interact()
        .unwrap();

    match actions[selection] {
        "walk" => {
            println!("You walk forward");
        }
        "meditate" => {
           let hero_contract = FieldElement::from_hex_be(
               "0x00fda344b6df51e0082a44257b6236b1bacdbb3b7dcbc361b05e6d699e5fa610")
               .unwrap();

            let account = SingleOwnerAccount::new(
                env.provider,
                env.signer,
                env.address,
                chain_id::TESTNET,
            );

            let result = account
                .execute(&[Call {
                    to: hero_contract,
                    selector: get_selector_from_name("set_health").unwrap(),
                    calldata: vec![FieldElement::from_dec_str("100").unwrap()],
                }])
                .send()
                .await
                .unwrap();

            dbg!(result);

            println!("You meditate and grow stronger!");
        }
        "summon a god" => {
            println!("You summon a god");
        }
        _ => {
            println!("You do nothing");
        }
    }

    Ok(())
}




