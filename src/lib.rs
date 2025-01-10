use std::error::Error;
use std::fs;

pub struct Config {
    pub crossword_file: String,
    pub answers_file: String,
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

pub struct CrosswordLocation <'a> {
    pub row: usize,
    pub col: usize,
    pub crossword: &'a Crossword<'a>,
}
impl <'a> CrosswordLocation<'a> {
    pub fn build(row: usize, col: usize, crossword: &'a Crossword<'a>) -> CrosswordLocation<'a> {
        CrosswordLocation{
            row,
            col,
            crossword,
        }
    }

    /// Returns the vector sum of two Crossword Locations.
    /// If the crosswords have dimension mismatch or if the sum is out of bounds of the crossword, returns None()
    pub fn add_locations(location1: &CrosswordLocation, location2: &CrosswordLocation) -> Option<CrosswordLocation<'a>> {
        if location1.crossword.rows != location2.crossword.rows
            || location1.crossword.cols != location2.crossword.cols {
            return None;
        }

        let row = location1.row + location2.row;
        let col = location1.col + location2.col;

        if row < 0 || row > location1.crossword.rows
            || col < 0 || col > location1.crossword.cols {}


        Some(CrosswordLocation{
            row,
            col,
            crossword,
        })
    }
}
pub struct AnswerLocations<'a> {
    pub start: &'a CrosswordLocation<'a>,
    pub end: &'a CrosswordLocation<'a>,
}

pub struct Crossword<'a> {
    pub characters: Vec<char>,
    pub rows: usize,
    pub cols: usize,
}

impl <'a> Crossword<'a> {
    /// Builds a Crossword from the given String.
    /// String contents must be alphanumeric symbols.
    /// Columns are not separated (symbols must be adjacent). Rows are separated by newlines.
    /// Each row must contain the same number of columns or an error will be returned.
    pub fn build(crossword_string: String) -> Result<Crossword, &'static str> {
        let mut crossword = Vec::new();

        let mut rows: usize = 0;
        let mut cols: usize = 0;

        for (i, line) in crossword_string.lines().enumerate() {
            let mut current_cols: usize = 0;
            for (j, char) in line.chars().enumerate() {
                if !char.is_alphanumeric() {
                    return Err(format!("'{}' is not alphanumeric in row {}, col{}.", char, i, j).into());
                } else {
                    crossword.push(char);
                }
                current_cols += 1;
            }
            if i == 0 {
                cols = current_cols;
            } else if current_cols != rows {
                return Err(format!("Row {} has a the wrong number of columnts. Exepcted {} and had {}", i, rows, current_cols).into());
            }
            rows += 1;
        }



        if crossword.len() == 0 {
            return Err("Crossword file empty".into());
        }
        Ok(Crossword{
            characters: crossword,
            rows,
            cols,
        })
    }

    pub fn at(&self, location: &CrosswordLocation) -> char {
        let i = location.row * self.cols + location.col;
        self.characters[i]
    }
}

impl IntoIterator for Crossword {
    type Item = char;
    type IntoIter = <Vec<char> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.characters.into_iter()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let crossword = Crossword::build(config.crossword_file)?;
    let answers = lex_answers(config.answers_file)?;

    Ok(())
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

pub fn solve_crossword(crossword: Crossword, answers: Vec<String>) -> Result<Vec<AnswerLocations>, Box<dyn Error>> {
    let mut answer_locations = Vec::new();
    for (i, line) in crossword.characters.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            for word in answers.iter() {

            }
        }
    }
    Ok(answer_locations)
}