use std::{
    fmt::{self, Formatter},
    str::FromStr,
};

pub(super) enum Language {
    Rust,
    Toml,
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.rsplit_once('.').ok_or("string missing extension")?.1 {
            "rs" => Ok(Self::Rust),
            "toml" => Ok(Self::Toml),
            _ => Err("no known extensions matched string".to_string()),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Rust => "Rust",
                Language::Toml => "TOML",
            },
        )
    }
}
