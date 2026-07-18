use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    pub text: String,
    pub file_path: PathBuf,
    pub top: i8
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        } else if args.len() > 4  {
            return Err("incorrect number of arguments")
        }
        let text: String = Self::assign_fuzz_text(&args[1]);
        let file_path: PathBuf = Self::assign_file_path(&args[2])?;
        if args.len() == 3 {
            let top: i8 = 100;
            return Ok(Args {text, file_path, top})
        }
        let top: i8 = Self::assign_top(&args[3])?;
        Ok(Args {text, file_path, top})
    }

    fn assign_fuzz_text(text: &str) -> String {
        text.to_string()
    }

    fn assign_file_path(file_path: &String) -> Result<PathBuf, &'static str> {
        let path = PathBuf::from(file_path);
        if !path.exists() {
            return Err("file does not exits")
        }
        if !path.is_file() {
            return Err("provided path is not a file")
        }
        Ok(path)
    }

    fn assign_top(top_value: &String) -> Result<i8, &'static str> {
        let top: i8 = top_value
            .parse()
            .map_err(|_| "top must be valid integer")?;
        if !(top > 0 && top < 100) {
            return Err("invalid \'top\' range")
        }
        Ok(top)
    }
}
#[cfg(test)]
mod tests {
    use crate::config::Args;

    // const CORRECT_FILE_PATH: &str = "../data/poem.txt";
    // const INCORRECT_FILE_PATH: &str = "../data/incorrect_file.txt";
    const INPUT: &str = include_str!("../data/poem.txt");

    #[test]
    fn test_file_included() {
        assert!(!INPUT.is_empty());
    }

    #[test]
    fn input_none_arguments() {
        let args: Vec<String> = vec![];
        let result= Args::build(&args);
        assert_eq!(result.unwrap_err(), "not enough arguments")
    }

    #[test]
    fn input_too_few_arguments() {
        let args: Vec<String> = vec![String::from("xyz")];
        let result= Args::build(&args);
        assert_eq!(result.unwrap_err(), "not enough arguments")
    }

    #[test]
    fn input_too_many_arguments() {
        let args: Vec<String> = vec![
            String::from("fuzz"),
            String::from("xyz"),
            String::from("some_file.txt"),
            String::from("50"),
            String::from("xxx")
        ];
        let result= Args::build(&args);
        assert_eq!(result.unwrap_err(), "incorrect number of arguments")
    }

    #[test]
    fn assign_correct_fuzz_text() {
        let text: String = String::from("some text");
        let result = Args::assign_fuzz_text(&*text);
        assert_eq!(result, text)

    }

    // #[test]
    // fn assign_correct_file_path() {
    //     let result = Args::assign_file_path(&CORRECT_FILE_PATH.to_string());
    //     assert_eq!(result.unwrap().exists(), true)
    // }
    //
    // #[test]
    // fn assign_incorrect_file_path() {
    //     let result = Args::assign_file_path(&INCORRECT_FILE_PATH.to_string());
    //     assert_eq!(result.unwrap().exists(), false)
    // }

}
