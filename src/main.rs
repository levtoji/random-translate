use clap::Parser;
use helper::CommandLineArgs;
use rand::seq::IndexedRandom;
use translators::{GoogleTranslator, Translator};

pub mod helper;

fn main() {
    let google_trans = GoogleTranslator::default();
    let languages = vec!["de", "en", "fr", "es", "br", "ru", "jp", "dt"];
    let mut translated_text = CommandLineArgs::parse().sentence;
    let mut chosen_language = *languages.choose(&mut rand::rng()).unwrap_or(&"de");
    let mut last_language = chosen_language;

    println!("Text to translate: {translated_text}");

    for _ in 1..10 {
        chosen_language = helper::remove_from_languages(&languages, &last_language)
            .choose(&mut rand::rng())
            .unwrap_or(&"de");
        translated_text = google_trans
            .translate_sync(&translated_text, "", chosen_language)
            .unwrap_or(String::from(translated_text));
        println!("{chosen_language} => {translated_text}");
        last_language = chosen_language;
    }
}
