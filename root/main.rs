mod processes;

pub fn main() -> anyhow::Result<()> {
    let arithmetics = processes::json_gather::read_json("root/processes/file.json")?;
    let equation = processes::gather::get_equation()?;
    let translated_equation = processes::solve::translate_equation(&equation, &arithmetics);
    let equation_result = processes::solve::process_equation(&translated_equation, &equation)?;

    println!(
        "The answer is {equation_result} (short {})",
        format!("{:.2}", equation_result)
    );

    Ok(())
}
