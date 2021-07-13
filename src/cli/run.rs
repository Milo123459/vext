use crate::cli::args;
use crate::editor::tab_manager;
pub use anyhow::Context;
pub use structopt::StructOpt;

pub fn run(tab: &mut tab_manager::TabManager) -> anyhow::Result<()> {
	let args = args::Arguments::from_args();
	let readonly = args.readonly();
	for arg in args.arguments {
		tab.new_tab(tab_manager::Tab {
			readonly,
			active: tab.amount_of_tabs() == 0,
			tab_type: tab_manager::TabType::File,
			title: "Woo!".to_owned(),
			path: Some(arg),
			added_content: None,
		})
	}
	println!("{:?}", &tab.tabs);
	Ok(())
}
