use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name = "fuzz")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
pub struct Cli {
    pub text: String,
    pub file_path: PathBuf,
    #[arg(short, long, default_value_t = 50)]
    pub top: u8,
    #[arg(short, long)]
    pub lines: bool,
    #[arg(short, long)]
    pub score: bool,
}

impl Cli {
    pub fn validate(&self) -> Result<(), &'static str> {
        Self::validate_fuzz_text(&self.text)?;
        Self::validate_file_path(&self.file_path)?;
        Self::validate_top(&self.top)?;
        Ok(())
    }

    fn validate_fuzz_text(text: &str) -> Result<(), &'static str> {
        if text.is_empty() {
            Err("fuzz text cannot be empty")
        } else {
            Ok(())
        }
    }

    fn validate_file_path(path: &PathBuf) -> Result<(), &'static str> {
        if !path.exists() {
            return Err("file does not exits");
        }
        if !path.is_file() {
            return Err("provided path is not a file");
        }
        if fs::metadata(path).unwrap().len() == 0 {
            return Err("provided file cannot be empty");
        }
        Ok(())
    }

    fn validate_top(top: &u8) -> Result<(), &'static str> {
        if top < &1 {
            return Err("min \'top\' range equals 1");
        }
        if top > &100 {
            return Err("max \'top\' range equals 100");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Cli;
    use std::fs::write;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn validate_correct_fuzz_text() {
        let text: String = String::from("some text");
        let result = Cli::validate_fuzz_text(&text);
        assert_eq!(result, Ok(()))
    }

    #[test]
    fn validate_empty_fuzz_text() {
        let text: String = String::from("");
        let result = Cli::validate_fuzz_text(&text);
        assert_eq!(result, Err("fuzz text cannot be empty"))
    }

    #[test]
    fn validate_correct_file_path() {
        let file = NamedTempFile::new().unwrap();
        write(file.path(), "some content").unwrap();
        let result = Cli::validate_file_path(&file.path().to_path_buf());
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn validate_incorrect_file_path() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("missing.txt");
        let result = Cli::validate_file_path(&path);
        assert_eq!(result, Err("file does not exits"));
    }

    #[test]
    fn validate_incorrect_directory_path() {
        let dir = TempDir::new().unwrap();
        let result = Cli::validate_file_path(&dir.path().to_path_buf());
        assert_eq!(result, Err("provided path is not a file"));
    }

    #[test]
    fn validate_empty_file() {
        let file = NamedTempFile::new().unwrap();
        let result = Cli::validate_file_path(&file.path().to_path_buf());
        assert_eq!(result, Err("provided file cannot be empty"));
    }

    #[test]
    fn validate_correct_top_value() {
        let top: u8 = 50;
        let result = Cli::validate_top(&top);
        assert_eq!(result, Ok(()))
    }

    #[test]
    fn validate_too_high_top_value() {
        let top: u8 = 120;
        let result = Cli::validate_top(&top);
        assert_eq!(result, Err("max \'top\' range equals 100"))
    }

    #[test]
    fn validate_too_low_top_value() {
        let top: u8 = 0;
        let result = Cli::validate_top(&top);
        assert_eq!(result, Err("min \'top\' range equals 1"))
    }
}
