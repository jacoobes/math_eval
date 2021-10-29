

    pub fn ask_for_expr() -> String {
        let mut expr = String::new();
        std::io::stdin().read_line(&mut expr).unwrap();
        expr
    }

    


