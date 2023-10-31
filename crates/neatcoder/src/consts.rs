///< Constants used throughout the application.
pub const CONFIG_EXTENSIONS: [&str; 15] = [
    "txt", "json", "toml", "lock", "yaml", "gemspec", "xml", "gradle",
    "csproj", "config", "sln", "mod", "sbt", "cabal", "md",
];

pub const CONFIG_FILES: [&str; 9] = [
    "Pipfile",       // Python
    "Gemfile",       // Ruby
    "cpanfile",      // Perl
    "Makefile.PL",   // Perl
    "Build.PL",      // Perl
    "DESCRIPTION",   // R
    "NAMESPACE",     // R
    "Package.swift", // Swift
    ".gitignore",
];
