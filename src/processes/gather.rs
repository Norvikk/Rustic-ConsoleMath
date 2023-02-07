pub fn get_equation() -> anyhow::Result<String> {
    let equation_format = inquire::Text::new("Equation:").prompt()?.trim().to_owned();

    Ok(equation_format)
}
