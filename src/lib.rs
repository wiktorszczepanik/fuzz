use std::error::Error;
use std::fs;
use std::path::PathBuf;

pub struct Config {
    pub text: String,
    pub file_path: PathBuf,
    pub top: i8
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        } else if args.len() > 4  {
            return Err("incorrect number of arguments")
        }
        let text: String = args[1].clone(); // no validation
        let file_path: PathBuf = Self::assign_file_path(&args[2])?;
        if args.len() == 3 {
            let top: i8 = 100;
            return Ok(Config{text, file_path, top})
        }
        let top: i8 = Self::assign_top(&args[3])?;
        Ok(Config{text, file_path, top})
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let _content = fs::read_to_string(config.file_path)
        .expect("cannot read file content");
    Ok(())
}
