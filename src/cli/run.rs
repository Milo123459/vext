use crate::cli::args;
pub use anyhow::Context;
pub use structopt::StructOpt;

pub fn run() -> anyhow::Result<()> {
	let args = args::Arguments::from_args();
	let readonly = args.readonly();
	println!("Tabs: {} Readonly: {}", args.arguments.len(), readonly);
	Ok(())
}
