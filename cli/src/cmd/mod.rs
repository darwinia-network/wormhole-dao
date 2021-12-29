mod multi_sig_wallet;
mod time_lock;
mod utils;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct EthereumOpts {
    #[structopt(long = "private-key", help = "Your private key string")]
    private_key: String,
}

#[derive(StructOpt)]
#[structopt(about = "Dao utilities")]
pub enum Command {
    #[structopt(name = "wallet")]
    MultiSigWallet(multi_sig_wallet::MultiSigWallet),
    #[structopt(name = "timelock")]
    TimeLock(time_lock::TimeLock),
}

impl Command {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Command::MultiSigWallet(cmd) => cmd.run().await?,
            Command::TimeLock(cmd) => cmd.run().await?,
        }
        Ok(())
    }
}

pub fn parse_args() -> Command {
    Command::from_args()
}
