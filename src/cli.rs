use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug, Clone)]
#[command(version, long_about = None)]
pub struct Args {
	#[clap(short, long)]
	pub input: String,

	#[clap(short, long)]
	pub output: String,

	#[clap(short = 'r', long, default_value_t = false)]
	pub use_root: bool,

	#[clap(short, long, default_value_t = false)]
	pub minify: bool,

	#[clap(short, long, default_value_t = false)]
	pub beautify: bool,
}

impl IntoIterator for Args {
	type Item = (String, String);
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		vec![
			("input".to_string(), self.input),
			("output".to_string(), self.output),
			("use_root".to_string(), self.use_root.to_string()),
			("minify".to_string(), self.minify.to_string()),
			("beautify".to_string(), self.beautify.to_string()),
		]
		.into_iter()
	}
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
