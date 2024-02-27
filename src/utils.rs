use std::{
	fs::{self, read_to_string},
	path::Path,
};

use crate::visitor::ParentType;
use anyhow::Result;

pub fn get_code(main: &str, rel_path: &str) -> String {
	let parent = Path::new(main).parent().unwrap();
	if !parent.exists() {
		panic!("Could not get parent of main path");
	}

	let path = parent.join(rel_path);
	read_to_string(path).unwrap()
}

pub fn make_bundled_file(main: &str, code: &str) -> Result<()> {
	let parent = Path::new(main).parent().ok_or_else(|| anyhow::anyhow!("Could not get parent of main path"))?;
	let bundled_path = parent.join("bundled.lua");
	if bundled_path.exists() {
		fs::remove_file(&bundled_path)?;
	}

	fs::write(bundled_path, code)?;
	Ok(())
}

pub fn create_func_call(code_to_wrap: &str, parent_type: &ParentType) -> String {
	if parent_type == ParentType::Stmt {
		return format!("{code_to_wrap}");
	}

	format!("(function() {code_to_wrap} end)()")
}
