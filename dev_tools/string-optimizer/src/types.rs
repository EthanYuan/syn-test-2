use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TextInfo {
    original: String,
    editable: String,
    lang: Language,
    metadata: Meta,
}

impl TextInfo {
    pub fn new(original: String, lang: Language, metadata: Meta) -> Self {
        TextInfo {
            original: original.to_owned(),
            editable: original,
            lang,
            metadata,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Meta {
    category: Category,
    file: PathBuf,
    code_line: usize,
}

impl Meta {
    pub fn new(category: Category, file: PathBuf, code_line: usize) -> Self {
        Meta {
            category,
            file,
            code_line,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Category {
    ClapHelp,
    ClapAbout,

    LogDebug,
    LogWarn,
    LogInfo,
    LogError,
    LogTrace,

    StdOutput,
    StdError,

    ThisError,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "help" => Ok(Category::ClapHelp),
            "about" => Ok(Category::ClapAbout),
            "debug" => Ok(Category::LogDebug),
            "warn" => Ok(Category::LogWarn),
            "info" => Ok(Category::LogInfo),
            "error" => Ok(Category::LogError),
            "trace" => Ok(Category::LogTrace),
            "println" => Ok(Category::StdOutput),
            "eprintln" => Ok(Category::StdError),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Language {
    EN,
}
