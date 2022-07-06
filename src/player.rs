use starknet::{
    contract::ContractFactory,
    core::types::{ContractArtifact, FieldElement},
    providers::SequencerGatewayProvider,
};
use serde_json::{from_reader, Result};

pub enum Class {
    Warrior,
    Mage,
    Rogue,
    Priest,
    Warlock,
    Shaman,
    Paladin,
    Hunter,
    Druid,
}

pub struct Player {
    pub name: String,
    pub class: Class,
    pub health: u32,
    pub damage: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            name: "Ajax".to_string(),
            class: Class::Warrior,
            health: 100,
            damage: 10,
        }
    }

    pub fn deploy_player(&self) -> Player {
        let contract_artifact: ContractArtifact =
            serde_json::from_reader(std::fs::File::open("../ajax/contracts/main.json").unwrap())
                .unwrap();

        let provider = SequencerGatewayProvider::starknet_alpha_goerli();

        let contract_factory = ContractFactory::new(contract_artifact, provider)
            .unwrap()
            .deploy(vec![FieldElement::from_dec_str("123456").unwrap()], None)
            .expect("Unable to deploy contract");


    }


}

