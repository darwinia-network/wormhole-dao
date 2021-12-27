mod multi_sig_wallet;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Dao utilities")]
pub enum Command {
    #[structopt(name = "wallet")]
    MultiSigWallet(multi_sig_wallet::MultiSigWallet),
}

impl Command {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Self::MultiSigWallet(cmd) => cmd.run().await?,
        }
        Ok(())
    }
}

pub fn parse_args() -> Command {
    Command::from_args()
}
