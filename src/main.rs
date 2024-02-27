mod utils;
mod visitor;

use anyhow::Result;
use utils::{create_func_call, get_code, make_bundled_file};
use visitor::{get_acquire_info, Visitor};
use std::time::Instant;

fn main() -> Result<()> {
	let main_path = "F:\\Roblox\\bundler\\lua\\main.lua";
	let main = include_str!("../lua/main.lua");
	let start = Instant::now();

	let ast = full_moon::parse(main)?;
	let stmts = ast.nodes().stmts().collect();

	let mut visitor = Visitor::default();
	let acquires = visitor.get_function_calls(&stmts);
	let bundled = acquires.iter().fold(main.to_string(), |acc, acquire| {
		let suffixes = acquire.function_call.suffixes().collect();

		let (lua_call, rel_path) = get_acquire_info(&mut visitor, suffixes);
		let code = get_code(main_path, &rel_path);
		let wrapped_code = create_func_call(&code, &acquire.parent_type);

		acc.replace(&lua_call, &wrapped_code)
	});

	println!("{:?}", start.elapsed());

	make_bundled_file(main_path, &bundled)?;

	Ok(())
}
