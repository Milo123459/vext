use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq, Clone)]
pub struct Arguments {
	/// Files / folders to open
	pub arguments: Vec<String>,

	/// Readonly mode
	#[structopt(long, short)]
	pub(crate) readonly: Option<Option<bool>>,
}

// For the usage of --readonly
impl Arguments {
	pub fn readonly(&self) -> bool {
		match self.readonly {
			None => false,
			Some(None) => true,
			Some(Some(a)) => a,
		}
	}
}
