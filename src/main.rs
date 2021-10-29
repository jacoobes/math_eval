pub mod cli;
pub mod tokenizer;
use cli::cli_tools::*;
fn main() {
    print_controls();
    loop {

        println!("Enter a math expression: ");
        let expr = ask_for_expr().trim();
        println!();
        
        
    }
}


