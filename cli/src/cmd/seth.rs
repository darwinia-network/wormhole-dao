use cast::SimpleCast;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Perform Ethereum RPC calls from the comfort of your command line.")]
pub enum Cast {
    Calldata {
        #[structopt(
            help = r#"When called with <sig> of the form <name>(<types>...), then perform ABI encoding to produce the hexadecimal calldata.
        If the value given—containing at least one slash character—then treat it as a file name to read, and proceed as if the contents were passed as hexadecimal data.
        Given data, ensure it is hexadecimal calldata starting with 0x and normalize it to lowercase.
        "#
        )]
        sig: String,
        args: Vec<String>,
    },
}

impl Cast {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Cast::Calldata { sig, args } => {
                println!("{}", SimpleCast::calldata(sig, &args)?);
            }
        }
        Ok(())
    }
}
