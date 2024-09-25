use std::borrow::Cow;
use std::char;
use std::collections::HashSet;
use std::env;
use std::io;
use std::io::BufRead;

extern crate regex;

use regex::{Captures, Regex};

#[derive(Copy, Clone)]
enum Script {
    Devanagari,
    Bengali,
    Tamil,
    Telugu,
    Gujarati,
    Gurmukhi,
    Oriya,
    Malayalam,
    Myanmar,
    Kannada,
    Sinhala,
}

enum Escape {
    None,
    Json,
    Html,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: corpus SCRIPT [none|json|html]");
        return;
    }
    if let Some(script) = get_script(&args[1]) {
        if let Some(escape) = get_escape(&args[2]) {
            let stdin = io::stdin();
            let mut set = HashSet::new();
            let json_re = Regex::new(r"[^\\]\\u([0-9a-fA-F]{4})").unwrap();
            let html_re = Regex::new(r"&#([0-9]*);|&zwj;|&zwnj;").unwrap();
            for res in stdin.lock().lines() {
                let line = res.unwrap_or(String::from("")); // ignore invalid UTF-8
                let line = match escape {
                    Escape::None => Cow::from(line),
                    Escape::Json => {
                        json_re.replace_all(&line, |caps: &Captures| {
                            let ds: Vec<u32> =
                                caps[1].chars().map(|c| c.to_digit(16).unwrap()).collect();
                            let u = (ds[0] << 12) | (ds[1] << 8) | (ds[2] << 4) | ds[3];
                            let c = char::from_u32(u).unwrap_or(' '); // to catch surrogates
                            c.to_string()
                        })
                    }
                    Escape::Html => {
                        html_re.replace_all(&line, |caps: &Captures| {
                            match &caps[0] {
                                "&zwj;" => '\u{200D}'.to_string(),
                                "&zwnj;" => '\u{200C}'.to_string(),
                                _ => {
                                    let u = caps[1].parse::<u32>().unwrap();
                                    let c = char::from_u32(u).unwrap_or(' '); // to catch surrogates
                                    c.to_string()
                                }
                            }
                        })
                    }
                };
                for word in line
                    .split(|c| !char_of_interest(script, c))
                    .filter(|w| cool_word(script, w))
                    .map(make_word)
                {
                    set.insert(word);
                }
            }
            let mut words: Vec<String> = set.drain().collect();
            words.sort();
            for word in words {
                println!("{}", word);
            }
        } else {
            println!("unknown escape");
        }
    } else {
        println!("unknown script");
    }
}

fn get_script(s: &str) -> Option<Script> {
    match s {
        "hi" => Some(Script::Devanagari),
        "bn" => Some(Script::Bengali),
        "ta" => Some(Script::Tamil),
        "te" => Some(Script::Telugu),
        "gu" => Some(Script::Gujarati),
        "pa" => Some(Script::Gurmukhi),
        "or" => Some(Script::Oriya),
        "ml" => Some(Script::Malayalam),
        "my" => Some(Script::Myanmar),
        "kn" => Some(Script::Kannada),
        "si" => Some(Script::Sinhala),
        _ => None,
    }
}

fn get_escape(s: &str) -> Option<Escape> {
    match s {
        "none" => Some(Escape::None),
        "json" => Some(Escape::Json),
        "html" => Some(Escape::Html),
        _ => None,
    }
}

fn char_of_interest(script: Script, c: char) -> bool {
    indic_script_char(script, c) || latin_combining_char(c)
}

fn indic_script_char(script: Script, c: char) -> bool {
    match script {
        Script::Devanagari => devanagari_char(c) || vedic_extensions_char(c) || misc_char(c),
        Script::Bengali => {
            bengali_char(c)
                || vedic_extensions_char(c)
                || devanagari_anudatta_char(c)
                || misc_char(c)
        }
        Script::Tamil => {
            tamil_char(c)
                || grantha_marks_char(c)
                || vedic_extensions_char(c)
                || devanagari_anudatta_char(c)
                || misc_char(c)
        }
        Script::Telugu => {
            telugu_char(c)
                || vedic_extensions_char(c)
                || devanagari_anudatta_char(c)
                || misc_char(c)
        }
        Script::Gujarati => {
            gujarati_char(c)
                || vedic_extensions_char(c)
                || devanagari_anudatta_char(c)
                || misc_char(c)
        }
        Script::Gurmukhi => {
            gurmukhi_char(c)
                || vedic_extensions_char(c)
                || devanagari_anudatta_char(c)
                || misc_char(c)
        }
        Script::Oriya => {
            oriya_char(c) || vedic_extensions_char(c) || devanagari_anudatta_char(c) || misc_char(c)
        }
        Script::Malayalam => {
            malayalam_char(c)
                || vedic_extensions_char(c)
                || devanagari_anudatta_char(c)
                || misc_char(c)
        }
        Script::Myanmar => myanmar_char(c) || vedic_extensions_char(c) || myanmar_misc_char(c),
        Script::Kannada => {
            kannada_char(c)
                || vedic_extensions_char(c)
                || devanagari_anudatta_char(c)
                || misc_char(c)
        }
        Script::Sinhala => sinhala_char(c) || vedic_extensions_char(c) || misc_char(c),
    }
}

