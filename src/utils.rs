use std::{fs, path::Path};

use crate::visitor::ParentType;
use anyhow::Result;

// pub fn get_code(main: &str, rel_path: &str) -> String {
// 	let parent = Path::new(main).parent().unwrap();
// 	if !parent.exists() {
// 		panic!("Could not get parent of main path. main: {main}, rel_path: {rel_path}");
// 	}

// 	let path = parent.join(rel_path);
// 	println!("Reading {:?}", path);

// 	read_to_string(path).unwrap()
// }

pub fn make_bundled_file(output_path: &str, code: &str) -> Result<()> {
	let bundled_path = Path::new(output_path).join("bundled.lua");
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
