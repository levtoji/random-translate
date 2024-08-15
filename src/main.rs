use rand::seq::SliceRandom;
use translators::{GoogleTranslator, Translator};

fn subtract(a: &Vec<&'static str>, b: &Vec<&str>) -> Vec<&'static str> {
    let mut c = a.clone();
    c.retain(|x| !b.contains(x));
    c
}

#[tokio::main]
async fn main() {
    let google_trans = GoogleTranslator::default();
    let languages = vec!["de", "en", "fr", "es", "br", "ru", "jp"];
    let mut translated_text = String::from("Ich bin ein Taugenichts");
    let mut chosen_language = *languages.choose(&mut rand::thread_rng()).unwrap_or(&"de");
    let mut last_language = chosen_language;

    println!(
        "Zu übersetzender Text:
         {translated_text}"
    );

    for _ in 1..10 {
        chosen_language = subtract(&languages, &vec![last_language])
            .choose(&mut rand::thread_rng())
            .unwrap_or(&"de");
        translated_text = google_trans
            .translate_async(&translated_text, "", chosen_language)
            .await
            .unwrap_or_default();
        println!("{chosen_language} => {translated_text}");
        last_language = chosen_language;
    }
}
