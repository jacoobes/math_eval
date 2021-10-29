pub mod cli;

use cli::cli_tools::ask_for_expr;
fn main() {
    println!();
    println!("Ctrl-C to force shutdown or \"end\" to stop evaluating");
    loop {

        println!("Enter a math expression: ");
        
        let expr = ask_for_expr().trim();
        println!();
        
        
    }
}


