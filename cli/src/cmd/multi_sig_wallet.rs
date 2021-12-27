use crate::bindings::multi_sig_wallet::MultiSigWallet as MultiSigWalletContract;
use structopt::StructOpt;

use ethers::prelude::*;
use std::{convert::TryFrom, str::FromStr, sync::Arc};

#[derive(StructOpt)]
#[structopt(about = "MultiSigWallet related commands.")]
pub enum MultiSigWallet {
    #[structopt(about = "Get Owners")]
    Owners,
}

impl MultiSigWallet {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Self::Owners => {
                let provider = Provider::<Http>::try_from("https://crab-rpc.darwinia.network")?;
                let wallet: LocalWallet =
                    "380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc".parse()?;
                let to = Address::from_str("0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2")?;
                let client = SignerMiddleware::new(provider, wallet);
                let client = Arc::new(client);
                let multi_sig_wallet = MultiSigWalletContract::new(to, client);
                let owners = multi_sig_wallet.get_owners().call().await?;
                println!("{:?}", owners);
            }
        }
        Ok(())
    }
}
