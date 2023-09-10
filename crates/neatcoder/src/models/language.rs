use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Language {
    pub language: LanguageType,
    pub(crate) custom: Option<String>,
}

#[wasm_bindgen]
impl Language {
    #[wasm_bindgen(constructor)]
    pub fn new(language: LanguageType) -> Language {
        Self {
            language,
            custom: None,
        }
    }

    #[wasm_bindgen(js_name = newCustom)]
    pub fn new_custom(language: String) -> Language {
        Self {
            language: LanguageType::Custom,
            custom: Some(language),
        }
    }
}

impl Language {
    pub fn is_custom(&self) -> bool {
        match self.language {
            LanguageType::Custom => true,
            _ => false,
        }
    }

    pub fn name(&self) -> String {
        match self.language {
            LanguageType::Custom => self.custom.clone().expect(
                "Found Language Type Custom but without `custom` String field",
            ), // Safe to unwrap
            _ => format!("{}", self.language),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum LanguageType {
    Python,     // [".py", ".pyw", ".pyx", ".pyc"],
    Java,       // [".java", ".class", ".jar"],
    JavaScript, // [".js", ".jsx", ".mjs"],
    TypeScript, // [".ts", ".tsx"],
    Ruby,       // [".rb", ".rbw", ".rbs"],
    PHP,        // [".php", ".php3", ".php4", ".php5", ".php7", ".phtml"],
    C,          // [".c", ".h"],
    CPlusPLus,  // [".cpp", ".cc", ".cxx", ".h", ".hpp", ".hh", ".hxx"],
    CSharp,     // [".cs"],
    ObjectiveC, // [".m", ".mm", ".h"],
    Swift,      // [".swift"],
    Go,         // [".go"],
    Rust,       // [".rs", ".rlib"],
    Perl,       // [".pl", ".pm", ".t", ".pod"],
    Shell,      // [".sh", ".bash", ".zsh", ".csh", ".tcsh", ".ksh"],
    Scala,      // [".scala", ".sc"],
    Kotlin,     // [".kt", ".kts"],
    Lua,        // [".lua"],
    Haskell,    // [".hs", ".lhs"],
    HTML,       // [".html", ".htm"],
    MATLAB,     // [".m", ".mat", ".fig", ".mx", ".mlapp"],
    R,          // [".r", ".rdata", ".rds", ".rda"]
    Custom,
}

impl LanguageType {
    pub fn default_extension(&self) -> Option<&str> {
        match self {
            LanguageType::Python => Some(".py"),
            LanguageType::Java => Some(".java"),
            LanguageType::JavaScript => Some(".js"),
            LanguageType::TypeScript => Some(".ts"),
            LanguageType::Ruby => Some(".rb"),
            LanguageType::PHP => Some(".php"),
            LanguageType::C => Some(".c"),
            LanguageType::CPlusPLus => Some(".cpp"),
            LanguageType::CSharp => Some(".cs"),
            LanguageType::ObjectiveC => Some(".m"),
            LanguageType::Swift => Some(".swift"),
            LanguageType::Go => Some(".go"),
            LanguageType::Rust => Some(".rs"),
            LanguageType::Perl => Some(".pl"),
            LanguageType::Shell => Some(".sh"),
            LanguageType::Scala => Some(".scala"),
            LanguageType::Kotlin => Some(".kt"),
            LanguageType::Lua => Some(".lua"),
            LanguageType::Haskell => Some(".hs"),
            LanguageType::HTML => Some(".html"),
            LanguageType::MATLAB => Some(".m"),
            LanguageType::R => Some(".r"),
            LanguageType::Custom => None,
        }
    }
}

impl Display for LanguageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self {
            LanguageType::Python => "Python",
            LanguageType::Java => "Java",
            LanguageType::JavaScript => "JavaScript",
            LanguageType::TypeScript => "TypeScript",
            LanguageType::Ruby => "Ruby",
            LanguageType::PHP => "PHP",
            LanguageType::C => "C",
            LanguageType::CPlusPLus => "C++",
            LanguageType::CSharp => "C#",
            LanguageType::ObjectiveC => "Objective-C",
            LanguageType::Swift => "Swift",
            LanguageType::Go => "Go",
            LanguageType::Rust => "Rust",
            LanguageType::Perl => "Perl",
            LanguageType::Shell => "Shell",
            LanguageType::Scala => "Scala",
            LanguageType::Kotlin => "Kotlin",
            LanguageType::Lua => "Lua",
            LanguageType::Haskell => "Haskell",
            LanguageType::HTML => "HTML",
            LanguageType::MATLAB => "MATLAB",
            LanguageType::R => "R",
            LanguageType::Custom => "Custom",
        };

        f.write_str(tag)
    }
}

// This is implemented outside the impl block because abstract data structs
// are not supported in javascript
#[wasm_bindgen(js_name = languageTypeFromFriendlyUX)]
pub fn language_type_from_friendly_ux(lang: String) -> LanguageType {
    let api = match lang.as_str() {
        "Python" => LanguageType::Python,
        "Java" => LanguageType::Java,
        "JavaScript" => LanguageType::JavaScript,
        "TypeScript" => LanguageType::TypeScript,
        "Ruby" => LanguageType::Ruby,
        "PHP" => LanguageType::PHP,
        "C" => LanguageType::C,
        "C++" => LanguageType::CPlusPLus,
        "C#" => LanguageType::CSharp,
        "Objective-C" => LanguageType::ObjectiveC,
        "Swift" => LanguageType::Swift,
        "Go" => LanguageType::Go,
        "Rust" => LanguageType::Rust,
        "Perl" => LanguageType::Perl,
        "Shell" => LanguageType::Shell,
        "Scala" => LanguageType::Scala,
        "Kotlin" => LanguageType::Kotlin,
        "Lua" => LanguageType::Lua,
        "Haskell" => LanguageType::Haskell,
        "HTML" => LanguageType::HTML,
        "MATLAB" => LanguageType::MATLAB,
        "R" => LanguageType::R,
        _ => LanguageType::Custom,
    };
    api
}
