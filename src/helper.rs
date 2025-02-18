use clap::Parser;

pub fn remove_from_languages(languages: &Vec<&'static str>, language: &str) -> Vec<&'static str> {
    let mut c = languages.clone();
    c.retain(|x| !language.contains(x));
    c
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineArgs {
    /// Sentence to translate
    #[arg(short, long)]
    pub sentence: String,
}
