use std::{
	fs::{read_to_string, write},
	path::PathBuf,
	time::Instant,
};

use darklua_core::{process, Configuration, GeneratorParameters, Options, Resources};
use stylua_lib::format_code;

pub fn beautify_code(file: &PathBuf) {
	let code = read_to_string(file).unwrap();
	let start = Instant::now();

	let default_config = stylua_lib::Config::default();
	let verify_output = stylua_lib::OutputVerification::Full;
	let formatted_code = format_code(&code, default_config, None, verify_output).unwrap();

	println!("Took {:?} to format bundled.lua", start.elapsed());
	write(file, formatted_code).unwrap();
}

pub fn minify_code(file: &PathBuf) {
	let start = Instant::now();

	let resources = Resources::from_file_system();
	let configuration = Configuration::empty().with_generator(GeneratorParameters::default_dense());
	let process_options = Options::new(&file).with_output(&file).with_configuration(configuration);

	process(&resources, process_options);
	println!("Took {:?} to minify bundled.lua", start.elapsed());
}
