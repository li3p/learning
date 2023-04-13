#[allow(dead_code)]
enum Language {
    Rust,
    TypeScript,
    Elixir,
    Haskell,
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        match self {
            Language::Rust => "Rust",
            Language::TypeScript => "TypeScript",
            Language::Elixir => "Elixir",
            Language::Haskell => "Haskell",
        }
    }
}

fn print_ref(v: impl AsRef<str>) {
    println!("v: {}", v.as_ref());
}

fn main() {
    let lang = Language::Rust;

    // str implements AsRef<str>
    print_ref("Hello world!");

    // String implements AsRef<str>
    print_ref(String::from("Hello world!"));

    // custom type Language implements AsRef<str>
    print_ref(lang);
}
