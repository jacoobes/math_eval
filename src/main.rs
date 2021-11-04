pub mod cli;
pub mod tokenizer;
use cli::cli_tools::*;
use tokenizer::tokenizer::Tokenizer;
fn main() {
    print_controls();
    loop {

        println!("Enter a math expression: ");
        let expr =ask_for_expr().trim().to_string();

        let token_generator = Tokenizer::new(&expr);
        
        
       token_generator.run();
        
        
    }
}


