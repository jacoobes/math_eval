pub mod cli;
pub mod tokenizer;
use cli::cli_tools::*;
use tokenizer::tokenizer::tokenizer;
fn main() {
    print_controls();
    loop {

        println!("Enter a math expression: ");
        let expr = ask_for_expr().trim().to_string();

        tokenizer(expr);
        println!();
        
        
    }
}


