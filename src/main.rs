mod cli;
mod utils;
mod visitor;

use anyhow::Result;
use cli::parse_args;
use std::{fs::read_to_string, path::PathBuf, time::Instant};
use utils::{create_func_call, make_bundled_file};
use visitor::{get_acquire_info, Visitor};

fn main() -> Result<()> {
	let args = parse_args();

	println!("Starting bundler...");

	let start = Instant::now();

	// let ast = full_moon::parse(&code)?;
	// let stmts = ast.nodes().stmts().collect();

	let indent = if args.use_root.unwrap_or(false) { "\n" } else { "" };
	let cloned_input_file = args.input_file.clone();
	let input_name = cloned_input_file.split('\\').last().unwrap();
	// println!("Parsed the file in {:?}{indent}", start.elapsed());

	let mut visitor = Visitor::default();
	let bundled_code = match replace_acquires(&PathBuf::from(&args.input_file), &mut visitor)? {
		Ok(bundled_code) => bundled_code,
		Err(default_code) => default_code,
	};

	println!("{indent}Took {:?} to bundle {input_name}", start.elapsed());

	make_bundled_file(&args.output, &bundled_code)?;

	Ok(())
}

fn replace_acquires(input_path: &PathBuf, mut visitor: &mut Visitor) -> anyhow::Result<Result<String, String>> {
	let raw_code = read_to_string(input_path)?;

	let ast = full_moon::parse(&raw_code)?;
	let stmts = ast.nodes().stmts().collect();

	let acquires = visitor.get_function_calls(&stmts);

	if acquires.is_empty() {
		Ok(Err(raw_code))
	} else {
		Ok(Ok(acquires.iter().fold(raw_code, |prev, acquire| {
			let suffixes = acquire.function_call.suffixes().collect();

			let (call, relative_path) = get_acquire_info(&mut visitor, suffixes);

			let mut path = input_path.clone().parent().unwrap().to_path_buf();
			path.push(relative_path);

			let code_from_acquire =
				replace_acquires(&path, &mut visitor).expect(&format!("failed to bundle file: {}", path.display()));

			let code_from_acquire = match code_from_acquire {
				Ok(code_with_acquire) => code_with_acquire,
				Err(code_without_acquire) => code_without_acquire,
			};

			let wrapped_code = create_func_call(&code_from_acquire, &acquire.parent_type);
			prev.replace(&call, &wrapped_code)
		})))
	}
}
