use crate::bindings::multi_sig_wallet::MultiSigWallet as MultiSigWalletContract;
use structopt::StructOpt;

use ethers::prelude::*;
use std::{convert::TryFrom, str::FromStr, sync::Arc};

#[derive(StructOpt)]
#[structopt(about = "MultiSigWallet related commands.")]
pub enum MultiSigWallet {
    #[structopt(about = "Get Owners.")]
    Owners,
    #[structopt(name = "tx")]
    Transaction(Tx),
}

#[derive(StructOpt)]
#[structopt(about = "MultiSigWallet transaction related commands.")]
pub enum Tx {
    #[structopt(about = "Transaction list.")]
    List {
        #[structopt(default_value = "0")]
        from: U256,

        to: Option<U256>,

        #[structopt(long)]
        no_pending: bool,
        #[structopt(long)]
        no_executed: bool,
    },
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
            Self::Transaction(_sub) => _sub.run().await?,
        }
        Ok(())
    }
}

impl Tx {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Self::List {
                from,
                to,
                no_pending,
                no_executed,
            } => {
                dbg!(&from, &to, &no_pending, &no_executed);
                let provider = Provider::<Http>::try_from("https://crab-rpc.darwinia.network")?;
                let wallet: LocalWallet =
                    "380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc".parse()?;
                let addr = Address::from_str("0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2")?;
                let client = SignerMiddleware::new(provider, wallet);
                let client = Arc::new(client);
                let multi_sig_wallet = MultiSigWalletContract::new(addr, client);
                let tx_count = if let Some(_to) = to {
                    _to
                } else {
                    multi_sig_wallet.transaction_count().call().await?
                };
                let ids = multi_sig_wallet
                    .get_transaction_ids(from, tx_count, !no_pending, !no_executed)
                    .call()
                    .await?;
                for id in ids {
                    // multicall
                    let tx = multi_sig_wallet.transactions(id).call().await?;
                    println!("========================================================");
                    println!("id          : {}", id);
                    println!("destination : {:?}", tx.0);
                    println!("value       : {}", tx.1);
                    println!("data        : {}", tx.2);
                    println!("executed    : {}", tx.3);
                }
            }
        }
        Ok(())
    }
}
