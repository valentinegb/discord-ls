use std::{
    fmt::{self, Formatter},
    str::FromStr,
};

pub(super) enum Language {
    Rust,
    Toml,
    Nix,
    Json,
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.rsplit_once('.').ok_or("string missing extension")?.1 {
            "rs" => Ok(Self::Rust),
            "toml" => Ok(Self::Toml),
            "nix" => Ok(Self::Nix),
            "json" | "jsonc" => Ok(Self::Json),
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
                Language::Nix => "Nix",
                Language::Json => "JSON",
            },
        )
    }
}
