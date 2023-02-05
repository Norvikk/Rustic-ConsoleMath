mod processes;

pub fn main() {
    let arithmetics = processes::json_gather::read_json(String::from("root/processes/file.json"));

    let equation = processes::gather::get_equation();


    let translated_equation = processes::solve::translate_equation(&equation, &arithmetics);

    let equation_result = processes::solve::process_equation(&translated_equation);
    println!("The answer is {} (short {})", equation_result, format!("{:.2}", equation_result)  )
}



