use crate::bindings::multi_sig_wallet::MultiSigWallet as MultiSigWalletContract;
use crate::cmd::utils::Bytes;
use ethers::prelude::*;
use std::{convert::TryFrom, str::FromStr, sync::Arc};
use structopt::StructOpt;

use super::EthereumOpts;

#[derive(StructOpt)]
#[structopt(about = "MultiSigWallet related commands.")]
pub enum MultiSigWallet {
    #[structopt(name = "owner")]
    Owners(Owner),
    #[structopt(about = "Required confirmations")]
    #[structopt(name = "threshold")]
    Threshold,
    #[structopt(about = "Set required confirmations")]
    #[structopt(name = "set-threshold")]
    SetThreshold {
        #[structopt(about = "New threshold required")]
        required: U256,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(name = "tx")]
    Transactions(Tx),
}

#[derive(StructOpt)]
#[structopt(about = "Owners management related commands.")]
pub enum Owner {
    #[structopt(about = "Owner list.")]
    List,
    #[structopt(about = "Add new owner")]
    Add {
        new_owner: Address,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Remove an owner")]
    Remove {
        rm_owner: Address,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Replace an owner with a new owner")]
    Replace {
        old_owner: Address,
        new_owner: Address,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
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
    #[structopt(about = "Submit and confirm a transaction.")]
    Submit {
        destination: Address,
        value: U256,
        data: Bytes,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Confirm a transaction.")]
    Confirm {
        #[structopt(about = "Transaction ID.")]
        tx_id: U256,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Execute a confirmed transaction.")]
    Execute {
        #[structopt(about = "Transaction ID.")]
        tx_id: U256,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Revoke a confirmation for a transaction.")]
    Revoke {
        #[structopt(about = "Transaction ID.")]
        tx_id: U256,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
}

impl MultiSigWallet {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Self::Owners(_owner) => _owner.run().await?,
            Self::Transactions(_sub) => _sub.run().await?,
            Self::Threshold => {
                let multi_sig_wallet = init_wallet_call().await?;
                let threshold = multi_sig_wallet.required().call().await?;
                println!("{}", threshold);
            }
            Self::SetThreshold { required, eth } => {
                let multi_sig_wallet = init_wallet_send(eth.private_key).await?;
                let call = multi_sig_wallet.change_requirement(required).gas(100_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
        }
        Ok(())
    }
}

impl Owner {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Owner::List => {
                let multi_sig_wallet = init_wallet_call().await?;
                let owners = multi_sig_wallet.get_owners().call().await?;
                println!("{:?}", owners);
            }
            Owner::Add { new_owner, eth } => {
                let multi_sig_wallet = init_wallet_send(eth.private_key).await?;
                let call = multi_sig_wallet.add_owner(new_owner).gas(100_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Owner::Remove { rm_owner, eth } => {
                let multi_sig_wallet = init_wallet_send(eth.private_key).await?;
                let call = multi_sig_wallet.remove_owner(rm_owner).gas(100_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Owner::Replace {
                old_owner,
                new_owner,
                eth,
            } => {
                let multi_sig_wallet = init_wallet_send(eth.private_key).await?;
                let call = multi_sig_wallet
                    .replace_owner(old_owner, new_owner)
                    .gas(100_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
        }
        Ok(())
    }
}

impl Tx {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Tx::List {
                from,
                to,
                no_pending,
                no_executed,
            } => {
                let multi_sig_wallet = init_wallet_call().await?;
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
                    if !no_executed && tx.3 || !no_pending && !tx.3 {
                        println!("========================================================");
                        println!("id          : {}", id);
                        println!("destination : {:?}", tx.0);
                        println!("value       : {}", tx.1);
                        println!("data        : {}", tx.2);
                        println!("executed    : {}", tx.3);
                    }
                }
            }
            Tx::Submit {
                destination,
                value,
                data,
                eth,
            } => {
                let multi_sig_wallet = init_wallet_send(eth.private_key).await?;
                let calldata = ethers::prelude::Bytes::from(data.0);
                let call = multi_sig_wallet
                    .submit_transaction(destination, value, calldata)
                    .gas(200_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Tx::Confirm { tx_id, eth } => {
                let multi_sig_wallet = init_wallet_send(eth.private_key).await?;
                let call = multi_sig_wallet.confirm_transaction(tx_id).gas(200_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Tx::Execute { tx_id, eth } => {
                let multi_sig_wallet = init_wallet_send(eth.private_key).await?;
                let call = multi_sig_wallet.execute_transaction(tx_id).gas(200_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Tx::Revoke { tx_id, eth } => {
                let multi_sig_wallet = init_wallet_send(eth.private_key).await?;
                let call = multi_sig_wallet.revoke_confirmation(tx_id).gas(100_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
        }
        Ok(())
    }
}

pub async fn init_wallet_call() -> eyre::Result<
    MultiSigWalletContract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
> {
    Ok(init_wallet_send(
        "380eb0f3d505f087e438eca80bc4df9a7faa24f868e69fc0440261a0fc0567dc".to_string(),
    )
    .await?)
}

pub async fn init_wallet_send(
    private_key: String,
) -> eyre::Result<
    MultiSigWalletContract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
> {
    // let provider = Provider::<Http>::try_from("https://crab-rpc.darwinia.network")?;
    let provider = Provider::<Http>::try_from("https://pangolin-rpc.darwinia.network")?;
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let key = private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    // let to = Address::from_str("0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2")?;
    let to = Address::from_str("0xc8C1680B18D432732D07c044669915726fAF67D0")?;
    let client = SignerMiddleware::new(provider, key);
    let client = Arc::new(client);
    let multi_sig_wallet = MultiSigWalletContract::new(to, client);
    Ok(multi_sig_wallet)
}
