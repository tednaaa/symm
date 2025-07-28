use std::path::Path;

pub fn prettify_path(path: &Path) -> String {
	let home_dir = dirs::home_dir().unwrap_or_default();

	format!("~/{}", path.strip_prefix(home_dir).unwrap_or(path).display())
}
