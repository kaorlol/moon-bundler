use crate::visitor::{get_require_info, Visitor};
use anyhow::Result;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub struct RequireReplacer<'a> {
	root: &'a Path,
	visitor: &'a mut Visitor,
	use_root: bool,
}

impl<'a> RequireReplacer<'a> {
	pub fn new(root: &'a Path, visitor: &'a mut Visitor, use_root: bool) -> Self {
		RequireReplacer { root, visitor, use_root }
	}

	pub fn replace_requires(&mut self, input: &PathBuf) -> Result<Result<String, String>> {
		let raw_code = read_to_string(input)?;

		let ast = full_moon::parse(&raw_code)?;
		let stmts = ast.nodes().stmts().collect();

		let requires = self.visitor.get_function_calls(&stmts);
		if requires.is_empty() {
			return Ok(Err(raw_code));
		}

		Ok(Ok(requires.iter().fold(raw_code, |prev, require| {
			let prefix = require.function_call.prefix();
			let suffixes = require.function_call.suffixes().cloned().collect();
			let (call, require_path) = get_require_info(self.visitor, prefix, &suffixes);
			let path = match self.use_root {
				true => self.root.join(require_path),
				false => input.parent().unwrap().join(require_path),
			};

			let code_from_require =
				self.replace_requires(&path).expect(&format!("failed to bundle file: {}", path.display()));

			let code_from_require = match code_from_require {
				Ok(code_with_require) => code_with_require,
				Err(code_without_require) => code_without_require,
			};

			let ast = full_moon::parse(&code_from_require).expect("failed to parse code from require");
			let stmts = ast.nodes().stmts().cloned().collect();
			let last_stmt = ast.nodes().last_stmt().cloned();

			let function_call = self.visitor.create_function_call(&stmts, last_stmt);
			prev.replace(&call.to_string(), &function_call.to_string())
		})))
	}
}
