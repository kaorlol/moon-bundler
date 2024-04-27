mod cli;
mod formatter;
mod require_replacer;
mod visitor;

use std::{
	fs::{read_to_string, remove_file, write},
	path::{Path, PathBuf},
	time::Instant,
};

use anyhow::Result;
use cli::parse_args;
use formatter::{beautify_code, minify_code};
use require_replacer::RequireReplacer;
use visitor::Visitor;

fn main() -> Result<()> {
	let args = parse_args();
	println!("Starting bundler...");
	args.clone().into_iter().for_each(|(key, value)| {
		println!("{}: {}", key, value);
	});

	let start = Instant::now();

	let input_file_binder = args.input.clone();
	let input_name = input_file_binder.split('\\').last().unwrap();
	let input_file = PathBuf::from(&args.input);

	let mut visitor = Visitor::default();
	let mut replacer = RequireReplacer::new(input_file.parent().unwrap(), &mut visitor, args.use_root);
	let bundled_code = replacer.replace_requires(&input_file)?;

	println!("\nTook {:?} to bundle {input_name}", start.elapsed());
	make_bundled_file(&args.output, &bundled_code.unwrap())?;

	let file = PathBuf::from(&args.output).join("bundled.lua");
	match args {
		cli::Args { minify: true, .. } => minify_code(&file),
		cli::Args { beautify: true, .. } => beautify_code(&file),
		_ => (),
	}

	let mut content = read_to_string(&file)?;
	content.insert_str(0, "-- This file was bundled by https://github.com/kaorlol/moon-bundler\n\n");
	write(&file, content)?;

	Ok(())
}

fn make_bundled_file(output_path: &str, code: &str) -> Result<()> {
	let bundled_path = Path::new(output_path).join("bundled.lua");
	if bundled_path.exists() {
		remove_file(&bundled_path)?;
	}

	write(bundled_path, code)?;
	Ok(())
}
