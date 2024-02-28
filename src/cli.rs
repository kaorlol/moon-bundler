use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Args {
	#[clap(short, long = "The main file to bundle")]
	pub input_file: String,

	#[clap(short, long = "The output folder")]
	pub output: String,

	#[clap(short, long = "If you use root paths in acquires")]
	pub use_root: Option<bool>,

	#[clap(short, long = "Extra prints showing what happening")]
	pub debug: Option<bool>,
}

pub fn parse_args() -> Args {
	Args::parse()
}