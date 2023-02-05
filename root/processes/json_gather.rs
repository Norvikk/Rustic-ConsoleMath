
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Text {
    pub plus: Vec<String>,
    pub minus: Vec<String>,
    pub multiplication: Vec<String>,
    pub divided: Vec<String>,
}

#[derive(Debug)]
pub struct ArithmeticOperator {
    
    pub operator: char,
    pub synonyms: Vec<String>,
}

pub fn read_json(path: String) -> [ArithmeticOperator; 4] {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let synonyms: Text = serde_json::from_str(&contents).unwrap();

    let operators: [ArithmeticOperator; 4] = [
        ArithmeticOperator {operator: '+', synonyms: synonyms.plus},
        ArithmeticOperator {operator: '-', synonyms: synonyms.minus},
        ArithmeticOperator {operator: '*', synonyms: synonyms.multiplication},
        ArithmeticOperator {operator: '/', synonyms: synonyms.divided},
    ]; 

    return operators;
    
}

