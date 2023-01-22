use std::io;
use std::time::Instant;

extern crate whatlang;
extern crate lingua;

use whatlang::{detect}; // Lang, Script
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};
use lingua::Language::{English, French, German, Spanish, Polish, Persian, Portuguese, Japanese, Latin, Lithuanian, Russian};

fn main() {
    let mut user_input = String::new();


    println!("Enter the text that you want to detect the language of: ");
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();

    let _txt_pt_br: &str = "Bom dia. Tudo bem com você?";
    let _txt_eng_us: &str = "Thank you very much.";
    let _txt_latin: &str = "Lorem ipsum";
    let languages = vec![English, French, German, Spanish, Portuguese, Polish, Persian, Japanese, Latin, Lithuanian, Russian];
    
    
    //// WhatLang lib
    let start_time_whatlang: Instant = Instant::now();
    println!("Starting detection with WhatLang...");
    let info = detect(&user_input).unwrap();
    //assert_eq!(info.lang(), Lang::Por);
    //assert_eq!(info.script(), Script::Latin);
    //assert_eq!(info.confidence(), 1.0);

    //assert!(info.is_reliable());
    println!("WhatLang thinks the text language is: {:#?}", info.lang().name());
    println!("WhatLang has {:#?} out of 1 confidence in this result", info.confidence());
    println!("Does WhatLang thinks this result is reliable: {:#?}", if info.is_reliable() { "Yes." } else { "No." });

    let end_time_whatlang: Instant = Instant::now();

    println!("Total execution of WhatLang: {:#?}", (end_time_whatlang - start_time_whatlang));

    //// Lingua Lib
    let start_time_lingua: Instant = Instant::now();
    println!("Starting detection with Lingua...");

    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();
    let detected_language: Option<Language> = detector.detect_language_of(&user_input);
    let confidence_values: Vec<(Language, f64)> = detector.compute_language_confidence_values(
        &user_input
    );

    // assert_eq!(detected_language, Some(English));

    println!("Lingua thinks the text language is: {:#?}", detected_language.unwrap());
    println!("Lingua detection confidence level (out of 1): {:#?}", confidence_values[0]);
    // println!("Does WhatLang thinks this result is reliable: {}", if info.is_reliable() { "Yes." } else { "No" });


    let end_time_lingua: Instant = Instant::now();
    println!("Total execution of WhatLang: {:#?}", (end_time_lingua - start_time_lingua));

}
