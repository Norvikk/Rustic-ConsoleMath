pub fn get_equation() -> String {
    let mut equation_format = String::new();

    while equation_format.trim().is_empty() {
        println!("Equation: ");
        std::io::stdin().read_line(&mut equation_format).unwrap();
    }
    return equation_format.trim().to_owned();
}
