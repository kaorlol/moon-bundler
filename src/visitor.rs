use full_moon::{
	ast::{
		self, Expression,
		Stmt::{Assignment, FunctionCall, LocalAssignment},
	},
	node::{Node, Tokens},
	tokenizer::{StringLiteralQuoteType, Symbol, TokenType},
};

#[derive(Default)]
pub struct Visitor;

#[derive(Clone, Debug)]
pub struct Acquire {
	pub function_call: ast::FunctionCall,
	pub parent_type: ParentType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParentType {
	Assignment,
	Stmt,
}

impl PartialEq<ParentType> for &ParentType {
	fn eq(&self, other: &ParentType) -> bool {
		**self == *other
	}
}

impl Visitor {
	pub fn has_acquire(&mut self, tokens: Tokens<'_>) -> bool {
		tokens.into_iter().any(|token| match token.token_type() {
			TokenType::Identifier { identifier } => identifier.as_str() == "acquire",
			_ => false,
		})
	}

	pub fn get_string_literal(&mut self, tokens: Tokens<'_>) -> (String, StringLiteralQuoteType) {
		tokens
			.into_iter()
			.find_map(|token| match token.token_type() {
				TokenType::StringLiteral { literal, quote_type, .. } => {
					Some((literal.as_str().to_string(), *quote_type))
				}
				_ => None,
			})
			.unwrap()
	}

	pub fn has_parentheses(&mut self, tokens: Tokens<'_>) -> bool {
		tokens
			.into_iter()
			.filter_map(|token| match token.token_type() {
				TokenType::Symbol { symbol } => match symbol {
					Symbol::LeftParen | Symbol::RightParen => Some(symbol),
					_ => None,
				},
				_ => None,
			})
			.collect::<Vec<&Symbol>>()
			.len() == 2
	}

	fn process_assignment(
		&mut self,
		assignment: Option<&ast::Assignment>,
		local_assignment: Option<&ast::LocalAssignment>,
	) -> Vec<Acquire> {
		let expressions = match assignment {
			Some(assignment) => assignment.expressions(),
			None => local_assignment.unwrap().expressions(),
		};

		expressions
			.iter()
			.filter_map(|expr| {
				if let Expression::FunctionCall(call) = expr {
					let tokens = call.prefix().tokens();
					if self.has_acquire(tokens) {
						return Some(Acquire { function_call: call.clone(), parent_type: ParentType::Assignment });
					}
				}
				None
			})
			.collect()
	}

	pub fn get_function_calls(&mut self, stmts: &Vec<&ast::Stmt>) -> Vec<Acquire> {
		stmts
			.iter()
			.filter_map(|stmt| match stmt {
				LocalAssignment(local_assignment) => {
					let function_calls = self.process_assignment(None, Some(local_assignment));
					Some(function_calls)
				}
				Assignment(assignment) => {
					let function_calls = self.process_assignment(Some(assignment), None);
					Some(function_calls)
				}
				FunctionCall(function_call) => {
					let tokens = function_call.prefix().tokens();
					if self.has_acquire(tokens) {
						let function_call = function_call.clone();
						let parent_type = ParentType::Stmt;
						return Some(vec![Acquire { function_call, parent_type }]);
					}

					None
				}
				_ => None,
			})
			.flatten()
			.collect()
	}
}

pub fn get_acquire_info(visitor: &mut Visitor, suffixes: Vec<&ast::Suffix>) -> (String, String) {
	suffixes
		.iter()
		.map(|suffix| {
			let (string_literal, quote_type) = visitor.get_string_literal(suffix.tokens());
			let parentheses = visitor.has_parentheses(suffix.tokens());

			if parentheses {
				let acquire_call = format!("acquire({})", wrap_with_quote(&string_literal, quote_type));
				return (acquire_call, string_literal);
			}

			panic!("Could not get acquire info");
		})
		.next()
		.unwrap()
}

pub fn wrap_with_quote(string: &str, quote_type: StringLiteralQuoteType) -> String {
	match quote_type {
		StringLiteralQuoteType::Single => format!("'{}'", string),
		StringLiteralQuoteType::Double => format!("\"{}\"", string),
		StringLiteralQuoteType::Brackets => format!("[[{}]]", string),
		_ => string.to_string(),
	}
}
