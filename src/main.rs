mod env;
mod player;

use color_eyre::Report;
use eyre::ErrReport;
use structopt::StructOpt;
use dialoguer::{Input, Select, MultiSelect, theme::ColorfulTheme};

use starknet::{
    contract::ContractFactory,
    core::{chain_id, types::{ContractArtifact, FieldElement, AddTransactionResult}, utils::get_selector_from_name},
    providers::SequencerGatewayProvider,
    accounts::{Account, Call, SingleOwnerAccount},
};


#[derive(StructOpt)]
pub struct Cmd {
    cmd: String,
}

#[tokio::main]
async fn main() -> Result<(), ErrReport> {
    set_up()?;
    
     let contract_artifact: ContractArtifact =
        serde_json::from_reader(std::fs::File::open("./contracts/src/hero_compiled.json").unwrap())
            .unwrap();

    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let contract_factory = ContractFactory::new(contract_artifact, provider).unwrap();

    let hero_contract = contract_factory
        .deploy(vec![FieldElement::from_dec_str("1").unwrap()],None)
        .await
        .expect("cannot deploy contract"); 

    let env = env::Env::new()?;
    let player = player::Player::new();

    game(env, &player, hero_contract).await?;

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

pub async fn game(env: env::Env, player: &player::Player, hero_contract: AddTransactionResult) -> Result<(), ErrReport> {

    println!("contract address: {:?}", Some(hero_contract.address));
    println!("contract hash: {:?} ", Some(hero_contract.transaction_hash));

    let account = SingleOwnerAccount::new(
        env.provider,
        env.signer,
        env.address,
        chain_id::TESTNET,
    );

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
            let result = account
                .execute(&[Call {
                    to: hero_contract.address.expect("Unable to get address"),
                    selector: get_selector_from_name("set_health").unwrap(),
                    calldata: vec![FieldElement::from_dec_str("100").unwrap()],
                }])
                .send()
                .await
                .expect("Unable to send transaction");

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