fn latin_combining_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0x300 && cp <= 0x36F
}

fn script_specific_char(script: Script, c: char) -> bool {
    match script {
        Script::Devanagari => devanagari_char(c),
        Script::Bengali => bengali_char(c),
        Script::Tamil => tamil_char(c),
        Script::Telugu => telugu_char(c),
        Script::Gujarati => gujarati_char(c),
        Script::Gurmukhi => gurmukhi_char(c),
        Script::Oriya => oriya_char(c),
        Script::Malayalam => malayalam_char(c),
        Script::Myanmar => myanmar_char(c),
        Script::Kannada => kannada_char(c),
        Script::Sinhala => sinhala_char(c),
    }
}

fn bengali_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0x980 && cp <= 0x9FF
}

fn devanagari_char(c: char) -> bool {
    let cp = c as u32;
    (cp >= 0x900 && cp <= 0x97F) || (cp >= 0xA8E0 && cp <= 0xA8FF)
}

fn gujarati_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0xA80 && cp <= 0xAFF
}

fn gurmukhi_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0xA00 && cp <= 0xA7F
}

fn kannada_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0xC80 && cp <= 0xCFF
}

fn malayalam_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0xD00 && cp <= 0xD7F
}

fn myanmar_char(c: char) -> bool {
    // U+1000..U+109F Myanmar block
    // U+AA60..U+AA7F Myanmar Extended A
    // U+A9E0..U+A9FF Myanmar Extended B
    matches!(c,
        '\u{1000}'..='\u{109f}' | '\u{AA60}' ..= '\u{AA7F}' | '\u{A9E0}' ..= '\u{A9FF}'
    )
}

fn myanmar_misc_char(c: char) -> bool {
    // match c {
    //     '\u{00A0}' => true, // No-break space
    //     '\u{200C}' => true, // Zero-width non-joiner
    //     '\u{200D}' => true, //‍ Zero-width joiner
    //     '\u{2010}' => true, // Hyphen
    //     '\u{2011}' => true, // No-break hyphen
    //     '\u{2012}' => true, // Figure dash
    //     '\u{2013}' => true, // En dash
    //     '\u{2014}' => true, // Em dash
    //     '\u{25CC}' => true, // Dotted circle
    //     _ => false,
    // }

    c == '\u{200C}' || c == '\u{200D}' || c == '\u{25CC}' // zwnj, zwj, dotted circle
}

fn oriya_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0xB00 && cp <= 0xB7F
}

fn sinhala_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0xD70 && cp <= 0xDFF
}

fn tamil_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0xB80 && cp <= 0xBFF
}

fn telugu_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0xC00 && cp <= 0xC7F
}

fn grantha_marks_char(c: char) -> bool {
    let cp = c as u32;
    cp == 0x11301 || cp == 0x11303 || cp == 0x1133C
}

fn vedic_extensions_char(c: char) -> bool {
    let cp = c as u32;
    cp >= 0x1CD0 && cp <= 0x1CFF
}

fn devanagari_anudatta_char(c: char) -> bool {
    let cp = c as u32;
    cp == 0x951 || cp == 0x952 // Devanagari udatta and anudatta
}

fn misc_char(c: char) -> bool {
    let cp = c as u32;
    cp == 0x951 || cp == 0x952 || // Devanagari udatta and anudatta
    cp == 0x200C || cp == 0x200D || cp == 0x25CC // zwnj, zwj, dotted circle
}

fn cool_word(script: Script, word: &str) -> bool {
    word.chars().any(|c| script_specific_char(script, c))
}

fn make_word(s: &str) -> String {
    String::from(s.trim_start_matches(latin_combining_char))
}

/*
fn break_char(c: char) -> bool {
    c.is_ascii() ||
    c == '‘' || c == '’' ||
    c == '“' || c == '”' ||
    c == '«' || c == '»' ||
    c == '©' || c == '®' || c == '™' ||
    c == '–' || c == '—' || // en dash and em dash
    c == '£' || c == '¤' || c == '¥' || c == '§' ||
    c == '±' || c == '°' || c == '·' || c == '×' || c == '÷' ||
    c == '\u{A0}' || // nbsp
    c == '\u{AD}' || // shy
    c == '\u{200b}' // zwsp
}

fn chop(s: &str) -> &str {
    s.trim_matches(boring_char)
}

fn boring_char(c: char) -> bool {
    c.is_ascii()
}

fn non_empty(word: &&str) -> bool {
    !word.is_empty()
}
*/
