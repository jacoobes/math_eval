pub mod cli;
pub mod panicker;
pub mod parser;
pub mod tokenizer;

use cli::cli_tools::*;
use tokenizer::tokenizer::Tokenizer;

use crate::parser::parser::Parser;

fn main() {
    print_controls();
    loop {
        println!("Enter a math expression: \n");
        let expr = ask_for_expr().trim().to_string();
        if expr == "end" {
            break;
        }
        let mut token_generator = Tokenizer::new(&expr);
        token_generator.run();
        if Parser::check_lexer_errors(&token_generator.tokens) {
            continue;
        }
        let mut parser = Parser::new(token_generator.tokens);
        let values = parser.parse();

        println!("{:?} \n\n\n", values)
    }
}
