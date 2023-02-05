use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub struct OpSynonyms {
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

pub fn read_json<P: AsRef<Path>>(path: P) -> anyhow::Result<[ArithmeticOperator; 4]> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let synonyms = serde_json::from_str::<OpSynonyms>(&contents)?;

    let operators = [
        ArithmeticOperator {
            operator: '+',
            synonyms: synonyms.plus,
        },
        ArithmeticOperator {
            operator: '-',
            synonyms: synonyms.minus,
        },
        ArithmeticOperator {
            operator: '*',
            synonyms: synonyms.multiplication,
        },
        ArithmeticOperator {
            operator: '/',
            synonyms: synonyms.divided,
        },
    ];

    Ok(operators)
}
