use eyre::{Result, ErrReport};
use starknet::{
        providers::{SequencerGatewayProvider},
        core::{chain_id, types::FieldElement},
        signers::{LocalWallet, SigningKey},
};
use dialoguer::{Input};

pub struct Env {
    pub address: FieldElement,
    pub signer: LocalWallet,
    pub provider: SequencerGatewayProvider,
    pub chain_id: FieldElement,
}

impl Env {
    pub fn new() -> Result<Env, ErrReport> {
        let provider = SequencerGatewayProvider::starknet_alpha_goerli();
        let signer = get_signer();
        let address = get_address();
        let chain_id = chain_id::TESTNET;

        Ok(Env {
            address,
            signer,
            provider,
            chain_id,
        })
    }
}

pub fn get_signer() -> LocalWallet {
    let mnemonic = Input::<String>::new()
        .with_prompt("Enter your key")
        .interact()
        .unwrap();
    let signer = LocalWallet::from(SigningKey::from_secret_scalar(
        FieldElement::from_hex_be(&mnemonic).unwrap(),
    ));
    signer
}

pub fn get_address() -> FieldElement {
    let address = Input::<String>::new()
        .with_prompt("Enter your address")
        .interact()
        .unwrap();
    let address = FieldElement::from_hex_be(&address).unwrap();
    address
}
