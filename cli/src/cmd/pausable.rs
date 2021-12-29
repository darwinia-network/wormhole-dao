use crate::bindings::pausable::Pausable as PausableContract;
use ethers::prelude::*;
use std::{convert::TryFrom, str::FromStr, sync::Arc};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Pausable related commands")]
pub enum Pausable {
    #[structopt(name = "Show paused status")]
    Paused {
        addr: Address,
    },
    Pause,
    Unpause,
}

impl Pausable {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Pausable::Paused { addr } => {
                let pausable = init_pausable_call(addr).await?;
                let paused = pausable.paused().call().await?;
                println!("{}", paused);
            }
            Pausable::Pause => {
                let to = Address::from_str("0xc8C1680B18D432732D07c044669915726fAF67D0")?;
                let pausable = init_pausable_call(to).await?;
                let calldata = pausable.pause().calldata().unwrap();
                println!("{}", calldata);
            }
            Pausable::Unpause => {
                let to = Address::from_str("0xc8C1680B18D432732D07c044669915726fAF67D0")?;
                let pausable = init_pausable_call(to).await?;
                let calldata = pausable.unpause().calldata().unwrap();
                println!("{}", calldata);
            }
        }
        Ok(())
    }
}

pub async fn init_pausable_call(
    to: Address,
) -> eyre::Result<PausableContract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>>
{
    // let provider = Provider::<Http>::try_from("https://crab-rpc.darwinia.network")?;
    let provider = Provider::<Http>::try_from("https://pangolin-rpc.darwinia.network")?;
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let key = "380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc"
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    // let to = Address::from_str("0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2")?;
    let client = SignerMiddleware::new(provider, key);
    let client = Arc::new(client);
    let pausable = PausableContract::new(to, client);
    Ok(pausable)
}
