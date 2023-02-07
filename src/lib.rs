pub mod processes;

#[cfg(test)]
mod tests {
    use crate::processes;


    
    #[test]
    fn addition() -> anyhow::Result<()>  {
        let equation: [String; 5] =  [String::from("10+10"), String::from("ten + ten"), String::from("10 plus ten"), String::from("ten plus ten"), String::from("unmatched property of 1 plus sample integer 9 plus ten")];
        let global_answer: f64 = 20.0;

        let arithmetics = processes::json_gather::read_json("src/data/file.json")?;
        for current_equation in equation {
            let equation = &current_equation;
            let translated_equation = processes::solve::translate_equation(&equation, &arithmetics);
            let equation_result = processes::solve::process_equation(&translated_equation, &current_equation)?;

            assert_eq!(&equation_result, &global_answer);
        }

        Ok(())
    }

    #[test]
    fn minus() -> anyhow::Result<()>  {
        let equation: [String; 5] =  [String::from("-10-10"), String::from("- ten - ten"), String::from("-10 minus ten"), String::from("- ten minus ten"), String::from("unmatched property of -1 minus integer 9 substracted by ten")];
        let global_answer: f64 = -20.0;

        let arithmetics = processes::json_gather::read_json("src/data/file.json")?;
        for current_equation in equation {
            let equation = &current_equation;
            let translated_equation = processes::solve::translate_equation(&equation, &arithmetics);
            let equation_result = processes::solve::process_equation(&translated_equation, &current_equation)?;
            assert_eq!(&equation_result, &global_answer);
        }

        Ok(())
    }

    #[test]
    fn multiplication() -> anyhow::Result<()>  {
        let equation: [String; 5] =  [String::from("5 * 4"), String::from("five * four"), String::from("5 times four"), String::from("five times 4"), String::from("unmatched property of five times the integer five substracted by 5")];
        let global_answer: f64 = 20.0;

        let arithmetics = processes::json_gather::read_json("src/data/file.json")?;
        for current_equation in equation {
            let equation = &current_equation;
            let translated_equation = processes::solve::translate_equation(&equation, &arithmetics);
            let equation_result = processes::solve::process_equation(&translated_equation, &current_equation)?;
            assert_eq!(&equation_result, &global_answer);
        }

        Ok(())
    } 


    #[test]
    fn division() -> anyhow::Result<()>  {
        let equation: [String; 5] =  [String::from("40 / 2"), String::from("400 / 20"), String::from("4 / 0.2"), String::from("four divided by 0.2"), String::from("unmatched property of four divided by the double 0.2 plus 5 substracted by 5")];
        let global_answer: f64 = 20.0;

        let arithmetics = processes::json_gather::read_json("src/data/file.json")?;
        for current_equation in equation {
            let equation = &current_equation;
            let translated_equation = processes::solve::translate_equation(&equation, &arithmetics);
            let equation_result = processes::solve::process_equation(&translated_equation, &current_equation)?;
            assert_eq!(&equation_result, &global_answer);
        }

        Ok(())
    }
    
}
