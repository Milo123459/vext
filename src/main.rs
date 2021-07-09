mod cli;

fn main() -> anyhow::Result<()> {
	cli::run::run()?;
	Ok(())
}
