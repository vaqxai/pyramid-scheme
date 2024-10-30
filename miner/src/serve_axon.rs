use neuron::{AxonProtocol, config, hotkey_location, load_key_seed, signer_from_seed, Subtensor};
use std::net::IpAddr;

use clap::Parser;

mod miner_config;

#[derive(Parser)]
struct Cli {
    ip: IpAddr,
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let args: Cli = Cli::parse();

    let hotkey_location = hotkey_location(
        config::WALLET_PATH.clone(),
        &*config::WALLET_NAME,
        &*config::HOTKEY_NAME,
    );

    let seed = load_key_seed(&hotkey_location).unwrap();

    let signer = signer_from_seed(&seed).unwrap();

    let subtensor = Subtensor::new(&*config::CHAIN_ENDPOINT).await.unwrap();

    subtensor
        .serve_axon(
            &signer,
            *config::NETUID,
            args.ip,
            args.port.unwrap_or_else(|| *miner_config::PORT),
            AxonProtocol::Tcp,
        )
        .await
        .unwrap();
}
