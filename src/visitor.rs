use full_moon::{
	ast::{
		self, Expression,
		Stmt::{self, Assignment, FunctionCall, LocalAssignment},
	},
	node::{Node, Tokens},
	tokenizer::{StringLiteralQuoteType, Symbol, Token, TokenReference, TokenType},
};

#[derive(Default, Clone, Copy)]
pub struct Visitor;

#[derive(Clone, Debug)]
pub struct Require {
	pub function_call: ast::FunctionCall,
	pub parent_type: ParentType,
}

impl Require {
	pub fn from(function_call: ast::FunctionCall, parent_type: ParentType) -> Self {
		Self { function_call, parent_type }
	}
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

pub enum Assignments<'a> {
	Assignment(&'a ast::Assignment),
	LocalAssignment(&'a ast::LocalAssignment),
}

impl Visitor {
	pub fn has_require(&mut self, tokens: Tokens<'_>) -> bool {
		tokens.into_iter().any(|token| match token.token_type() {
			TokenType::Identifier { identifier } => identifier.as_str() == "require",
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
			.expect("Could not get string literal")
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

	fn process_assignment(&mut self, assignment: &Assignments) -> Vec<Require> {
		let expressions = match assignment {
			Assignments::Assignment(assignment) => assignment.expressions(),
			Assignments::LocalAssignment(local_assignment) => local_assignment.expressions(),
		};

		expressions
			.iter()
			.filter_map(|expr| {
				if let Expression::FunctionCall(call) = expr {
					let tokens = call.prefix().tokens();
					if self.has_require(tokens) {
						return Some(Require::from(call.clone(), ParentType::Assignment));
					}
				}
				None
			})
			.collect()
	}

	pub fn get_function_calls(&mut self, stmts: &Vec<&ast::Stmt>) -> Vec<Require> {
		stmts
			.iter()
			.filter_map(|stmt| match stmt {
				LocalAssignment(local_assignment) => {
					let function_calls = self.process_assignment(&Assignments::LocalAssignment(&local_assignment));
					Some(function_calls)
				}
				Assignment(assignment) => {
					let function_calls = self.process_assignment(&Assignments::Assignment(&assignment));
					Some(function_calls)
				}
				FunctionCall(function_call) => {
					let tokens = function_call.prefix().tokens();
					if self.has_require(tokens) {
						return Some(vec![Require::from(function_call.clone(), ParentType::Stmt)]);
					}
					None
				}
				_ => None,
			})
			.flatten()
			.collect()
	}

	pub fn create_function_call(&self, stmts: &Vec<ast::Stmt>, last_stmt: Option<ast::LastStmt>) -> Stmt {
		let left_paren = Token::new(TokenType::Symbol { symbol: Symbol::LeftParen });
		let right_paren = Token::new(TokenType::Symbol { symbol: Symbol::RightParen });

		let function = Token::new(TokenType::Symbol { symbol: Symbol::Function });
		let end = Token::new(TokenType::Symbol { symbol: Symbol::End });

		let stmts = stmts.iter().map(|stmt| (stmt.clone(), None)).collect();
		let last_stmt = last_stmt.map(|stmt| (stmt.clone(), None));

		let prefix = ast::Prefix::Expression(Box::new(Expression::Function((
			TokenReference::new(vec![left_paren], function, vec![]),
			ast::FunctionBody::new()
				.with_block(ast::Block::new().with_stmts(stmts).with_last_stmt(last_stmt))
				.with_end_token(TokenReference::new(vec![], end, vec![right_paren])),
		))));

		Stmt::FunctionCall(ast::FunctionCall::new(prefix))
	}
}

pub fn get_require_info(
	visitor: &mut Visitor,
	prefix: &ast::Prefix,
	suffixes: &Vec<ast::Suffix>,
) -> (ast::FunctionCall, String) {
	suffixes
		.into_iter()
		.map(|suffix| {
			let (string_literal, _) = visitor.get_string_literal(suffix.tokens());
			let parentheses = visitor.has_parentheses(suffix.tokens());

			if parentheses {
				let require_call = ast::FunctionCall::new(prefix.clone()).with_suffixes(suffixes.clone());
				return (require_call, string_literal);
			}

			panic!("Could not get require info");
		})
		.next()
		.unwrap()
}
