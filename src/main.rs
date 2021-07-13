mod cli;
mod editor;

fn main() -> anyhow::Result<()> {
	let mut tab_manager = editor::tab_manager::TabManager {
		tabs: vec![],
		current_tab: None,
	};
	cli::run::run(&mut tab_manager)?;
	Ok(())
}
