use super::EthereumOpts;
use crate::cmd::multi_sig_wallet;
use crate::cmd::time_lock;
use crate::cmd::utils::Bytes;
use ethers::prelude::*;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Proposal related commands")]
pub enum Proposal {
    #[structopt(about = "Proposal list.")]
    List {
        #[structopt(default_value = "8317180")]
        #[structopt(long, short)]
        from_block: String,
        #[structopt(long, short)]
        to_block: Option<String>,
        #[structopt(long)]
        latest: Option<String>,
        #[structopt(long)]
        no_done: bool,
        #[structopt(long)]
        no_ready: bool,
        #[structopt(long)]
        no_pending: bool,
        #[structopt(long)]
        no_cancel: bool,
    },
    #[structopt(about = "Schedule an proposal containing a single transaction.")]
    Schedule {
        #[structopt(
            about = "The address of the smart contract that the timelock should operate on."
        )]
        target: Address,
        #[structopt(
            about = "In wei, that should be sent with the transaction. Most of the time this will be 0."
        )]
        value: String,
        #[structopt(
            about = "Containing the encoded function selector and parameters of the call by abi.encode."
        )]
        data: Bytes,
        #[structopt(about = "That specifies a dependency between operations.")]
        predecessor: H256,
        #[structopt(
            about = "Used to disambiguate two otherwise identical proposals. This can be any random value."
        )]
        salt: H256,
        #[structopt(about = "Delay time to execute the proposal, should be larger than minDelay")]
        delay: String,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Cancel an proposal.")]
    Cancel {
        #[structopt(about = "Proposal ID")]
        id: H256,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Execute an (ready) proposal containing a single transaction.")]
    Execute {
        #[structopt(
            about = "The address of the smart contract that the timelock should operate on."
        )]
        target: Address,
        #[structopt(
            about = "In wei, that should be sent with the transaction. Most of the time this will be 0."
        )]
        value: String,
        #[structopt(
            about = "Containing the encoded function selector and parameters of the call by abi.encode."
        )]
        data: Bytes,
        #[structopt(about = "That specifies a dependency between operations.")]
        predecessor: H256,
        #[structopt(
            about = "Used to disambiguate two otherwise identical proposals. This can be any random value."
        )]
        salt: H256,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
}

impl Proposal {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Proposal::List {
                from_block,
                to_block,
                latest,
                no_done,
                no_ready,
                no_pending,
                no_cancel,
            } => {
                time_lock::load_proposals(
                    from_block, to_block, latest, no_done, no_ready, no_pending, no_cancel,
                )
                .await?
            }
            Proposal::Schedule {
                target,
                value,
                data,
                predecessor,
                salt,
                delay,
                eth,
            } => {
                let time_lock = time_lock::init_timelock_call().await?;
                let calldata = ethers::prelude::Bytes::from(data.0);
                let _value = U256::from_str_radix(&value, 10)?;
                let _delay = U256::from_str_radix(&delay, 10)?;
                let payload = time_lock
                    .schedule(
                        target,
                        _value,
                        calldata,
                        *predecessor.as_fixed_bytes(),
                        *salt.as_fixed_bytes(),
                        _delay,
                    )
                    .calldata()
                    .unwrap();

                let multi_sig_wallet = multi_sig_wallet::init_wallet_send(eth.private_key).await?;
                let destination = Address::from_str("0x2401224012bAE7C2f217392665CA7abC16dCDE1e")?;
                let call =
                    multi_sig_wallet.submit_transaction(destination, U256::default(), payload);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Proposal::Cancel { id, eth } => {
                let time_lock = time_lock::init_timelock_call().await?;
                let calldata = time_lock.cancel(*id.as_fixed_bytes()).calldata().unwrap();

                let multi_sig_wallet = multi_sig_wallet::init_wallet_send(eth.private_key).await?;
                let destination = Address::from_str("0x2401224012bAE7C2f217392665CA7abC16dCDE1e")?;
                let call =
                    multi_sig_wallet.submit_transaction(destination, U256::default(), calldata);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Proposal::Execute {
                target,
                value,
                data,
                predecessor,
                salt,
                eth,
            } => {
                let time_lock = time_lock::init_timelock_send(eth.private_key).await?;
                let calldata = ethers::prelude::Bytes::from(data.0);
                let _value = U256::from_str_radix(&value, 10)?;
                let call = time_lock
                    .execute(
                        target,
                        _value,
                        calldata,
                        *predecessor.as_fixed_bytes(),
                        *salt.as_fixed_bytes(),
                    )
                    .gas(500_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
        }
        Ok(())
    }
}
