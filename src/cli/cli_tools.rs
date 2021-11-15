pub fn ask_for_expr() -> String {
    let mut expr = String::new();
    std::io::stdin().read_line(&mut expr).unwrap();
    expr
}

pub fn print_controls() {
    println!();
    println!("------------------------------------------------------------------");
    println!("Ctrl-C to force shutdown or type \"end\" terminates the calculator");
    println!("------------------------------------------------------------------");
    println!();
}
