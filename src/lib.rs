use std::error::Error;
use std::fs;

pub struct Config {
    pub crossword_file: String,
    pub answers_file: String,
}

pub struct CrosswordLocation {
    pub row: i32,
    pub col: i32,
}
pub struct AnswerLocation {
    pub start: CrosswordLocation,
    pub end: CrosswordLocation,
}

pub struct Crossword {
    pub crossword: Vec<Vec<char>>,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let crossword_file = args[1].clone();
        let answers_file = args[2].clone();

        Ok(Config {
            crossword_file,
            answers_file,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let crossword = lex_crossword(config.crossword_file)?;
    let answers = lex_answers(config.answers_file)?;

    Ok(())
}

/// Reads into memory the crossword at the given filepath.
/// Crossword File contents must be alphanumeric symbols.
/// Columns are not separated (symbols must be adjacent). Rows are separated by newlines.
/// Each row must contain the same number of columns or an error will be returned.
pub fn lex_crossword(crossword_file: String) -> Result<Crossword, Box<dyn Error>> {
    let file_contents = fs::read_to_string(crossword_file)?;
    let mut crossword = Vec::new();

    for (i, line) in file_contents.lines().enumerate() {
        crossword.push(Vec::new());
        for (j, char) in line.chars().enumerate() {
            if !char.is_alphanumeric() {
                return Err(format!("{} is not alphanumeric in row {}, col{}.", line, i, j).into());
            } else {
                crossword[i].push(char);
            }
        }
        if i != 0 && crossword[i].len() != crossword[0].len() {
            return Err(format!("Not all crossword rows same length. Row {} has length {} while row 0 has length {}",
            i, crossword[i].len(), crossword[0].len()).into());
        }
    }

    if crossword.len() == 0 {
        return Err("Crossword file empty".into());
    }
    Ok(Crossword{crossword})
}

/// Reads into memory the crossword answers at the given filepath.
/// Each word must be separated by a newline, and should not contain spaces.
pub fn lex_answers(answers_file: String) -> Result<Vec<String>, Box<dyn Error>> {
    let file_contents = fs::read_to_string(answers_file)?;
    let mut answers = Vec::new();

    for line in file_contents.lines() {
        answers.push(line.to_string());
    }

    if answers.len() == 0 {
        return Err("Answers file empty".into());
    }
    Ok(answers)
}

pub fn solve_crossword(crossword: Crossword, answers: Vec<String>) -> Result<Vec<AnswerLocation>, Box<dyn Error>> {
    let mut answer_locations = Vec::new();
    for (i, line) in crossword.crossword.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            for word in answers.iter() {

            }
        }
    }
    Ok(answer_locations)
}