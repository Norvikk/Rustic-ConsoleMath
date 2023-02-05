use super::json_gather::ArithmeticOperator;

pub fn process_equation(equation: &str) -> f64 {
    let interpreter = meval::eval_str(&equation).unwrap();
    return interpreter;
}

#[allow(dead_code)]
pub fn translate_equation(equation: &str, operators: &[ArithmeticOperator; 4]) -> String {
    struct bool_symbol {
        pub state: bool,
        pub symbol: char,
    }


    let mut result_equation = String::new();

    for word in equation.split_whitespace() {
        
        let carrier1_check = check_for_num(&word);
        let carrier2_check = check_for_synonym(&word, &operators);
        let carrier3_check = attempt_number_translation(&word);



        if carrier1_check {
            result_equation.push_str(&word);
        } 
        else if carrier2_check.state {
            result_equation.push_str(&carrier2_check.symbol.to_string());
        }
        else if carrier3_check != -127{
            result_equation.push_str(&carrier3_check.to_string())
        }
        
    }

    
    println!("Entry name *{}*", result_equation);

    return result_equation;

    fn check_for_num(num: &str) -> bool {
        match num.parse::<f64>() {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn check_for_synonym(word: &str, operators: &[ArithmeticOperator; 4]) -> bool_symbol {
        let mut result: bool_symbol = bool_symbol { state: false, symbol: ' ' };
        
        if operators[0].synonyms.iter().any(|s| s == word) {
            result.state = true;
            result.symbol = operators[0].operator;
        } else if operators[1].synonyms.iter().any(|s| s == word) {
            result.state = true;
            result.symbol = operators[1].operator;
        } else if operators[2].synonyms.iter().any(|s| s == word) {
            result.state = true;
            result.symbol = operators[2].operator; 
        } else if operators[3].synonyms.iter().any(|s| s == word) {
            result.state = true;
            result.symbol = operators[3].operator; }



        return result;
    }



    fn attempt_number_translation(word: &str) -> i8{

        let numbers = [
                "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
                "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty",
                "twentyone", "twentytwo", "twentythree", "twentyfour", "twentyfive", "twentysix", "twentyseven", "twentyeight", "twentynine", "thirty",
            ];

        let mut i = 0;
       for x in numbers{
            if x == word{
                return i + 1;
            }
        i += 1;
       }

       return  -127;
    }
}
