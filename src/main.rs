pub mod cli;
pub mod panicker;
pub mod tokenizer;
pub mod parser;

use cli::cli_tools::*;
use tokenizer::tokenizer::Tokenizer;
fn main() {
    print_controls();
    loop {
        println!("Enter a math expression: ");
        let expr = ask_for_expr().trim().to_string();

        let mut token_generator = Tokenizer::new(&expr);
        token_generator.run();
        println!("{:?}", token_generator.tokens)
    }
}
