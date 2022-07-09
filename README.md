──▄█▀█▄─────────██
▄████████▄───▄▀█▄▄▄▄
██▀▼▼▼▼▼─▄▀──█▄▄
█████▄▲▲▲─▄▄▄▀───▀▄
██████▀▀▀▀─▀────────▀▀

Welcome to Ajax!

Ajax is a text based adventure game to leverages the power of the CLI and the Rust stark-rs library to create a simple interface for interacting with Cairo smart contracts. 

Users upload their wallet and key and then create a character.  This character is deployed as a Cairo smart contract.  Individual decisions players make affect the state of the player and the smart contract itself.  When the player encounters an enemy a new enemy contract is deployed.  

When they enter battle, the player users the battle.cairo contract to interact with the enemy contract.

Bugs
1) When a new contract is generated the player needs to wait between 1-2 minutes for the contract to be verified on testnet.  For practical purposes players should pause for 1-2 minutes after starting the game before taking any action. This can be fixed by getting the contract state from Starknet and not letting players take action until the contract is fully deployed.  Need to figure out how best to impliment. 
