mod cli;
mod formatter;
mod utils;
mod visitor;

use std::{fs::read_to_string, path::PathBuf, time::Instant};

use anyhow::Result;
use cli::parse_args;
use formatter::{beautify_code, minify_code};
use utils::{create_func_call, make_bundled_file};
use visitor::{get_acquire_info, Visitor};

fn main() -> Result<()> {
	let args = parse_args();

	println!("Starting bundler...");
	let start = Instant::now();
	let input_file_binder = args.input.clone();
	let input_name = input_file_binder.split('\\').last().unwrap();

	println!("Using root paths: {}", args.use_root);

	let mut visitor = Visitor::default();
	let bundled_code = match replace_acquires(&PathBuf::from(&args.input), &mut visitor)? {
		Ok(bundled_code) => bundled_code,
		Err(default_code) => default_code,
	};

	println!("\nTook {:?} to bundle {input_name}", start.elapsed());

	make_bundled_file(&args.output, &bundled_code)?;

	let file = PathBuf::from(&args.output).join("bundled.lua");
	if args.beautify {
		beautify_code(&file);
	}

	if args.minify {
		minify_code(&file);
	}

	Ok(())
}

fn replace_acquires(input_path: &PathBuf, mut visitor: &mut Visitor) -> Result<Result<String, String>> {
	let raw_code = read_to_string(input_path)?;

	let ast = full_moon::parse(&raw_code)?;
	let stmts = ast.nodes().stmts().collect();

	let acquires = visitor.get_function_calls(&stmts);
	if acquires.is_empty() {
		return Ok(Err(raw_code));
	}

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
