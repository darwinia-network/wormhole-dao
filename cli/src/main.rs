mod bindings;
mod cmd;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let command = cmd::parse_args();
    command.run().await?;
    Ok(())
}
