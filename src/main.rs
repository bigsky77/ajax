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
use starknet::providers::Provider;
use starknet::signers::LocalWallet;


#[derive(StructOpt)]
pub struct Cmd {
    cmd: String,
}

#[tokio::main]
async fn main() -> Result<(), ErrReport> {
    set_up()?;
    
    let contract_artifact: ContractArtifact =
        serde_json::from_reader(std::fs::File::open("contracts/src/hero_compiled.json").unwrap())
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
            println!("You walk forward into the desert");

            let enemy_contract = enemy().await;

            start_battle(hero_contract, enemy_contract,  account).await?;
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
            let result = account 
                .execute(&[Call {
                    to: hero_contract.address.expect("unable to get address"),
                    selector: get_selector_from_name("set_damage").unwrap(),
                    calldata: vec![FieldElement::from_dec_str("20").unwrap()],
                }])
                .send()
                .await
                .expect("unable to send transaction");

            dbg!(result);
            println!("You summon a the god of STARKNET and he increases your power!");
        } 
        _ => {
            println!("You do nothing");
        }
    }
    Ok(())
}

pub async fn enemy() -> AddTransactionResult {
    println!("A giant Phoenix flying low over the desert spots you!");
    
     let contract_artifact: ContractArtifact =
        serde_json::from_reader(std::fs::File::open("contracts/src/enemy_compiled.json").unwrap())
            .unwrap();

    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let contract_factory = ContractFactory::new(contract_artifact, provider).unwrap();

    let enemy_contract = contract_factory
        .deploy(vec![FieldElement::from_dec_str("10").unwrap()],None)
        .await
        .expect("cannot deploy contract"); 

    println!("enemy information {:?}", enemy_contract.address);

    enemy_contract
}

pub async fn start_battle(hero: AddTransactionResult, enemy: AddTransactionResult, account: SingleOwnerAccount<SequencerGatewayProvider, LocalWallet> ) -> Result<(), ErrReport> {
    println!("the giant Phoenix sweeps down towards you!  You draw your spear and ready yourself for battle!");

    let contract_artifact: ContractArtifact =
        serde_json::from_reader(std::fs::File::open("contracts/src/battle_compiled.json").unwrap())
            .unwrap();

    let provider = SequencerGatewayProvider::starknet_alpha_goerli();
    let contract_factory = ContractFactory::new(contract_artifact, provider).unwrap();
    
    let battle_contract = contract_factory 
        .deploy(vec![FieldElement::from_dec_str("").unwrap()], None)
        .await
        .expect("cannot deploy contract");

     let actions = &[
        "throw spear!",
        "raise your sheild and defend!",
        "summon a god!",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&actions[..])
        .interact()
        .unwrap();

    match actions[selection] {
        "throw a spear!" => {
            let result = account 
                .execute(&[Call {
                    to:battle_contract.address.expect("unable to get address"),
                    selector: get_selector_from_name("take_damage").unwrap(),
                    calldata: vec![(enemy.address).unwrap()],
                }])
                .send()
                .await
                .expect("unable to send transaction");

            dbg!(result);
            println!("You throw a spear and it jabs into the Phoenix's side!");
        }
        "raise your sheild and defend!" => {
            let result = account
                .execute(&[Call {
                    to:battle_contract.address.expect("unable to get address"),
                    selector: get_selector_from_name("take_damage").unwrap(),
                    calldata: vec![(enemy.address).unwrap()],
                }])
                .send()
                .await
                .expect("unable to send transaction");

            dbg!(result);
            println!("You raise your shield and defend!");
        }
        "summon a god!" => {
            let result = account
                .execute(&[Call {
                    to:battle_contract.address.expect("unable to get address"),
                    selector: get_selector_from_name("take_damage").unwrap(),
                    calldata: vec![(enemy.address).unwrap()],
                }])
                .send()
                .await
                .expect("unable to send transaction");

            dbg!(result);
            println!("You summon a the god of STARKNET and he throws lightning at the phoenix!");
        }
        _ => {
            println!("You do nothing");
        }
    }         

    Ok(())
}
/*
pub async fn check_health(hero: AddTransactionResult, enemy: AddTransactionResult, account: SingleOwnerAccount<SequencerGatewayProvider, LocalWallet>) -> Result<(), ErrReport> {
    let hero_health = account.provider().call_contract(hero.address.unwrap()).await?;

    dbg!(hero_health);
    Ok(())
}*/


