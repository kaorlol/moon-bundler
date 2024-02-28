use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Args {
	#[clap(short, long)]
	pub input: String,

	#[clap(short, long)]
	pub output: String,

	#[clap(short, long, default_value_t = false)]
	pub use_root: bool,

	#[clap(short, long, default_value_t = false)]
	pub minify: bool,

	#[clap(short, long, default_value_t = false)]
	pub beautify: bool,
}

pub fn parse_args() -> Args {
	let args = Args::parse();
	if !Path::new(&args.input).exists() {
		panic!("Input file does not exist");
	}

	if !Path::new(&args.output).exists() {
		panic!("Output path does not exist");
	}

	args
}
