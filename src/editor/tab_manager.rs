#[derive(Debug)]
pub enum TabType {
	File,
	Folder,
	Settings,
	Error,
}

#[derive(Debug)]
pub struct Tab {
	/// The type of the tab.
	pub tab_type: TabType,
	/// The path of the tab, if the tab has the type `File` or `Folder`.
	pub path: Option<String>,
	/// Title of the tab. Defaults to `Untitled`, but files and the user can set custom titles for tabs.
	pub title: String,
	/// Is the tab readonly. Defaults to `false`.
	pub readonly: bool,
	/// Added content that can be accessed by tabs, usually used for the `Error` variant.
	pub added_content: Option<String>,
	/// Is the tab active.
	pub active: bool,
}

pub struct TabManager {
	pub tabs: Vec<Tab>,
	pub current_tab: Option<Tab>,
}

impl TabManager {
	/// Create a new tab with the given type and path. The title is automatically generated, and if not provided defaults to "Untitled {index}"
	pub fn new_tab(&mut self, tab: Tab) {
		self.tabs.push(tab);
	}
	/// Get the amount of tabs that are open
	pub fn amount_of_tabs(&self) -> i32 {
		self.tabs.len() as i32
	}
	/// Remove a tab via its index
	pub fn remove_tab(&mut self, index: i32) {
		self.tabs.remove(index as usize);
	}
}
