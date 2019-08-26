use std::collections::{HashSet, HashMap};
use std::cmp;
use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let mut set = HashSet::new();
    let mut bad = HashMap::new();
    for res in stdin.lock().lines() {
        let line = res.unwrap();
        for res in SyllableIter::new(&line) {
            match res {
                Ok(s) => { set.insert(s); }
                Err(s) => {
                    bad.insert(s, line.clone());
                }
            }
        }
    }
    let mut syllables: Vec<String> = set.drain().collect();
    syllables.sort();
    for s in syllables {
        println!("{}", s);
    }
    let mut bad: Vec<(String, String)> = bad.drain().collect();
    bad.sort();
    for (ref s, ref line) in bad {
        println!("bad: {:?} {} in line: {}", s, friendly(s), friendly(line));
    }
}

fn friendly(s: &str) -> String {
    let v: Vec<String> = s.chars().map(char_to_string).map(|s| {
        let mut ss = String::new();
        ss.push('[');
        ss.push_str(&s);
        ss.push(']');
        ss
    }).collect();
    v.as_slice().join(" + ")
}

fn char_to_string(c: char) -> String {
    match indic_name(c) {
        Some(s) => String::from(s),
        None => format!("U+{:X}", c as u32),
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ShapingClass {
    Bindu,
    Visarga,
    Avagraha,
    Nukta,
    Virama,
    Cantillation,
    GeminationMark,
    PureKiller,
    SyllableModifier,
    Consonant,
    VowelIndependent,
    VowelDependent,
    ConsonantDead,
    ConsonantMedial,
    ConsonantPlaceholder,
    ConsonantWithStacker,
    ConsonantPreRepha,
    ModifyingLetter,
    Placeholder,
    Number,
    Symbol,
    Joiner,
    NonJoiner,
    DottedCircle,
}

#[derive(Copy, Clone, Debug)]
enum MarkPlacementSubclass {
    TopPosition,
    RightPosition,
    BottomPosition,
    LeftPosition,
    LeftAndRightPosition,
    TopAndRightPosition,
    TopAndLeftPosition,
    TopLeftAndRightPosition,
    TopAndBottomPosition,
    Overstruck,
}

#[derive(Copy, Clone, Debug)]
enum Syllable {
    Consonant,
    Vowel,
    Standalone,
    Symbol,
    Broken,
}

fn shaping_class(ch: char) -> Option<ShapingClass> {
    let (opt_shaping, _) = indic_character(ch);
    opt_shaping
}

fn consonant(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::Consonant) => !ra(ch),
        Some(ShapingClass::ConsonantDead) => true,
        _ => false,
    }
}

fn vowel(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::VowelIndependent) => true,
        _ => false,
    }
}

fn nukta(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::Nukta) => true,
        _ => false,
    }
}

fn halant(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::Virama) => true,
        _ => false,
    }
}

fn zwj(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::Joiner) => true,
        _ => false,
    }
}

fn zwnj(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::NonJoiner) => true,
        _ => false,
    }
}

fn ra(ch: char) -> bool {
    match ch {
        '\u{0930}' => true, // Devanagari
        '\u{09B0}' => true, // Bengali
        '\u{09F0}' => true, // Bengali, Assamese
        '\u{0A30}' => true, // Gurmukhi
        '\u{0AB0}' => true, // Gujarati
        '\u{0B30}' => true, // Oriya
        '\u{0BB0}' => true, // Tamil
        '\u{0C30}' => true, // Telugu
        '\u{0CB0}' => true, // Kannada
        '\u{0D30}' => true, // Malayalam
        '\u{0DBB}' => true, // Sinhala
        _ => false,
    }
}

fn matra(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::VowelDependent) => true,
        Some(ShapingClass::PureKiller) => true,
        _ => false,
    }
}

fn syllable_modifier(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::SyllableModifier) => true,
        Some(ShapingClass::Bindu) => true,
        Some(ShapingClass::Visarga) => true,
        Some(ShapingClass::GeminationMark) => true,
        _ => false,
    }
}

fn vedic_sign(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::Cantillation) => true,
        _ => false,
    }
}

fn placeholder(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::Placeholder) => true,
        Some(ShapingClass::ConsonantPlaceholder) => true,
        _ => false,
    }
}

fn dotted_circle(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::DottedCircle) => true,
        _ => false,
    }
}

fn repha(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::ConsonantPreRepha) => true,
        _ => false,
    }
}

fn consonant_medial(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::ConsonantMedial) => true,
        _ => false,
    }
}

fn symbol(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::Symbol) => true,
        _ => false,
    }
}

fn avagraha(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::Avagraha) => true,
        _ => false,
    }
}

fn consonant_with_stacker(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::ConsonantWithStacker) => true,
        _ => false,
    }
}

fn other(ch: char) -> bool {
    match shaping_class(ch) {
        // FIXME Some(ShapingClass::Other) => true,
        Some(ShapingClass::Number) => true,
        Some(ShapingClass::ModifyingLetter) => true,
        _ => false,
    }
}

fn match_unit(_cs: &[char]) -> Option<usize> {
    Some(0)
}

fn match_one(cs: &[char], f: impl FnOnce(char) -> bool) -> Option<usize> {
    if !cs.is_empty() && f(cs[0]) {
        Some(1)
    } else {
        None
    }
}

fn match_nonempty(cs: &[char], f: impl FnOnce(&[char]) -> Option<usize>) -> Option<usize> {
    match f(cs) {
        Some(n) if n > 0 => Some(n),
        _ => None,
    }
}

fn match_optional(cs: &[char], f: impl FnOnce(&[char]) -> Option<usize>) -> Option<usize> {
    if let Some(n) = f(cs) {
        Some(n)
    } else {
        Some(0)
    }
}

fn match_optional_seq(
    cs: &[char],
    f: impl FnOnce(&[char]) -> Option<usize>,
    g: impl Copy + Fn(&[char]) -> Option<usize>,
) -> Option<usize> {
    match_either(cs, g, |cs| match_seq(cs, f, g))
}

fn match_repeat_num(
    mut cs: &[char],
    num: usize,
    f: &impl Fn(&[char]) -> Option<usize>,
) -> Option<usize> {
    let mut total: usize = 0;
    for _i in 0..num {
        if let Some(n) = f(cs) {
            total += n;
            cs = &cs[n..];
        } else {
            return None;
        }
    }
    Some(total)
}

fn match_repeat_upto(
    cs: &[char],
    max: usize,
    f: impl Fn(&[char]) -> Option<usize>,
    g: impl Fn(&[char]) -> Option<usize>,
) -> Option<usize> {
    let mut best = None;

    for i in 0..=max {
        if let Some(nf) = match_repeat_num(cs, i, &f) {
            if let Some(ng) = g(&cs[nf..]) {
                best = Some(nf + ng)
            }
        }
    }
    best
}

fn match_seq(
    cs: &[char],
    f1: impl FnOnce(&[char]) -> Option<usize>,
    f2: impl FnOnce(&[char]) -> Option<usize>,
) -> Option<usize> {
    if let Some(n1) = f1(cs) {
        if let Some(n2) = f2(&cs[n1..]) {
            Some(n1 + n2)
        } else {
            None
        }
    } else {
        None
    }
}

fn match_either(
    cs: &[char],
    f1: impl FnOnce(&[char]) -> Option<usize>,
    f2: impl FnOnce(&[char]) -> Option<usize>,
) -> Option<usize> {
    let res1 = f1(cs);
    let res2 = f2(cs);
    match (res1, res2) {
        (Some(n1), Some(n2)) => Some(cmp::max(n1, n2)),
        (Some(n1), None) => Some(n1),
        (None, Some(n2)) => Some(n2),
        (None, None) => None,
    }
}

fn match_either_seq(
    cs: &[char],
    f1: impl FnOnce(&[char]) -> Option<usize>,
    f2: impl FnOnce(&[char]) -> Option<usize>,
    g: impl Copy + Fn(&[char]) -> Option<usize>,
) -> Option<usize> {
    let res1 = match_seq(cs, f1, &g);
    let res2 = match_seq(cs, f2, &g);
    match (res1, res2) {
        (Some(n1), Some(n2)) => Some(cmp::max(n1, n2)),
        (Some(n1), None) => Some(n1),
        (None, Some(n2)) => Some(n2),
        (None, None) => None,
    }
}

fn match_c(cs: &[char]) -> Option<usize> {
    match_either(cs, |cs| match_one(cs, consonant), |cs| match_one(cs, ra))
}

fn match_z(cs: &[char]) -> Option<usize> {
    match_either(cs, |cs| match_one(cs, zwj), |cs| match_one(cs, zwnj))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_reph(cs: &[char]) -> Option<usize> {
    match_either(
        cs,
        |cs| match_seq(cs, |cs| match_one(cs, ra), |cs| match_one(cs, halant)),
        |cs| match_one(cs, repha),
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_cn(cs: &[char]) -> Option<usize> {
    match_seq(cs,
        match_c,
        |cs| match_optional_seq(cs,
            |cs| match_one(cs, zwj),
            |cs| match_optional(cs, |cs| match_one(cs, nukta)),
        )
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_forced_rakar(cs: &[char]) -> Option<usize> {
    match_seq(
        cs,
        |cs| match_one(cs, zwj),
        |cs| {
            match_seq(
                cs,
                |cs| match_one(cs, halant),
                |cs| match_seq(cs, |cs| match_one(cs, zwj), |cs| match_one(cs, ra)),
            )
        },
    )
}

fn match_s(cs: &[char]) -> Option<usize> {
    match_seq(
        cs,
        |cs| match_one(cs, symbol),
        |cs| match_optional(cs, |cs| match_one(cs, nukta)),
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_matra_group(cs: &[char]) -> Option<usize> {
    match_repeat_upto(cs, 3, match_z,
        |cs| {
            match_seq(
                cs,
                |cs| match_one(cs, matra),
                |cs| match_optional_seq(cs,
                    |cs| match_one(cs, nukta),
                    |cs| {
                        match_optional(cs, |cs| {
                            match_either(cs, |cs| match_one(cs, halant), match_forced_rakar)
                        })
                    },
                )
            )
        },
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_syllable_tail(cs: &[char]) -> Option<usize> {
    match_optional_seq(cs,
        |cs| match_optional_seq(cs,
            match_z,
            |cs| match_seq(cs,
                |cs| match_one(cs, syllable_modifier),
                |cs| match_optional_seq(cs,
                    |cs| match_one(cs, syllable_modifier),
                    |cs| match_optional(cs, |cs| match_one(cs, zwnj)),
                )
            )
        ),
        |cs| match_repeat_upto(cs, 3,
            |cs| match_one(cs, avagraha),
            |cs| match_repeat_upto(cs, 2, |cs| match_one(cs, vedic_sign), match_unit),
        )
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_halant_group(cs: &[char]) -> Option<usize> {
    match_optional_seq(cs,
        match_z,
        |cs| match_seq(cs,
            |cs| match_one(cs, halant),
            |cs| match_optional(cs,
                |cs| match_seq(cs,
                    |cs| match_one(cs, zwj),
                    |cs| match_optional(cs, |cs| match_one(cs, nukta)),
                )
            )
        )
    )
}

// This is not used as we expand it inline
/*
fn match_final_halant_group(cs: &[char]) -> Option<usize> {
    match_either(cs,
        match_halant_group,
        |cs| match_seq(cs,
            |cs| match_one(cs, halant),
            |cs| match_one(cs, zwnj)))
}
*/

fn match_medial_group(cs: &[char]) -> Option<usize> {
    match_optional(cs, |cs| match_one(cs, consonant_medial))
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_halant_or_matra_group(cs: &[char]) -> Option<usize> {
    // this can match a short sequence so we expand and reorder it
    match_either(cs,
        |cs| match_seq(cs,
            |cs| match_one(cs, halant),
            |cs| match_one(cs, zwnj)),
        // Currently deviates from spec. See:
        // https://github.com/n8willis/opentype-shaping-documents/issues/72
        |cs| match_either(cs,
            |cs| match_repeat_upto(cs, 4, match_matra_group, match_unit),
            match_halant_group,
        )
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_consonant_syllable(cs: &[char]) -> Option<usize> {
    match_optional_seq(cs,
        |cs| match_either(cs,
            |cs| match_one(cs, repha),
            |cs| match_one(cs, consonant_with_stacker),
        ),
        |cs| match_repeat_upto(cs, 4,
            |cs| match_seq(cs,
                match_cn,
                match_halant_group
            ),
            |cs| match_seq(cs, match_cn,
                |cs| match_seq(cs,
                    match_medial_group,
                    |cs| match_seq(cs,
                        match_halant_or_matra_group,
                        match_syllable_tail
                    )
                )
            )
        )
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_vowel_syllable(cs: &[char]) -> Option<usize> {
    match_optional_seq(cs,
        match_reph,
        |cs| match_seq(cs,
            |cs| match_one(cs, vowel),
            |cs| match_optional_seq(cs,
                |cs| match_one(cs, nukta),
                |cs| match_either(cs,
                    |cs| match_one(cs, zwj),
                    |cs| match_repeat_upto(cs, 4,
                        |cs| match_seq(cs,
                            match_halant_group,
                            match_cn),
                        |cs| match_seq(cs,
                            match_medial_group,
                            |cs| match_seq(cs,
                                match_halant_or_matra_group,
                                match_syllable_tail,
                            )
                        )
                    )
                )
            )
        )
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_standalone_syllable(cs: &[char]) -> Option<usize> {
    match_either_seq(cs,
        |cs| match_optional_seq(cs,
            |cs| match_either(cs,
                    |cs| match_one(cs, repha),
                    |cs| match_one(cs, consonant_with_stacker),
            ),
            |cs| match_one(cs, placeholder)
        ),
        |cs| match_seq(cs,
            |cs| match_optional(cs, match_reph),
            |cs| match_one(cs, dotted_circle),
        ),
        |cs| match_optional_seq(cs,
            |cs| match_one(cs, nukta),
            |cs| match_repeat_upto(cs, 4,
                |cs| match_seq(cs,
                    match_halant_group,
                    match_cn
                ),
                |cs| match_seq(cs,
                    match_medial_group,
                    |cs| match_seq(cs,
                        match_halant_or_matra_group,
                        match_syllable_tail
                    )
                )
            )
        )
    )
}

fn match_symbol_syllable(cs: &[char]) -> Option<usize> {
    match_seq(cs, match_s, match_syllable_tail)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn match_broken_syllable(cs: &[char]) -> Option<usize> {
    match_nonempty(cs,
        |cs| match_optional_seq(cs,
            match_reph,
            |cs| match_optional_seq(cs,
                |cs| match_one(cs, nukta),
                |cs| match_repeat_upto(cs, 4,
                    |cs| match_seq(cs,
                        match_halant_group,
                        match_cn
                    ),
                    |cs| match_seq(cs,
                        match_medial_group,
                        |cs| match_seq(cs,
                            match_halant_or_matra_group,
                            match_syllable_tail
                        )
                    )
                )
            )
        )
    )
}

fn match_syllable(cs: &[char]) -> Option<(usize, Syllable)> {
    let consonant = (match_consonant_syllable(cs), Syllable::Consonant);
    let vowel = (match_vowel_syllable(cs), Syllable::Vowel);
    let standalone = (match_standalone_syllable(cs), Syllable::Standalone);
    let symbol = (match_symbol_syllable(cs), Syllable::Symbol);
    let broken = (match_broken_syllable(cs), Syllable::Broken);

    // To prevent incorrect splitting (and mis-categorisation) of a syllable,
    // greediest syllable match, wins. In the event of a tie, precedence is
    // consonant > vowel > standalone > symbol > broken
    let syllables = &mut [consonant, vowel, standalone, symbol, broken];
    syllables.sort_by(|(len1, _), (len2, _)| len2.cmp(len1));

    match syllables[0] {
        (Some(len), syllable_type) => Some((len, syllable_type)),
        (None, _) => None,
    }
}

struct SyllableIter {
    buf: Vec<char>,
    i: usize,
}

impl SyllableIter {
    pub fn new(s: &str) -> Self {
        SyllableIter {
            buf: s.chars().collect(),
            i: 0
        }
    }
}

impl Iterator for SyllableIter {
    type Item = Result<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = {
            let cs = &self.buf[self.i..];
            if cs.len() == 0 { return None; }
            match match_syllable(cs) {
                Some((len, _)) => {
                    assert_ne!(len, 0);
                    let s = cs[0..len].iter().collect();
                    self.i += len;
                    Some(Some(Ok(s)))
                }
                _ => {
                    self.i += 1;
                    if other(cs[0]) {
                        // ignore numbers and modifying letters
                        None
                    } else {
                        // let s = cs[0..1].iter().collect();
                        let s = cs.iter().collect();
                        Some(Some(Err(s)))
                    }
                }
            }
        };
        match res {
            Some(res) => res,
            None => self.next(),
        }
    }
}

fn indic_character(ch: char) -> (Option<ShapingClass>, Option<MarkPlacementSubclass>) {
    use MarkPlacementSubclass::*;
    use ShapingClass::*;

    match ch as u32 {
        // Devanagari character table
        0x0900 => (Some(Bindu), Some(TopPosition)),             // Inverted Candrabindu
        0x0901 => (Some(Bindu), Some(TopPosition)),             // Candrabindu
        0x0902 => (Some(Bindu), Some(TopPosition)),             // Anusvara
        0x0903 => (Some(Visarga), Some(RightPosition)),         // Visarga
        0x0904 => (Some(VowelIndependent), None),               // Short A
        0x0905 => (Some(VowelIndependent), None),               // A
        0x0906 => (Some(VowelIndependent), None),               // Aa
        0x0907 => (Some(VowelIndependent), None),               // I
        0x0908 => (Some(VowelIndependent), None),               // Ii
        0x0909 => (Some(VowelIndependent), None),               // U
        0x090A => (Some(VowelIndependent), None),               // Uu
        0x090B => (Some(VowelIndependent), None),               // Vocalic R
        0x090C => (Some(VowelIndependent), None),               // Vocalic L
        0x090D => (Some(VowelIndependent), None),               // Candra E
        0x090E => (Some(VowelIndependent), None),               // Short E
        0x090F => (Some(VowelIndependent), None),               // E
        0x0910 => (Some(VowelIndependent), None),               // Ai
        0x0911 => (Some(VowelIndependent), None),               // Candra O
        0x0912 => (Some(VowelIndependent), None),               // Short O
        0x0913 => (Some(VowelIndependent), None),               // O
        0x0914 => (Some(VowelIndependent), None),               // Au
        0x0915 => (Some(Consonant), None),                      // Ka
        0x0916 => (Some(Consonant), None),                      // Kha
        0x0917 => (Some(Consonant), None),                      // Ga
        0x0918 => (Some(Consonant), None),                      // Gha
        0x0919 => (Some(Consonant), None),                      // Nga
        0x091A => (Some(Consonant), None),                      // Ca
        0x091B => (Some(Consonant), None),                      // Cha
        0x091C => (Some(Consonant), None),                      // Ja
        0x091D => (Some(Consonant), None),                      // Jha
        0x091E => (Some(Consonant), None),                      // Nya
        0x091F => (Some(Consonant), None),                      // Tta
        0x0920 => (Some(Consonant), None),                      // Ttha
        0x0921 => (Some(Consonant), None),                      // Dda
        0x0922 => (Some(Consonant), None),                      // Ddha
        0x0923 => (Some(Consonant), None),                      // Nna
        0x0924 => (Some(Consonant), None),                      // Ta
        0x0925 => (Some(Consonant), None),                      // Tha
        0x0926 => (Some(Consonant), None),                      // Da
        0x0927 => (Some(Consonant), None),                      // Dha
        0x0928 => (Some(Consonant), None),                      // Na
        0x0929 => (Some(Consonant), None),                      // Nnna
        0x092A => (Some(Consonant), None),                      // Pa
        0x092B => (Some(Consonant), None),                      // Pha
        0x092C => (Some(Consonant), None),                      // Ba
        0x092D => (Some(Consonant), None),                      // Bha
        0x092E => (Some(Consonant), None),                      // Ma
        0x092F => (Some(Consonant), None),                      // Ya
        0x0930 => (Some(Consonant), None),                      // Ra
        0x0931 => (Some(Consonant), None),                      // Rra
        0x0932 => (Some(Consonant), None),                      // La
        0x0933 => (Some(Consonant), None),                      // Lla
        0x0934 => (Some(Consonant), None),                      // Llla
        0x0935 => (Some(Consonant), None),                      // Va
        0x0936 => (Some(Consonant), None),                      // Sha
        0x0937 => (Some(Consonant), None),                      // Ssa
        0x0938 => (Some(Consonant), None),                      // Sa
        0x0939 => (Some(Consonant), None),                      // Ha
        0x093A => (Some(VowelDependent), Some(TopPosition)),    // Sign Oe
        0x093B => (Some(VowelDependent), Some(RightPosition)),  // Sign Ooe
        0x093C => (Some(Nukta), Some(BottomPosition)),          // Nukta
        0x093D => (Some(Avagraha), None),                       // Avagraha
        0x093E => (Some(VowelDependent), Some(RightPosition)),  // Sign Aa
        0x093F => (Some(VowelDependent), Some(LeftPosition)),   // Sign I
        0x0940 => (Some(VowelDependent), Some(RightPosition)),  // Sign Ii
        0x0941 => (Some(VowelDependent), Some(BottomPosition)), // Sign U
        0x0942 => (Some(VowelDependent), Some(BottomPosition)), // Sign Uu
        0x0943 => (Some(VowelDependent), Some(BottomPosition)), // Sign Vocalic R
        0x0944 => (Some(VowelDependent), Some(BottomPosition)), // Sign Vocalic Rr
        0x0945 => (Some(VowelDependent), Some(TopPosition)),    // Sign Candra E
        0x0946 => (Some(VowelDependent), Some(TopPosition)),    // Sign Short E
        0x0947 => (Some(VowelDependent), Some(TopPosition)),    // Sign E
        0x0948 => (Some(VowelDependent), Some(TopPosition)),    // Sign Ai
        0x0949 => (Some(VowelDependent), Some(RightPosition)),  // Sign Candra O
        0x094A => (Some(VowelDependent), Some(RightPosition)),  // Sign Short O
        0x094B => (Some(VowelDependent), Some(RightPosition)),  // Sign O
        0x094C => (Some(VowelDependent), Some(RightPosition)),  // Sign Au
        0x094D => (Some(Virama), Some(BottomPosition)),         // Virama
        0x094E => (Some(VowelDependent), Some(LeftPosition)),   // Sign Prishthamatra E
        0x094F => (Some(VowelDependent), Some(RightPosition)),  // Sign Aw
        0x0950 => (None, None),                                 // Om
        0x0951 => (Some(Cantillation), Some(TopPosition)),      // Udatta
        0x0952 => (Some(Cantillation), Some(BottomPosition)),   // Anudatta
        0x0953 => (None, Some(TopPosition)),                    // Grave accent
        0x0954 => (None, Some(TopPosition)),                    // Acute accent
        0x0955 => (Some(VowelDependent), Some(TopPosition)),    // Sign Candra Long E
        0x0956 => (Some(VowelDependent), Some(BottomPosition)), // Sign Ue
        0x0957 => (Some(VowelDependent), Some(BottomPosition)), // Sign Uue
        0x0958 => (Some(Consonant), None),                      // Qa
        0x0959 => (Some(Consonant), None),                      // Khha
        0x095A => (Some(Consonant), None),                      // Ghha
        0x095B => (Some(Consonant), None),                      // Za
        0x095C => (Some(Consonant), None),                      // Dddha
        0x095D => (Some(Consonant), None),                      // Rha
        0x095E => (Some(Consonant), None),                      // Fa
        0x095F => (Some(Consonant), None),                      // Yya
        0x0960 => (Some(VowelIndependent), None),               // Vocalic Rr
        0x0961 => (Some(VowelIndependent), None),               // Vocalic Ll
        0x0962 => (Some(VowelDependent), Some(BottomPosition)), // Sign Vocalic L
        0x0963 => (Some(VowelDependent), Some(BottomPosition)), // Sign Vocalic Ll
        0x0964 => (None, None),                                 // Danda
        0x0965 => (None, None),                                 // Double Danda
        0x0966 => (Some(Number), None),                         // Digit Zero
        0x0967 => (Some(Number), None),                         // Digit One
        0x0968 => (Some(Number), None),                         // Digit Two
        0x0969 => (Some(Number), None),                         // Digit Three
        0x096A => (Some(Number), None),                         // Digit Four
        0x096B => (Some(Number), None),                         // Digit Five
        0x096C => (Some(Number), None),                         // Digit Six
        0x096D => (Some(Number), None),                         // Digit Seven
        0x096E => (Some(Number), None),                         // Digit Eight
        0x096F => (Some(Number), None),                         // Digit Nine
        0x0970 => (None, None),                                 // Abbreviation Sign
        0x0971 => (None, None),                                 // Sign High Spacing Dot
        0x0972 => (Some(VowelIndependent), None),               // Candra Aa
        0x0973 => (Some(VowelIndependent), None),               // Oe
        0x0974 => (Some(VowelIndependent), None),               // Ooe
        0x0975 => (Some(VowelIndependent), None),               // Aw
        0x0976 => (Some(VowelIndependent), None),               // Ue
        0x0977 => (Some(VowelIndependent), None),               // Uue
        0x0978 => (Some(Consonant), None),                      // Marwari Dda
        0x0979 => (Some(Consonant), None),                      // Zha
        0x097A => (Some(Consonant), None),                      // Heavy Ya
        0x097B => (Some(Consonant), None),                      // Gga
        0x097C => (Some(Consonant), None),                      // Jja
        0x097D => (Some(Consonant), None),                      // Glottal Stop
        0x097E => (Some(Consonant), None),                      // Ddda
        0x097F => (Some(Consonant), None),                      // Bba

        // Bengali character table
        0x0980 => (Some(Placeholder), None),                          // Anji
        0x0981 => (Some(Bindu), Some(TopPosition)),                   // Candrabindu
        0x0982 => (Some(Bindu), Some(RightPosition)),                 // Anusvara
        0x0983 => (Some(Visarga), Some(RightPosition)),               // Visarga
        0x0984 => (None, None),                                       // unassigned
        0x0985 => (Some(VowelIndependent), None),                     // A
        0x0986 => (Some(VowelIndependent), None),                     // Aa
        0x0987 => (Some(VowelIndependent), None),                     // I
        0x0988 => (Some(VowelIndependent), None),                     // Ii
        0x0989 => (Some(VowelIndependent), None),                     // U
        0x098A => (Some(VowelIndependent), None),                     // Uu
        0x098B => (Some(VowelIndependent), None),                     // Vocalic R
        0x098C => (Some(VowelIndependent), None),                     // Vocalic L
        0x098D => (None, None),                                       // unassigned
        0x098E => (None, None),                                       // unassigned
        0x098F => (Some(VowelIndependent), None),                     // E
        0x0990 => (Some(VowelIndependent), None),                     // Ai
        0x0991 => (None, None),                                       // unassigned
        0x0992 => (None, None),                                       // unassigned
        0x0993 => (Some(VowelIndependent), None),                     // O
        0x0994 => (Some(VowelIndependent), None),                     // Au
        0x0995 => (Some(Consonant), None),                            // Ka
        0x0996 => (Some(Consonant), None),                            // Kha
        0x0997 => (Some(Consonant), None),                            // Ga
        0x0998 => (Some(Consonant), None),                            // Gha
        0x0999 => (Some(Consonant), None),                            // Nga
        0x099A => (Some(Consonant), None),                            // Ca
        0x099B => (Some(Consonant), None),                            // Cha
        0x099C => (Some(Consonant), None),                            // Ja
        0x099D => (Some(Consonant), None),                            // Jha
        0x099E => (Some(Consonant), None),                            // Nya
        0x099F => (Some(Consonant), None),                            // Tta
        0x09A0 => (Some(Consonant), None),                            // Ttha
        0x09A1 => (Some(Consonant), None),                            // Dda
        0x09A2 => (Some(Consonant), None),                            // Ddha
        0x09A3 => (Some(Consonant), None),                            // Nna
        0x09A4 => (Some(Consonant), None),                            // Ta
        0x09A5 => (Some(Consonant), None),                            // Tha
        0x09A6 => (Some(Consonant), None),                            // Da
        0x09A7 => (Some(Consonant), None),                            // Dha
        0x09A8 => (Some(Consonant), None),                            // Na
        0x09A9 => (None, None),                                       // unassigned
        0x09AA => (Some(Consonant), None),                            // Pa
        0x09AB => (Some(Consonant), None),                            // Pha
        0x09AC => (Some(Consonant), None),                            // Ba
        0x09AD => (Some(Consonant), None),                            // Bha
        0x09AE => (Some(Consonant), None),                            // Ma
        0x09AF => (Some(Consonant), None),                            // Ya
        0x09B0 => (Some(Consonant), None),                            // Ra
        0x09B1 => (None, None),                                       // unassigned
        0x09B2 => (Some(Consonant), None),                            // La
        0x09B3 => (None, None),                                       // unassigned
        0x09B4 => (None, None),                                       // unassigned
        0x09B5 => (None, None),                                       // unassigned
        0x09B6 => (Some(Consonant), None),                            // Sha
        0x09B7 => (Some(Consonant), None),                            // Ssa
        0x09B8 => (Some(Consonant), None),                            // Sa
        0x09B9 => (Some(Consonant), None),                            // Ha
        0x09BA => (None, None),                                       // unassigned
        0x09BB => (None, None),                                       // unassigned
        0x09BC => (Some(Nukta), Some(BottomPosition)),                // Nukta
        0x09BD => (Some(Avagraha), None),                             // Avagraha
        0x09BE => (Some(VowelDependent), Some(RightPosition)),        // Sign Aa
        0x09BF => (Some(VowelDependent), Some(LeftPosition)),         // Sign I
        0x09C0 => (Some(VowelDependent), Some(RightPosition)),        // Sign Ii
        0x09C1 => (Some(VowelDependent), Some(BottomPosition)),       // Sign U
        0x09C2 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Uu
        0x09C3 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic R
        0x09C4 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic Rr
        0x09C5 => (None, None),                                       // unassigned
        0x09C6 => (None, None),                                       // unassigned
        0x09C7 => (Some(VowelDependent), Some(LeftPosition)),         // Sign E
        0x09C8 => (Some(VowelDependent), Some(LeftPosition)),         // Sign Ai
        0x09C9 => (None, None),                                       // unassigned
        0x09CA => (None, None),                                       // unassigned
        0x09CB => (Some(VowelDependent), Some(LeftAndRightPosition)), // Sign O
        0x09CC => (Some(VowelDependent), Some(LeftAndRightPosition)), // Sign Au
        0x09CD => (Some(Virama), Some(BottomPosition)),               // Virama
        0x09CE => (Some(ConsonantDead), None),                        // Khanda Ta
        0x09CF => (None, None),                                       // unassigned
        0x09D0 => (None, None),                                       // unassigned
        0x09D1 => (None, None),                                       // unassigned
        0x09D2 => (None, None),                                       // unassigned
        0x09D3 => (None, None),                                       // unassigned
        0x09D4 => (None, None),                                       // unassigned
        0x09D5 => (None, None),                                       // unassigned
        0x09D6 => (None, None),                                       // unassigned
        0x09D7 => (Some(VowelDependent), Some(RightPosition)),        // Au Length Mark
        0x09D8 => (None, None),                                       // unassigned
        0x09D9 => (None, None),                                       // unassigned
        0x09DA => (None, None),                                       // unassigned
        0x09DB => (None, None),                                       // unassigned
        0x09DC => (Some(Consonant), None),                            // Rra
        0x09DD => (Some(Consonant), None),                            // Rha
        0x09DE => (None, None),                                       // unassigned
        0x09DF => (Some(Consonant), None),                            // Yya
        0x09E0 => (Some(VowelIndependent), None),                     // Vocalic Rr
        0x09E1 => (Some(VowelIndependent), None),                     // Vocalic Ll
        0x09E2 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic L
        0x09E3 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic Ll
        0x09E4 => (None, None),                                       // unassigned
        0x09E5 => (None, None),                                       // unassigned
        0x09E6 => (Some(Number), None),                               // Digit Zero
        0x09E7 => (Some(Number), None),                               // Digit One
        0x09E8 => (Some(Number), None),                               // Digit Two
        0x09E9 => (Some(Number), None),                               // Digit Three
        0x09EA => (Some(Number), None),                               // Digit Four
        0x09EB => (Some(Number), None),                               // Digit Five
        0x09EC => (Some(Number), None),                               // Digit Six
        0x09ED => (Some(Number), None),                               // Digit Seven
        0x09EE => (Some(Number), None),                               // Digit Eight
        0x09EF => (Some(Number), None),                               // Digit Nine
        0x09F0 => (Some(Consonant), None),                            // Assamese Ra
        0x09F1 => (Some(Consonant), None),                            // Assamese Wa
        0x09F2 => (Some(Symbol), None),                               // Rupee Mark
        0x09F3 => (Some(Symbol), None),                               // Rupee Sign
        0x09F4 => (Some(Number), None),                               // Numerator One
        0x09F5 => (Some(Number), None),                               // Numerator Two
        0x09F6 => (Some(Number), None),                               // Numerator Three
        0x09F7 => (Some(Number), None),                               // Numerator Four
        0x09F8 => (Some(Number), None),                               // Numerator One Less Than Denominator
        0x09F9 => (Some(Number), None),                               // Denominator Sixteen
        0x09FA => (Some(Symbol), None),                               // Isshar
        0x09FB => (Some(Symbol), None),                               // Ganda Mark
        0x09FC => (None, None),                                       // Vedic Anusvara
        0x09FD => (None, None),                                       // Abbreviation Sign
        0x09FE => (Some(SyllableModifier), Some(TopPosition)),        // Sandhi Mark

        // Gurmukhi character table
        0x0A00 => (None, None),                                  // unassigned
        0x0A01 => (Some(Bindu), Some(TopPosition)),              // Adak Bindi
        0x0A02 => (Some(Bindu), Some(TopPosition)),              // Bindi
        0x0A03 => (Some(Visarga), Some(RightPosition)),          // Visarga
        0x0A04 => (None, None),                                  // unassigned
        0x0A05 => (Some(VowelIndependent), None),                // A
        0x0A06 => (Some(VowelIndependent), None),                // Aa
        0x0A07 => (Some(VowelIndependent), None),                // I
        0x0A08 => (Some(VowelIndependent), None),                // Ii
        0x0A09 => (Some(VowelIndependent), None),                // U
        0x0A0A => (Some(VowelIndependent), None),                // Uu
        0x0A0B => (None, None),                                  // unassigned
        0x0A0C => (None, None),                                  // unassigned
        0x0A0D => (None, None),                                  // unassigned
        0x0A0E => (None, None),                                  // unassigned
        0x0A0F => (Some(VowelIndependent), None),                // Ee
        0x0A10 => (Some(VowelIndependent), None),                // Ai
        0x0A11 => (None, None),                                  // unassigned
        0x0A12 => (None, None),                                  // unassigned
        0x0A13 => (Some(VowelIndependent), None),                // Oo
        0x0A14 => (Some(VowelIndependent), None),                // Au
        0x0A15 => (Some(Consonant), None),                       // Ka
        0x0A16 => (Some(Consonant), None),                       // Kha
        0x0A17 => (Some(Consonant), None),                       // Ga
        0x0A18 => (Some(Consonant), None),                       // Gha
        0x0A19 => (Some(Consonant), None),                       // Nga
        0x0A1A => (Some(Consonant), None),                       // Ca
        0x0A1B => (Some(Consonant), None),                       // Cha
        0x0A1C => (Some(Consonant), None),                       // Ja
        0x0A1D => (Some(Consonant), None),                       // Jha
        0x0A1E => (Some(Consonant), None),                       // Nya
        0x0A1F => (Some(Consonant), None),                       // Tta
        0x0A20 => (Some(Consonant), None),                       // Ttha
        0x0A21 => (Some(Consonant), None),                       // Dda
        0x0A22 => (Some(Consonant), None),                       // Ddha
        0x0A23 => (Some(Consonant), None),                       // Nna
        0x0A24 => (Some(Consonant), None),                       // Ta
        0x0A25 => (Some(Consonant), None),                       // Tha
        0x0A26 => (Some(Consonant), None),                       // Da
        0x0A27 => (Some(Consonant), None),                       // Dha
        0x0A28 => (Some(Consonant), None),                       // Na
        0x0A29 => (None, None),                                  // unassigned
        0x0A2A => (Some(Consonant), None),                       // Pa
        0x0A2B => (Some(Consonant), None),                       // Pha
        0x0A2C => (Some(Consonant), None),                       // Ba
        0x0A2D => (Some(Consonant), None),                       // Bha
        0x0A2E => (Some(Consonant), None),                       // Ma
        0x0A2F => (Some(Consonant), None),                       // Ya
        0x0A30 => (Some(Consonant), None),                       // Ra
        0x0A31 => (None, None),                                  // unassigned
        0x0A32 => (Some(Consonant), None),                       // La
        0x0A33 => (Some(Consonant), None),                       // Lla
        0x0A34 => (None, None),                                  // unassigned
        0x0A35 => (Some(Consonant), None),                       // Va
        0x0A36 => (Some(Consonant), None),                       // Sha
        0x0A37 => (None, None),                                  // unassigned
        0x0A38 => (Some(Consonant), None),                       // Sa
        0x0A39 => (Some(Consonant), None),                       // Ha
        0x0A3A => (None, None),                                  // unassigned
        0x0A3B => (None, None),                                  // unassigned
        0x0A3C => (Some(Nukta), Some(BottomPosition)),           // Nukta
        0x0A3D => (None, None),                                  // unassigned
        0x0A3E => (Some(VowelDependent), Some(RightPosition)),   // Sign Aa
        0x0A3F => (Some(VowelDependent), Some(LeftPosition)),    // Sign I
        0x0A40 => (Some(VowelDependent), Some(RightPosition)),   // Sign Ii
        0x0A41 => (Some(VowelDependent), Some(BottomPosition)),  // Sign U
        0x0A42 => (Some(VowelDependent), Some(BottomPosition)),  // Sign Uu
        0x0A43 => (None, None),                                  // unassigned
        0x0A44 => (None, None),                                  // unassigned
        0x0A45 => (None, None),                                  // unassigned
        0x0A46 => (None, None),                                  // unassigned
        0x0A47 => (Some(VowelDependent), Some(TopPosition)),     // Sign Ee
        0x0A48 => (Some(VowelDependent), Some(TopPosition)),     // Sign Ai
        0x0A49 => (None, None),                                  // unassigned
        0x0A4A => (None, None),                                  // unassigned
        0x0A4B => (Some(VowelDependent), Some(TopPosition)),     // Sign Oo
        0x0A4C => (Some(VowelDependent), Some(TopPosition)),     // Sign Au
        0x0A4D => (Some(Virama), Some(BottomPosition)),          // Virama
        0x0A4E => (None, None),                                  // unassigned
        0x0A4F => (None, None),                                  // unassigned
        0x0A50 => (None, None),                                  // unassigned
        0x0A51 => (None, None),                                  // Udaat
        0x0A52 => (None, None),                                  // unassigned
        0x0A53 => (None, None),                                  // unassigned
        0x0A54 => (None, None),                                  // unassigned
        0x0A55 => (None, None),                                  // unassigned
        0x0A56 => (None, None),                                  // unassigned
        0x0A57 => (None, None),                                  // unassigned
        0x0A58 => (None, None),                                  // unassigned
        0x0A59 => (Some(Consonant), None),                       // Khha
        0x0A5A => (Some(Consonant), None),                       // Ghha
        0x0A5B => (Some(Consonant), None),                       // Za
        0x0A5C => (Some(Consonant), None),                       // Rra
        0x0A5D => (None, None),                                  // unassigned
        0x0A5E => (Some(Consonant), None),                       // Fa
        0x0A5F => (None, None),                                  // unassigned
        0x0A60 => (None, None),                                  // unassigned
        0x0A61 => (None, None),                                  // unassigned
        0x0A62 => (None, None),                                  // unassigned
        0x0A63 => (None, None),                                  // unassigned
        0x0A64 => (None, None),                                  // unassigned
        0x0A65 => (None, None),                                  // unassigned
        0x0A66 => (Some(Number), None),                          // Digit Zero
        0x0A67 => (Some(Number), None),                          // Digit One
        0x0A68 => (Some(Number), None),                          // Digit Two
        0x0A69 => (Some(Number), None),                          // Digit Three
        0x0A6A => (Some(Number), None),                          // Digit Four
        0x0A6B => (Some(Number), None),                          // Digit Five
        0x0A6C => (Some(Number), None),                          // Digit Six
        0x0A6D => (Some(Number), None),                          // Digit Seven
        0x0A6E => (Some(Number), None),                          // Digit Eight
        0x0A6F => (Some(Number), None),                          // Digit Nine
        0x0A70 => (Some(Bindu), Some(TopPosition)),              // Tippi
        0x0A71 => (Some(GeminationMark), Some(TopPosition)),     // Addak
        0x0A72 => (Some(ConsonantPlaceholder), None),            // Iri
        0x0A73 => (Some(ConsonantPlaceholder), None),            // Ura
        0x0A74 => (None, None),                                  // Ek Onkar
        0x0A75 => (Some(ConsonantMedial), Some(BottomPosition)), // Yakash
        0x0A76 => (None, None),                                  // Abbreviation Sign

        // Gujarati character table
        0x0A81 => (Some(Bindu), Some(TopPosition)),                  // Candrabindu
        0x0A82 => (Some(Bindu), Some(TopPosition)),                  // Anusvara
        0x0A83 => (Some(Visarga), Some(RightPosition)),              // Visarga
        0x0A84 => (None, None),                                      // unassigned
        0x0A85 => (Some(VowelIndependent), None),                    // A
        0x0A86 => (Some(VowelIndependent), None),                    // Aa
        0x0A87 => (Some(VowelIndependent), None),                    // I
        0x0A88 => (Some(VowelIndependent), None),                    // Ii
        0x0A89 => (Some(VowelIndependent), None),                    // U
        0x0A8A => (Some(VowelIndependent), None),                    // Uu
        0x0A8B => (Some(VowelIndependent), None),                    // Vocalic R
        0x0A8C => (Some(VowelIndependent), None),                    // Vocalic L
        0x0A8D => (Some(VowelIndependent), None),                    // Candra E
        0x0A8E => (None, None),                                      // unassigned
        0x0A8F => (Some(VowelIndependent), None),                    // E
        0x0A90 => (Some(VowelIndependent), None),                    // Ai
        0x0A91 => (Some(VowelIndependent), None),                    // Candra O
        0x0A92 => (None, None),                                      // unassigned
        0x0A93 => (Some(VowelIndependent), None),                    // O
        0x0A94 => (Some(VowelIndependent), None),                    // Au
        0x0A95 => (Some(Consonant), None),                           // Ka
        0x0A96 => (Some(Consonant), None),                           // Kha
        0x0A97 => (Some(Consonant), None),                           // Ga
        0x0A98 => (Some(Consonant), None),                           // Gha
        0x0A99 => (Some(Consonant), None),                           // Nga
        0x0A9A => (Some(Consonant), None),                           // Ca
        0x0A9B => (Some(Consonant), None),                           // Cha
        0x0A9C => (Some(Consonant), None),                           // Ja
        0x0A9D => (Some(Consonant), None),                           // Jha
        0x0A9E => (Some(Consonant), None),                           // Nya
        0x0A9F => (Some(Consonant), None),                           // Tta
        0x0AA0 => (Some(Consonant), None),                           // Ttha
        0x0AA1 => (Some(Consonant), None),                           // Dda
        0x0AA2 => (Some(Consonant), None),                           // Ddha
        0x0AA3 => (Some(Consonant), None),                           // Nna
        0x0AA4 => (Some(Consonant), None),                           // Ta
        0x0AA5 => (Some(Consonant), None),                           // Tha
        0x0AA6 => (Some(Consonant), None),                           // Da
        0x0AA7 => (Some(Consonant), None),                           // Dha
        0x0AA8 => (Some(Consonant), None),                           // Na
        0x0AA9 => (None, None),                                      // unassigned
        0x0AAA => (Some(Consonant), None),                           // Pa
        0x0AAB => (Some(Consonant), None),                           // Pha
        0x0AAC => (Some(Consonant), None),                           // Ba
        0x0AAD => (Some(Consonant), None),                           // Bha
        0x0AAE => (Some(Consonant), None),                           // Ma
        0x0AAF => (Some(Consonant), None),                           // Ya
        0x0AB0 => (Some(Consonant), None),                           // Ra
        0x0AB1 => (None, None),                                      // unassigned
        0x0AB2 => (Some(Consonant), None),                           // La
        0x0AB3 => (Some(Consonant), None),                           // Lla
        0x0AB4 => (None, None),                                      // unassigned
        0x0AB5 => (Some(Consonant), None),                           // Va
        0x0AB6 => (Some(Consonant), None),                           // Sha
        0x0AB7 => (Some(Consonant), None),                           // Ssa
        0x0AB8 => (Some(Consonant), None),                           // Sa
        0x0AB9 => (Some(Consonant), None),                           // Ha
        0x0ABA => (None, None),                                      // unassigned
        0x0ABB => (None, None),                                      // unassigned
        0x0ABC => (Some(Nukta), Some(BottomPosition)),               // Nukta
        0x0ABD => (Some(Avagraha), None),                            // Avagraha
        0x0ABE => (Some(VowelDependent), Some(RightPosition)),       // Sign Aa
        0x0ABF => (Some(VowelDependent), Some(LeftPosition)),        // Sign I
        0x0AC0 => (Some(VowelDependent), Some(RightPosition)),       // Sign Ii
        0x0AC1 => (Some(VowelDependent), Some(BottomPosition)),      // Sign U
        0x0AC2 => (Some(VowelDependent), Some(BottomPosition)),      // Sign Uu
        0x0AC3 => (Some(VowelDependent), Some(BottomPosition)),      // Sign Vocalic R
        0x0AC4 => (Some(VowelDependent), Some(BottomPosition)),      // Sign Vocalic Rr
        0x0AC5 => (Some(VowelDependent), Some(TopPosition)),         // Sign Candra E
        0x0AC6 => (None, None),                                      // unassigned
        0x0AC7 => (Some(VowelDependent), Some(TopPosition)),         // Sign E
        0x0AC8 => (Some(VowelDependent), Some(TopPosition)),         // Sign Ai
        0x0AC9 => (Some(VowelDependent), Some(TopAndRightPosition)), // Sign Candra O
        0x0ACA => (None, None),                                      // unassigned
        0x0ACB => (Some(VowelDependent), Some(RightPosition)),       // Sign O
        0x0ACC => (Some(VowelDependent), Some(RightPosition)),       // Sign Au
        0x0ACD => (Some(Virama), Some(BottomPosition)),              // Virama
        0x0ACE => (None, None),                                      // unassigned
        0x0ACF => (None, None),                                      // unassigned
        0x0AD0 => (None, None),                                      // Om
        0x0AD1 => (None, None),                                      // unassigned
        0x0AD2 => (None, None),                                      // unassigned
        0x0AD3 => (None, None),                                      // unassigned
        0x0AD4 => (None, None),                                      // unassigned
        0x0AD5 => (None, None),                                      // unassigned
        0x0AD6 => (None, None),                                      // unassigned
        0x0AD7 => (None, None),                                      // unassigned
        0x0AD8 => (None, None),                                      // unassigned
        0x0AD9 => (None, None),                                      // unassigned
        0x0ADA => (None, None),                                      // unassigned
        0x0ADB => (None, None),                                      // unassigned
        0x0ADC => (None, None),                                      // unassigned
        0x0ADD => (None, None),                                      // unassigned
        0x0ADE => (None, None),                                      // unassigned
        0x0ADF => (None, None),                                      // unassigned
        0x0AE0 => (Some(VowelIndependent), None),                    // Vocalic Rr
        0x0AE1 => (Some(VowelIndependent), None),                    // Vocalic Ll
        0x0AE2 => (Some(VowelDependent), Some(BottomPosition)),      // Sign Vocalic L
        0x0AE3 => (Some(VowelDependent), Some(BottomPosition)),      // Sign Vocalic Ll
        0x0AE4 => (None, None),                                      // unassigned
        0x0AE5 => (None, None),                                      // unassigned
        0x0AE6 => (Some(Number), None),                              // Digit Zero
        0x0AE7 => (Some(Number), None),                              // Digit One
        0x0AE8 => (Some(Number), None),                              // Digit Two
        0x0AE9 => (Some(Number), None),                              // Digit Three
        0x0AEA => (Some(Number), None),                              // Digit Four
        0x0AEB => (Some(Number), None),                              // Digit Five
        0x0AEC => (Some(Number), None),                              // Digit Six
        0x0AED => (Some(Number), None),                              // Digit Seven
        0x0AEE => (Some(Number), None),                              // Digit Eight
        0x0AEF => (Some(Number), None),                              // Digit Nine
        0x0AF0 => (Some(Symbol), None),                              // Abbreviation
        0x0AF1 => (Some(Symbol), None),                              // Rupee Sign
        0x0AF2 => (None, None),                                      // unassigned
        0x0AF3 => (None, None),                                      // unassigned
        0x0AF4 => (None, None),                                      // unassigned
        0x0AF5 => (None, None),                                      // unassigned
        0x0AF6 => (None, None),                                      // unassigned
        0x0AF7 => (None, None),                                      // unassigned
        0x0AF8 => (None, None),                                      // unassigned
        0x0AF9 => (Some(Consonant), None),                           // Zha
        0x0AFA => (Some(Cantillation), Some(TopPosition)),           // Sukun
        0x0AFB => (Some(Cantillation), Some(TopPosition)),           // Shadda
        0x0AFC => (Some(Cantillation), Some(TopPosition)),           // Maddah
        0x0AFD => (Some(Nukta), Some(TopPosition)),                  // Three-Dot Nukta Above
        0x0AFE => (Some(Nukta), Some(TopPosition)),                  // Circle Nukta Above
        0x0AFF => (Some(Nukta), Some(TopPosition)),                  // Two-Circle Nukta Above

        // Oriya character table
        0x0B00 => (None, None),                                          // unassigned
        0x0B01 => (Some(Bindu), Some(TopPosition)),                      // Candrabindu
        0x0B02 => (Some(Bindu), Some(RightPosition)),                    // Anusvara
        0x0B03 => (Some(Visarga), Some(RightPosition)),                  // Visarga
        0x0B04 => (None, None),                                          // unassigned
        0x0B05 => (Some(VowelIndependent), None),                        // A
        0x0B06 => (Some(VowelIndependent), None),                        // Aa
        0x0B07 => (Some(VowelIndependent), None),                        // I
        0x0B08 => (Some(VowelIndependent), None),                        // Ii
        0x0B09 => (Some(VowelIndependent), None),                        // U
        0x0B0A => (Some(VowelIndependent), None),                        // Uu
        0x0B0B => (Some(VowelIndependent), None),                        // Vocalic R
        0x0B0C => (Some(VowelIndependent), None),                        // Vocalic L
        0x0B0D => (None, None),                                          // unassigned
        0x0B0E => (None, None),                                          // unassigned
        0x0B0F => (Some(VowelIndependent), None),                        // E
        0x0B10 => (Some(VowelIndependent), None),                        // Ai
        0x0B11 => (None, None),                                          // unassigned
        0x0B12 => (None, None),                                          // unassigned
        0x0B13 => (Some(VowelIndependent), None),                        // O
        0x0B14 => (Some(VowelIndependent), None),                        // Au
        0x0B15 => (Some(Consonant), None),                               // Ka
        0x0B16 => (Some(Consonant), None),                               // Kha
        0x0B17 => (Some(Consonant), None),                               // Ga
        0x0B18 => (Some(Consonant), None),                               // Gha
        0x0B19 => (Some(Consonant), None),                               // Nga
        0x0B1A => (Some(Consonant), None),                               // Ca
        0x0B1B => (Some(Consonant), None),                               // Cha
        0x0B1C => (Some(Consonant), None),                               // Ja
        0x0B1D => (Some(Consonant), None),                               // Jha
        0x0B1E => (Some(Consonant), None),                               // Nya
        0x0B1F => (Some(Consonant), None),                               // Tta
        0x0B20 => (Some(Consonant), None),                               // Ttha
        0x0B21 => (Some(Consonant), None),                               // Dda
        0x0B22 => (Some(Consonant), None),                               // Ddha
        0x0B23 => (Some(Consonant), None),                               // Nna
        0x0B24 => (Some(Consonant), None),                               // Ta
        0x0B25 => (Some(Consonant), None),                               // Tha
        0x0B26 => (Some(Consonant), None),                               // Da
        0x0B27 => (Some(Consonant), None),                               // Dha
        0x0B28 => (Some(Consonant), None),                               // Na
        0x0B29 => (None, None),                                          // unassigned
        0x0B2A => (Some(Consonant), None),                               // Pa
        0x0B2B => (Some(Consonant), None),                               // Pha
        0x0B2C => (Some(Consonant), None),                               // Ba
        0x0B2D => (Some(Consonant), None),                               // Bha
        0x0B2E => (Some(Consonant), None),                               // Ma
        0x0B2F => (Some(Consonant), None),                               // Ya
        0x0B30 => (Some(Consonant), None),                               // Ra
        0x0B31 => (None, None),                                          // unassigned
        0x0B32 => (Some(Consonant), None),                               // La
        0x0B33 => (Some(Consonant), None),                               // Lla
        0x0B34 => (None, None),                                          // unassigned
        0x0B35 => (Some(Consonant), None),                               // Va
        0x0B36 => (Some(Consonant), None),                               // Sha
        0x0B37 => (Some(Consonant), None),                               // Ssa
        0x0B38 => (Some(Consonant), None),                               // Sa
        0x0B39 => (Some(Consonant), None),                               // Ha
        0x0B3A => (None, None),                                          // unassigned
        0x0B3B => (None, None),                                          // unassigned
        0x0B3C => (Some(Nukta), Some(BottomPosition)),                   // Nukta
        0x0B3D => (Some(Avagraha), None),                                // Avagraha
        0x0B3E => (Some(VowelDependent), Some(RightPosition)),           // Sign Aa
        0x0B3F => (Some(VowelDependent), Some(TopPosition)),             // Sign I
        0x0B40 => (Some(VowelDependent), Some(RightPosition)),           // Sign Ii
        0x0B41 => (Some(VowelDependent), Some(BottomPosition)),          // Sign U
        0x0B42 => (Some(VowelDependent), Some(BottomPosition)),          // Sign Uu
        0x0B43 => (Some(VowelDependent), Some(BottomPosition)),          // Sign Vocalic R
        0x0B44 => (Some(VowelDependent), Some(BottomPosition)),          // Sign Vocalic Rr
        0x0B45 => (None, None),                                          // unassigned
        0x0B46 => (None, None),                                          // unassigned
        0x0B47 => (Some(VowelDependent), Some(LeftPosition)),            // Sign E
        0x0B48 => (Some(VowelDependent), Some(TopAndLeftPosition)),      // Sign Ai
        0x0B49 => (None, None),                                          // unassigned
        0x0B4A => (None, None),                                          // unassigned
        0x0B4B => (Some(VowelDependent), Some(LeftAndRightPosition)),    // Sign O
        0x0B4C => (Some(VowelDependent), Some(TopLeftAndRightPosition)), // Sign Au
        0x0B4D => (Some(Virama), Some(BottomPosition)),                  // Virama
        0x0B4E => (None, None),                                          // unassigned
        0x0B4F => (None, None),                                          // unassigned
        0x0B50 => (None, None),                                          // unassigned
        0x0B51 => (None, None),                                          // unassigned
        0x0B52 => (None, None),                                          // unassigned
        0x0B53 => (None, None),                                          // unassigned
        0x0B54 => (None, None),                                          // unassigned
        0x0B55 => (None, None),                                          // unassigned
        0x0B56 => (Some(VowelDependent), Some(TopPosition)),             // Ai Length Mark
        0x0B57 => (Some(VowelDependent), Some(TopAndRightPosition)),     // Au Length Mark
        0x0B58 => (None, None),                                          // unassigned
        0x0B59 => (None, None),                                          // unassigned
        0x0B5A => (None, None),                                          // unassigned
        0x0B5B => (None, None),                                          // unassigned
        0x0B5C => (Some(Consonant), None),                               // Rra
        0x0B5D => (Some(Consonant), None),                               // Rha
        0x0B5E => (None, None),                                          // unassigned
        0x0B5F => (Some(Consonant), None),                               // Yya
        0x0B60 => (Some(VowelIndependent), None),                        // Vocalic Rr
        0x0B61 => (Some(VowelIndependent), None),                        // Vocalic Ll
        0x0B62 => (Some(VowelDependent), Some(BottomPosition)),          // Sign Vocalic L
        0x0B63 => (Some(VowelDependent), Some(BottomPosition)),          // Sign Vocalic Ll
        0x0B64 => (None, None),                                          // unassigned
        0x0B65 => (None, None),                                          // unassigned
        0x0B66 => (Some(Number), None),                                  // Digit Zero
        0x0B67 => (Some(Number), None),                                  // Digit One
        0x0B68 => (Some(Number), None),                                  // Digit Two
        0x0B69 => (Some(Number), None),                                  // Digit Three
        0x0B6A => (Some(Number), None),                                  // Digit Four
        0x0B6B => (Some(Number), None),                                  // Digit Five
        0x0B6C => (Some(Number), None),                                  // Digit Six
        0x0B6D => (Some(Number), None),                                  // Digit Seven
        0x0B6E => (Some(Number), None),                                  // Digit Eight
        0x0B6F => (Some(Number), None),                                  // Digit Nine
        0x0B70 => (Some(Symbol), None),                                  // Isshar
        0x0B71 => (Some(Consonant), None),                               // Wa
        0x0B72 => (Some(Number), None),                                  // Fraction 1/4
        0x0B73 => (Some(Number), None),                                  // Fraction 1/2
        0x0B74 => (Some(Number), None),                                  // Fraction 3/4
        0x0B75 => (Some(Number), None),                                  // Fraction 1/16
        0x0B76 => (Some(Number), None),                                  // Fraction 1/8
        0x0B77 => (Some(Number), None),                                  // Fraction 3/16
        0x0B78 => (None, None),                                          // unassigned
        0x0B79 => (None, None),                                          // unassigned
        0x0B7A => (None, None),                                          // unassigned
        0x0B7B => (None, None),                                          // unassigned
        0x0B7C => (None, None),                                          // unassigned
        0x0B7D => (None, None),                                          // unassigned
        0x0B7E => (None, None),                                          // unassigned
        0x0B7F => (None, None),                                          // unassigned

        // Tamil character table
        0x0B80 => (None, None),                                       // unassigned
        0x0B81 => (None, None),                                       // unassigned
        0x0B82 => (Some(Bindu), Some(TopPosition)),                   // Anusvara
        0x0B83 => (Some(ModifyingLetter), None),                      // Visarga
        0x0B84 => (None, None),                                       // unassigned
        0x0B85 => (Some(VowelIndependent), None),                     // A
        0x0B86 => (Some(VowelIndependent), None),                     // Aa
        0x0B87 => (Some(VowelIndependent), None),                     // I
        0x0B88 => (Some(VowelIndependent), None),                     // Ii
        0x0B89 => (Some(VowelIndependent), None),                     // U
        0x0B8A => (Some(VowelIndependent), None),                     // Uu
        0x0B8B => (None, None),                                       // unassigned
        0x0B8C => (None, None),                                       // unassigned
        0x0B8D => (None, None),                                       // unassigned
        0x0B8E => (Some(VowelIndependent), None),                     // E
        0x0B8F => (Some(VowelIndependent), None),                     // Ee
        0x0B90 => (Some(VowelIndependent), None),                     // Ai
        0x0B91 => (None, None),                                       // unassigned
        0x0B92 => (Some(VowelIndependent), None),                     // O
        0x0B93 => (Some(VowelIndependent), None),                     // Oo
        0x0B94 => (Some(VowelIndependent), None),                     // Au
        0x0B95 => (Some(Consonant), None),                            // Ka
        0x0B96 => (None, None),                                       // unassigned
        0x0B97 => (None, None),                                       // unassigned
        0x0B98 => (None, None),                                       // unassigned
        0x0B99 => (Some(Consonant), None),                            // Nga
        0x0B9A => (Some(Consonant), None),                            // Ca
        0x0B9B => (None, None),                                       // unassigned
        0x0B9C => (Some(Consonant), None),                            // Ja
        0x0B9D => (None, None),                                       // unassigned
        0x0B9E => (Some(Consonant), None),                            // Nya
        0x0B9F => (Some(Consonant), None),                            // Tta
        0x0BA0 => (None, None),                                       // unassigned
        0x0BA1 => (None, None),                                       // unassigned
        0x0BA2 => (None, None),                                       // unassigned
        0x0BA3 => (Some(Consonant), None),                            // Nna
        0x0BA4 => (Some(Consonant), None),                            // Ta
        0x0BA5 => (None, None),                                       // unassigned
        0x0BA6 => (None, None),                                       // unassigned
        0x0BA7 => (None, None),                                       // unassigned
        0x0BA8 => (Some(Consonant), None),                            // Na
        0x0BA9 => (Some(Consonant), None),                            // Nnna
        0x0BAA => (Some(Consonant), None),                            // Pa
        0x0BAB => (None, None),                                       // unassigned
        0x0BAC => (None, None),                                       // unassigned
        0x0BAD => (None, None),                                       // unassigned
        0x0BAE => (Some(Consonant), None),                            // Ma
        0x0BAF => (Some(Consonant), None),                            // Ya
        0x0BB0 => (Some(Consonant), None),                            // Ra
        0x0BB1 => (Some(Consonant), None),                            // Rra
        0x0BB2 => (Some(Consonant), None),                            // La
        0x0BB3 => (Some(Consonant), None),                            // Lla
        0x0BB4 => (Some(Consonant), None),                            // Llla
        0x0BB5 => (Some(Consonant), None),                            // Va
        0x0BB6 => (Some(Consonant), None),                            // Sha
        0x0BB7 => (Some(Consonant), None),                            // Ssa
        0x0BB8 => (Some(Consonant), None),                            // Sa
        0x0BB9 => (Some(Consonant), None),                            // Ha
        0x0BBA => (None, None),                                       // unassigned
        0x0BBB => (None, None),                                       // unassigned
        0x0BBC => (None, None),                                       // unassigned
        0x0BBD => (None, None),                                       // unassigned
        0x0BBE => (Some(VowelDependent), Some(RightPosition)),        // Sign Aa
        0x0BBF => (Some(VowelDependent), Some(RightPosition)),        // Sign I
        0x0BC0 => (Some(VowelDependent), Some(TopPosition)),          // Sign Ii
        0x0BC1 => (Some(VowelDependent), Some(RightPosition)),        // Sign U
        0x0BC2 => (Some(VowelDependent), Some(RightPosition)),        // Sign Uu
        0x0BC3 => (None, None),                                       // unassigned
        0x0BC4 => (None, None),                                       // unassigned
        0x0BC5 => (None, None),                                       // unassigned
        0x0BC6 => (Some(VowelDependent), Some(LeftPosition)),         // Sign E
        0x0BC7 => (Some(VowelDependent), Some(LeftPosition)),         // Sign Ee
        0x0BC8 => (Some(VowelDependent), Some(LeftPosition)),         // Sign Ai
        0x0BC9 => (None, None),                                       // unassigned
        0x0BCA => (Some(VowelDependent), Some(LeftAndRightPosition)), // Sign O
        0x0BCB => (Some(VowelDependent), Some(LeftAndRightPosition)), // Sign Oo
        0x0BCC => (Some(VowelDependent), Some(LeftAndRightPosition)), // Sign Au
        0x0BCD => (Some(Virama), Some(TopPosition)),                  // Virama
        0x0BCE => (None, None),                                       // unassigned
        0x0BCF => (None, None),                                       // unassigned
        0x0BD0 => (None, None),                                       // Om
        0x0BD1 => (None, None),                                       // unassigned
        0x0BD2 => (None, None),                                       // unassigned
        0x0BD3 => (None, None),                                       // unassigned
        0x0BD4 => (None, None),                                       // unassigned
        0x0BD5 => (None, None),                                       // unassigned
        0x0BD6 => (None, None),                                       // unassigned
        0x0BD7 => (Some(VowelDependent), Some(RightPosition)),        // Au Length Mark
        0x0BD8 => (None, None),                                       // unassigned
        0x0BD9 => (None, None),                                       // unassigned
        0x0BDA => (None, None),                                       // unassigned
        0x0BDB => (None, None),                                       // unassigned
        0x0BDC => (None, None),                                       // unassigned
        0x0BDD => (None, None),                                       // unassigned
        0x0BDE => (None, None),                                       // unassigned
        0x0BDF => (None, None),                                       // unassigned
        0x0BE0 => (None, None),                                       // unassigned
        0x0BE1 => (None, None),                                       // unassigned
        0x0BE2 => (None, None),                                       // unassigned
        0x0BE3 => (None, None),                                       // unassigned
        0x0BE4 => (None, None),                                       // unassigned
        0x0BE5 => (None, None),                                       // unassigned
        0x0BE6 => (Some(Number), None),                               // Digit Zero
        0x0BE7 => (Some(Number), None),                               // Digit One
        0x0BE8 => (Some(Number), None),                               // Digit Two
        0x0BE9 => (Some(Number), None),                               // Digit Three
        0x0BEA => (Some(Number), None),                               // Digit Four
        0x0BEB => (Some(Number), None),                               // Digit Five
        0x0BEC => (Some(Number), None),                               // Digit Six
        0x0BED => (Some(Number), None),                               // Digit Seven
        0x0BEE => (Some(Number), None),                               // Digit Eight
        0x0BEF => (Some(Number), None),                               // Digit Nine
        0x0BF0 => (Some(Number), None),                               // Number Ten
        0x0BF1 => (Some(Number), None),                               // Number One Hundred
        0x0BF2 => (Some(Number), None),                               // Number One Thousand
        0x0BF3 => (Some(Symbol), None),                               // Day Sign
        0x0BF4 => (Some(Symbol), None),                               // Month Sign
        0x0BF5 => (Some(Symbol), None),                               // Year Sign
        0x0BF6 => (Some(Symbol), None),                               // Debit Sign
        0x0BF7 => (Some(Symbol), None),                               // Credit Sign
        0x0BF8 => (Some(Symbol), None),                               // As Above Sign
        0x0BF9 => (Some(Symbol), None),                               // Tamil Rupee Sign
        0x0BFA => (Some(Symbol), None),                               // Number Sign

        // Telugu character table
        0x0C00 => (Some(Bindu), Some(TopPosition)),                   // Combining Candrabindu Above
        0x0C01 => (Some(Bindu), Some(RightPosition)),                 // Candrabindu
        0x0C02 => (Some(Bindu), Some(RightPosition)),                 // Anusvara
        0x0C03 => (Some(Visarga), Some(RightPosition)),               // Visarga
        0x0C04 => (Some(Bindu), Some(TopPosition)),                   // Combining Anusvara Above
        0x0C05 => (Some(VowelIndependent), None),                     // A
        0x0C06 => (Some(VowelIndependent), None),                     // Aa
        0x0C07 => (Some(VowelIndependent), None),                     // I
        0x0C08 => (Some(VowelIndependent), None),                     // Ii
        0x0C09 => (Some(VowelIndependent), None),                     // U
        0x0C0A => (Some(VowelIndependent), None),                     // Uu
        0x0C0B => (Some(VowelIndependent), None),                     // Vocalic R
        0x0C0C => (Some(VowelIndependent), None),                     // Vocalic L
        0x0C0D => (None, None),                                       // unassigned
        0x0C0E => (Some(VowelIndependent), None),                     // E
        0x0C0F => (Some(VowelIndependent), None),                     // Ee
        0x0C10 => (Some(VowelIndependent), None),                     // Ai
        0x0C11 => (None, None),                                       // unassigned
        0x0C12 => (Some(VowelIndependent), None),                     // O
        0x0C13 => (Some(VowelIndependent), None),                     // Oo
        0x0C14 => (Some(VowelIndependent), None),                     // Au
        0x0C15 => (Some(Consonant), None),                            // Ka
        0x0C16 => (Some(Consonant), None),                            // Kha
        0x0C17 => (Some(Consonant), None),                            // Ga
        0x0C18 => (Some(Consonant), None),                            // Gha
        0x0C19 => (Some(Consonant), None),                            // Nga
        0x0C1A => (Some(Consonant), None),                            // Ca
        0x0C1B => (Some(Consonant), None),                            // Cha
        0x0C1C => (Some(Consonant), None),                            // Ja
        0x0C1D => (Some(Consonant), None),                            // Jha
        0x0C1E => (Some(Consonant), None),                            // Nya
        0x0C1F => (Some(Consonant), None),                            // Tta
        0x0C20 => (Some(Consonant), None),                            // Ttha
        0x0C21 => (Some(Consonant), None),                            // Dda
        0x0C22 => (Some(Consonant), None),                            // Ddha
        0x0C23 => (Some(Consonant), None),                            // Nna
        0x0C24 => (Some(Consonant), None),                            // Ta
        0x0C25 => (Some(Consonant), None),                            // Tha
        0x0C26 => (Some(Consonant), None),                            // Da
        0x0C27 => (Some(Consonant), None),                            // Dha
        0x0C28 => (Some(Consonant), None),                            // Na
        0x0C29 => (None, None),                                       // unassigned
        0x0C2A => (Some(Consonant), None),                            // Pa
        0x0C2B => (Some(Consonant), None),                            // Pha
        0x0C2C => (Some(Consonant), None),                            // Ba
        0x0C2D => (Some(Consonant), None),                            // Bha
        0x0C2E => (Some(Consonant), None),                            // Ma
        0x0C2F => (Some(Consonant), None),                            // Ya
        0x0C30 => (Some(Consonant), None),                            // Ra
        0x0C31 => (Some(Consonant), None),                            // Rra
        0x0C32 => (Some(Consonant), None),                            // La
        0x0C33 => (Some(Consonant), None),                            // Lla
        0x0C34 => (Some(Consonant), None),                            // Llla
        0x0C35 => (Some(Consonant), None),                            // Va
        0x0C36 => (Some(Consonant), None),                            // Sha
        0x0C37 => (Some(Consonant), None),                            // Ssa
        0x0C38 => (Some(Consonant), None),                            // Sa
        0x0C39 => (Some(Consonant), None),                            // Ha
        0x0C3A => (None, None),                                       // unassigned
        0x0C3B => (None, None),                                       // unassigned
        0x0C3C => (None, None),                                       // unassigned
        0x0C3D => (Some(Avagraha), None),                             // Avagraha
        0x0C3E => (Some(VowelDependent), Some(TopPosition)),          // Sign Aa
        0x0C3F => (Some(VowelDependent), Some(TopPosition)),          // Sign I
        0x0C40 => (Some(VowelDependent), Some(TopPosition)),          // Sign Ii
        0x0C41 => (Some(VowelDependent), Some(RightPosition)),        // Sign U
        0x0C42 => (Some(VowelDependent), Some(RightPosition)),        // Sign Uu
        0x0C43 => (Some(VowelDependent), Some(RightPosition)),        // Sign Vocalic R
        0x0C44 => (Some(VowelDependent), Some(RightPosition)),        // Sign Vocalic Rr
        0x0C45 => (None, None),                                       // unassigned
        0x0C46 => (Some(VowelDependent), Some(TopPosition)),          // Sign E
        0x0C47 => (Some(VowelDependent), Some(TopPosition)),          // Sign Ee
        0x0C48 => (Some(VowelDependent), Some(TopAndBottomPosition)), // Sign Ai
        0x0C49 => (None, None),                                       // unassigned
        0x0C4A => (Some(VowelDependent), Some(TopPosition)),          // Sign O
        0x0C4B => (Some(VowelDependent), Some(TopPosition)),          // Sign Oo
        0x0C4C => (Some(VowelDependent), Some(TopPosition)),          // Sign Au
        0x0C4D => (Some(Virama), Some(TopPosition)),                  // Virama
        0x0C4E => (None, None),                                       // unassigned
        0x0C4F => (None, None),                                       // unassigned
        0x0C50 => (None, None),                                       // unassigned
        0x0C51 => (None, None),                                       // unassigned
        0x0C52 => (None, None),                                       // unassigned
        0x0C53 => (None, None),                                       // unassigned
        0x0C54 => (None, None),                                       // unassigned
        0x0C55 => (Some(VowelDependent), Some(TopPosition)),          // Length Mark
        0x0C56 => (Some(VowelDependent), Some(BottomPosition)),       // Ai Length Mark
        0x0C57 => (None, None),                                       // unassigned
        0x0C58 => (Some(Consonant), None),                            // Tsa
        0x0C59 => (Some(Consonant), None),                            // Dza
        0x0C5A => (Some(Consonant), None),                            // Rrra
        0x0C5B => (None, None),                                       // unassigned
        0x0C5C => (None, None),                                       // unassigned
        0x0C5D => (None, None),                                       // unassigned
        0x0C5E => (None, None),                                       // unassigned
        0x0C5F => (None, None),                                       // unassigned
        0x0C60 => (Some(VowelIndependent), None),                     // Vocalic Rr
        0x0C61 => (Some(VowelIndependent), None),                     // Vocalic Ll
        0x0C62 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic L
        0x0C63 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic Ll
        0x0C64 => (None, None),                                       // unassigned
        0x0C65 => (None, None),                                       // unassigned
        0x0C66 => (Some(Number), None),                               // Digit Zero
        0x0C67 => (Some(Number), None),                               // Digit One
        0x0C68 => (Some(Number), None),                               // Digit Two
        0x0C69 => (Some(Number), None),                               // Digit Three
        0x0C6A => (Some(Number), None),                               // Digit Four
        0x0C6B => (Some(Number), None),                               // Digit Five
        0x0C6C => (Some(Number), None),                               // Digit Six
        0x0C6D => (Some(Number), None),                               // Digit Seven
        0x0C6E => (Some(Number), None),                               // Digit Eight
        0x0C6F => (Some(Number), None),                               // Digit Nine
        0x0C70 => (None, None),                                       // unassigned
        0x0C71 => (None, None),                                       // unassigned
        0x0C72 => (None, None),                                       // unassigned
        0x0C73 => (None, None),                                       // unassigned
        0x0C74 => (None, None),                                       // unassigned
        0x0C75 => (None, None),                                       // unassigned
        0x0C76 => (None, None),                                       // unassigned
        0x0C77 => (None, None),                                       // unassigned
        0x0C78 => (Some(Number), None),                               // Fraction Zero Odd P
        0x0C79 => (Some(Number), None),                               // Fraction One Odd P
        0x0C7A => (Some(Number), None),                               // Fraction Two Odd P
        0x0C7B => (Some(Number), None),                               // Fraction Three Odd P
        0x0C7C => (Some(Number), None),                               // Fraction One Even P
        0x0C7D => (Some(Number), None),                               // Fraction Two Even P
        0x0C7E => (Some(Number), None),                               // Fraction Three Even P
        0x0C7F => (Some(Symbol), None),                               // Tuumu

        // Kannada character table
        0x0C80 => (None, None),                                      // Spacing Candrabindu
        0x0C81 => (Some(Bindu), Some(TopPosition)),                  // Candrabindu
        0x0C82 => (Some(Bindu), Some(RightPosition)),                // Anusvara
        0x0C83 => (Some(Visarga), Some(RightPosition)),              // Visarga
        0x0C84 => (None, None),                                      // Siddham
        0x0C85 => (Some(VowelIndependent), None),                    // A
        0x0C86 => (Some(VowelIndependent), None),                    // Aa
        0x0C87 => (Some(VowelIndependent), None),                    // I
        0x0C88 => (Some(VowelIndependent), None),                    // Ii
        0x0C89 => (Some(VowelIndependent), None),                    // U
        0x0C8A => (Some(VowelIndependent), None),                    // Uu
        0x0C8B => (Some(VowelIndependent), None),                    // Vocalic R
        0x0C8C => (Some(VowelIndependent), None),                    // Vocalic L
        0x0C8D => (None, None),                                      // unassigned
        0x0C8E => (Some(VowelIndependent), None),                    // E
        0x0C8F => (Some(VowelIndependent), None),                    // Ee
        0x0C90 => (Some(VowelIndependent), None),                    // Ai
        0x0C91 => (None, None),                                      // unassigned
        0x0C92 => (Some(VowelIndependent), None),                    // O
        0x0C93 => (Some(VowelIndependent), None),                    // Oo
        0x0C94 => (Some(VowelIndependent), None),                    // Au
        0x0C95 => (Some(Consonant), None),                           // Ka
        0x0C96 => (Some(Consonant), None),                           // Kha
        0x0C97 => (Some(Consonant), None),                           // Ga
        0x0C98 => (Some(Consonant), None),                           // Gha
        0x0C99 => (Some(Consonant), None),                           // Nga
        0x0C9A => (Some(Consonant), None),                           // Ca
        0x0C9B => (Some(Consonant), None),                           // Cha
        0x0C9C => (Some(Consonant), None),                           // Ja
        0x0C9D => (Some(Consonant), None),                           // Jha
        0x0C9E => (Some(Consonant), None),                           // Nya
        0x0C9F => (Some(Consonant), None),                           // Tta
        0x0CA0 => (Some(Consonant), None),                           // Ttha
        0x0CA1 => (Some(Consonant), None),                           // Dda
        0x0CA2 => (Some(Consonant), None),                           // Ddha
        0x0CA3 => (Some(Consonant), None),                           // Nna
        0x0CA4 => (Some(Consonant), None),                           // Ta
        0x0CA5 => (Some(Consonant), None),                           // Tha
        0x0CA6 => (Some(Consonant), None),                           // Da
        0x0CA7 => (Some(Consonant), None),                           // Dha
        0x0CA8 => (Some(Consonant), None),                           // Na
        0x0CA9 => (None, None),                                      // unassigned
        0x0CAA => (Some(Consonant), None),                           // Pa
        0x0CAB => (Some(Consonant), None),                           // Pha
        0x0CAC => (Some(Consonant), None),                           // Ba
        0x0CAD => (Some(Consonant), None),                           // Bha
        0x0CAE => (Some(Consonant), None),                           // Ma
        0x0CAF => (Some(Consonant), None),                           // Ya
        0x0CB0 => (Some(Consonant), None),                           // Ra
        0x0CB1 => (Some(Consonant), None),                           // Rra
        0x0CB2 => (Some(Consonant), None),                           // La
        0x0CB3 => (Some(Consonant), None),                           // Lla
        0x0CB4 => (None, None),                                      // unassigned
        0x0CB5 => (Some(Consonant), None),                           // Va
        0x0CB6 => (Some(Consonant), None),                           // Sha
        0x0CB7 => (Some(Consonant), None),                           // Ssa
        0x0CB8 => (Some(Consonant), None),                           // Sa
        0x0CB9 => (Some(Consonant), None),                           // Ha
        0x0CBA => (None, None),                                      // unassigned
        0x0CBB => (None, None),                                      // unassigned
        0x0CBC => (Some(Nukta), Some(BottomPosition)),               // Nukta
        0x0CBD => (Some(Avagraha), None),                            // Avagraha
        0x0CBE => (Some(VowelDependent), Some(RightPosition)),       // Sign Aa
        0x0CBF => (Some(VowelDependent), Some(TopPosition)),         // Sign I
        0x0CC0 => (Some(VowelDependent), Some(TopAndRightPosition)), // Sign Ii
        0x0CC1 => (Some(VowelDependent), Some(RightPosition)),       // Sign U
        0x0CC2 => (Some(VowelDependent), Some(RightPosition)),       // Sign Uu
        0x0CC3 => (Some(VowelDependent), Some(RightPosition)),       // Sign Vocalic R
        0x0CC4 => (Some(VowelDependent), Some(RightPosition)),       // Sign Vocalic Rr
        0x0CC5 => (None, None),                                      // unassigned
        0x0CC6 => (Some(VowelDependent), Some(TopPosition)),         // Sign E
        0x0CC7 => (Some(VowelDependent), Some(TopAndRightPosition)), // Sign Ee
        0x0CC8 => (Some(VowelDependent), Some(TopAndRightPosition)), // Sign Ai
        0x0CC9 => (None, None),                                      // unassigned
        0x0CCA => (Some(VowelDependent), Some(TopAndRightPosition)), // Sign O
        0x0CCB => (Some(VowelDependent), Some(TopAndRightPosition)), // Sign Oo
        0x0CCC => (Some(VowelDependent), Some(TopPosition)),         // Sign Au
        0x0CCD => (Some(Virama), Some(TopPosition)),                 // Virama
        0x0CCE => (None, None),                                      // unassigned
        0x0CCF => (None, None),                                      // unassigned
        0x0CD0 => (None, None),                                      // unassigned
        0x0CD1 => (None, None),                                      // unassigned
        0x0CD2 => (None, None),                                      // unassigned
        0x0CD3 => (None, None),                                      // unassigned
        0x0CD4 => (None, None),                                      // unassigned
        0x0CD5 => (Some(VowelDependent), Some(RightPosition)),       // Length Mark
        0x0CD6 => (Some(VowelDependent), Some(RightPosition)),       // Ai Length Mark
        0x0CD7 => (None, None),                                      // unassigned
        0x0CD8 => (None, None),                                      // unassigned
        0x0CD9 => (None, None),                                      // unassigned
        0x0CDA => (None, None),                                      // unassigned
        0x0CDB => (None, None),                                      // unassigned
        0x0CDC => (None, None),                                      // unassigned
        0x0CDD => (None, None),                                      // unassigned
        0x0CDE => (Some(Consonant), None),                           // Fa
        0x0CDF => (None, None),                                      // unassigned
        0x0CE0 => (Some(VowelIndependent), None),                    // Vocalic Rr
        0x0CE1 => (Some(VowelIndependent), None),                    // Vocalic Ll
        0x0CE2 => (Some(VowelDependent), Some(BottomPosition)),      // Sign Vocalic L
        0x0CE3 => (Some(VowelDependent), Some(BottomPosition)),      // Sign Vocalic Ll
        0x0CE4 => (None, None),                                      // unassigned
        0x0CE5 => (None, None),                                      // unassigned
        0x0CE6 => (Some(Number), None),                              // Digit Zero
        0x0CE7 => (Some(Number), None),                              // Digit One
        0x0CE8 => (Some(Number), None),                              // Digit Two
        0x0CE9 => (Some(Number), None),                              // Digit Three
        0x0CEA => (Some(Number), None),                              // Digit Four
        0x0CEB => (Some(Number), None),                              // Digit Five
        0x0CEC => (Some(Number), None),                              // Digit Six
        0x0CED => (Some(Number), None),                              // Digit Seven
        0x0CEE => (Some(Number), None),                              // Digit Eight
        0x0CEF => (Some(Number), None),                              // Digit Nine
        0x0CF0 => (None, None),                                      // unassigned
        0x0CF1 => (Some(ConsonantWithStacker), None),                // Jihvamuliya
        0x0CF2 => (Some(ConsonantWithStacker), None),                // Upadhmaniya

        // Malayalam character table
        0x0D00 => (Some(Bindu), Some(TopPosition)),                   // Combining Anusvara Above
        0x0D01 => (Some(Bindu), Some(TopPosition)),                   // Candrabindu
        0x0D02 => (Some(Bindu), Some(RightPosition)),                 // Anusvara
        0x0D03 => (Some(Visarga), Some(RightPosition)),               // Visarga
        0x0D04 => (None, None),                                       // unassigned
        0x0D05 => (Some(VowelIndependent), None),                     // A
        0x0D06 => (Some(VowelIndependent), None),                     // Aa
        0x0D07 => (Some(VowelIndependent), None),                     // I
        0x0D08 => (Some(VowelIndependent), None),                     // Ii
        0x0D09 => (Some(VowelIndependent), None),                     // U
        0x0D0A => (Some(VowelIndependent), None),                     // Uu
        0x0D0B => (Some(VowelIndependent), None),                     // Vocalic R
        0x0D0C => (Some(VowelIndependent), None),                     // Vocalic L
        0x0D0D => (None, None),                                       // unassigned
        0x0D0E => (Some(VowelIndependent), None),                     // E
        0x0D0F => (Some(VowelIndependent), None),                     // Ee
        0x0D10 => (Some(VowelIndependent), None),                     // Ai
        0x0D11 => (None, None),                                       // unassigned
        0x0D12 => (Some(VowelIndependent), None),                     // O
        0x0D13 => (Some(VowelIndependent), None),                     // Oo
        0x0D14 => (Some(VowelIndependent), None),                     // Au
        0x0D15 => (Some(Consonant), None),                            // Ka
        0x0D16 => (Some(Consonant), None),                            // Kha
        0x0D17 => (Some(Consonant), None),                            // Ga
        0x0D18 => (Some(Consonant), None),                            // Gha
        0x0D19 => (Some(Consonant), None),                            // Nga
        0x0D1A => (Some(Consonant), None),                            // Ca
        0x0D1B => (Some(Consonant), None),                            // Cha
        0x0D1C => (Some(Consonant), None),                            // Ja
        0x0D1D => (Some(Consonant), None),                            // Jha
        0x0D1E => (Some(Consonant), None),                            // Nya
        0x0D1F => (Some(Consonant), None),                            // Tta
        0x0D20 => (Some(Consonant), None),                            // Ttha
        0x0D21 => (Some(Consonant), None),                            // Dda
        0x0D22 => (Some(Consonant), None),                            // Ddha
        0x0D23 => (Some(Consonant), None),                            // Nna
        0x0D24 => (Some(Consonant), None),                            // Ta
        0x0D25 => (Some(Consonant), None),                            // Tha
        0x0D26 => (Some(Consonant), None),                            // Da
        0x0D27 => (Some(Consonant), None),                            // Dha
        0x0D28 => (Some(Consonant), None),                            // Na
        0x0D29 => (Some(Consonant), None),                            // Nnna
        0x0D2A => (Some(Consonant), None),                            // Pa
        0x0D2B => (Some(Consonant), None),                            // Pha
        0x0D2C => (Some(Consonant), None),                            // Ba
        0x0D2D => (Some(Consonant), None),                            // Bha
        0x0D2E => (Some(Consonant), None),                            // Ma
        0x0D2F => (Some(Consonant), None),                            // Ya
        0x0D30 => (Some(Consonant), None),                            // Ra
        0x0D31 => (Some(Consonant), None),                            // Rra
        0x0D32 => (Some(Consonant), None),                            // La
        0x0D33 => (Some(Consonant), None),                            // Lla
        0x0D34 => (Some(Consonant), None),                            // Llla
        0x0D35 => (Some(Consonant), None),                            // Va
        0x0D36 => (Some(Consonant), None),                            // Sha
        0x0D37 => (Some(Consonant), None),                            // Ssa
        0x0D38 => (Some(Consonant), None),                            // Sa
        0x0D39 => (Some(Consonant), None),                            // Ha
        0x0D3A => (Some(Consonant), None),                            // Ttta
        0x0D3B => (Some(PureKiller), Some(TopPosition)),              // Vertical Bar Virama
        0x0D3C => (Some(PureKiller), Some(TopPosition)),              // Circular Virama
        0x0D3D => (Some(Avagraha), None),                             // Avagraha
        0x0D3E => (Some(VowelDependent), Some(RightPosition)),        // Sign Aa
        0x0D3F => (Some(VowelDependent), Some(RightPosition)),        // Sign I
        0x0D40 => (Some(VowelDependent), Some(RightPosition)),        // Sign Ii
        0x0D41 => (Some(VowelDependent), Some(RightPosition)),        // Sign U
        0x0D42 => (Some(VowelDependent), Some(RightPosition)),        // Sign Uu
        0x0D43 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic R
        0x0D44 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic Rr
        0x0D45 => (None, None),                                       // unassigned
        0x0D46 => (Some(VowelDependent), Some(LeftPosition)),         // Sign E
        0x0D47 => (Some(VowelDependent), Some(LeftPosition)),         // Sign Ee
        0x0D48 => (Some(VowelDependent), Some(LeftPosition)),         // Sign Ai
        0x0D49 => (None, None),                                       // unassigned
        0x0D4A => (Some(VowelDependent), Some(LeftAndRightPosition)), // Sign O
        0x0D4B => (Some(VowelDependent), Some(LeftAndRightPosition)), // Sign Oo
        0x0D4C => (Some(VowelDependent), Some(LeftAndRightPosition)), // Sign Au
        0x0D4D => (Some(Virama), Some(TopPosition)),                  // Virama
        0x0D4E => (Some(ConsonantPreRepha), None),                    // Dot Reph
        0x0D4F => (Some(Symbol), None),                               // Para
        0x0D50 => (None, None),                                       // unassigned
        0x0D51 => (None, None),                                       // unassigned
        0x0D52 => (None, None),                                       // unassigned
        0x0D53 => (None, None),                                       // unassigned
        0x0D54 => (Some(ConsonantDead), None),                        // Chillu M
        0x0D55 => (Some(ConsonantDead), None),                        // Chillu Y
        0x0D56 => (Some(ConsonantDead), None),                        // Chillu Lll
        0x0D57 => (Some(VowelDependent), Some(RightPosition)),        // Au Length Mark
        0x0D58 => (Some(Number), None),                               // Fraction 1/160
        0x0D59 => (Some(Number), None),                               // Fraction 1/40
        0x0D5A => (Some(Number), None),                               // Fraction 3/80
        0x0D5B => (Some(Number), None),                               // Fraction 1/20
        0x0D5C => (Some(Number), None),                               // Fraction 1/10
        0x0D5D => (Some(Number), None),                               // Fraction 3/20
        0x0D5E => (Some(Number), None),                               // Fraction 1/5
        0x0D5F => (Some(VowelIndependent), None),                     // Archaic Ii
        0x0D60 => (Some(VowelIndependent), None),                     // Vocalic Rr
        0x0D61 => (Some(VowelIndependent), None),                     // Vocalic Ll
        0x0D62 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic L
        0x0D63 => (Some(VowelDependent), Some(BottomPosition)),       // Sign Vocalic Ll
        0x0D64 => (None, None),                                       // unassigned
        0x0D65 => (None, None),                                       // unassigned
        0x0D66 => (Some(Number), None),                               // Digit Zero
        0x0D67 => (Some(Number), None),                               // Digit One
        0x0D68 => (Some(Number), None),                               // Digit Two
        0x0D69 => (Some(Number), None),                               // Digit Three
        0x0D6A => (Some(Number), None),                               // Digit Four
        0x0D6B => (Some(Number), None),                               // Digit Five
        0x0D6C => (Some(Number), None),                               // Digit Six
        0x0D6D => (Some(Number), None),                               // Digit Seven
        0x0D6E => (Some(Number), None),                               // Digit Eight
        0x0D6F => (Some(Number), None),                               // Digit Nine
        0x0D70 => (Some(Number), None),                               // Number Ten
        0x0D71 => (Some(Number), None),                               // Number One Hundred
        0x0D72 => (Some(Number), None),                               // Number One Thousand
        0x0D73 => (Some(Number), None),                               // Fraction 1/4
        0x0D74 => (Some(Number), None),                               // Fraction 1/2
        0x0D75 => (Some(Number), None),                               // Fraction 3/4
        0x0D76 => (Some(Number), None),                               // Fraction 1/16
        0x0D77 => (Some(Number), None),                               // Fraction 1/8
        0x0D78 => (Some(Number), None),                               // Fraction 3/16
        0x0D79 => (Some(Symbol), None),                               // Date Mark
        0x0D7A => (Some(ConsonantDead), None),                        // Chillu Nn
        0x0D7B => (Some(ConsonantDead), None),                        // Chillu N
        0x0D7C => (Some(ConsonantDead), None),                        // Chillu Rr
        0x0D7D => (Some(ConsonantDead), None),                        // Chillu L
        0x0D7E => (Some(ConsonantDead), None),                        // Chillu Ll
        0x0D7F => (Some(ConsonantDead), None),                        // Chillu K

        // Sinhala character table
        0x0D80 => (None, None),                                          // unassigned
        0x0D81 => (None, None),                                          // unassigned
        0x0D82 => (Some(Bindu), Some(RightPosition)),                    // Anusvara
        0x0D83 => (Some(Visarga), Some(RightPosition)),                  // Visarga
        0x0D84 => (None, None),                                          // unassigned
        0x0D85 => (Some(VowelIndependent), None),                        // A
        0x0D86 => (Some(VowelIndependent), None),                        // Aa
        0x0D87 => (Some(VowelIndependent), None),                        // Ae
        0x0D88 => (Some(VowelIndependent), None),                        // Aae
        0x0D89 => (Some(VowelIndependent), None),                        // I
        0x0D8A => (Some(VowelIndependent), None),                        // Ii
        0x0D8B => (Some(VowelIndependent), None),                        // U
        0x0D8C => (Some(VowelIndependent), None),                        // Uu
        0x0D8D => (Some(VowelIndependent), None),                        // Vocalic R
        0x0D8E => (Some(VowelIndependent), None),                        // Vocalic Rr
        0x0D8F => (Some(VowelIndependent), None),                        // Vocalic L
        0x0D90 => (Some(VowelIndependent), None),                        // Vocalic Ll
        0x0D91 => (Some(VowelIndependent), None),                        // E
        0x0D92 => (Some(VowelIndependent), None),                        // Ee
        0x0D93 => (Some(VowelIndependent), None),                        // Ai
        0x0D94 => (Some(VowelIndependent), None),                        // O
        0x0D95 => (Some(VowelIndependent), None),                        // Oo
        0x0D96 => (Some(VowelIndependent), None),                        // Au
        0x0D97 => (None, None),                                          // unassigned
        0x0D98 => (None, None),                                          // unassigned
        0x0D99 => (None, None),                                          // unassigned
        0x0D9A => (Some(Consonant), None),                               // Ka
        0x0D9B => (Some(Consonant), None),                               // Kha
        0x0D9C => (Some(Consonant), None),                               // Ga
        0x0D9D => (Some(Consonant), None),                               // Gha
        0x0D9E => (Some(Consonant), None),                               // Nga
        0x0D9F => (Some(Consonant), None),                               // Nnga
        0x0DA0 => (Some(Consonant), None),                               // Ca
        0x0DA1 => (Some(Consonant), None),                               // Cha
        0x0DA2 => (Some(Consonant), None),                               // Ja
        0x0DA3 => (Some(Consonant), None),                               // Jha
        0x0DA4 => (Some(Consonant), None),                               // Nya
        0x0DA5 => (Some(Consonant), None),                               // Jnya
        0x0DA6 => (Some(Consonant), None),                               // Nyja
        0x0DA7 => (Some(Consonant), None),                               // Tta
        0x0DA8 => (Some(Consonant), None),                               // Ttha
        0x0DA9 => (Some(Consonant), None),                               // Dda
        0x0DAA => (Some(Consonant), None),                               // Ddha
        0x0DAB => (Some(Consonant), None),                               // Nna
        0x0DAC => (Some(Consonant), None),                               // Nndda
        0x0DAD => (Some(Consonant), None),                               // Ta
        0x0DAE => (Some(Consonant), None),                               // Tha
        0x0DAF => (Some(Consonant), None),                               // Da
        0x0DB0 => (Some(Consonant), None),                               // Dha
        0x0DB1 => (Some(Consonant), None),                               // Na
        0x0DB2 => (None, None),                                          // unassigned
        0x0DB3 => (Some(Consonant), None),                               // Nda
        0x0DB4 => (Some(Consonant), None),                               // Pa
        0x0DB5 => (Some(Consonant), None),                               // Pha
        0x0DB6 => (Some(Consonant), None),                               // Ba
        0x0DB7 => (Some(Consonant), None),                               // Bha
        0x0DB8 => (Some(Consonant), None),                               // Ma
        0x0DB9 => (Some(Consonant), None),                               // Mba
        0x0DBA => (Some(Consonant), None),                               // Ya
        0x0DBB => (Some(Consonant), None),                               // Ra
        0x0DBC => (None, None),                                          // unassigned
        0x0DBD => (Some(Consonant), None),                               // La
        0x0DBE => (None, None),                                          // unassigned
        0x0DBF => (None, None),                                          // unassigned
        0x0DC0 => (Some(Consonant), None),                               // Va
        0x0DC1 => (Some(Consonant), None),                               // Sha
        0x0DC2 => (Some(Consonant), None),                               // Ssa
        0x0DC3 => (Some(Consonant), None),                               // Sa
        0x0DC4 => (Some(Consonant), None),                               // Ha
        0x0DC5 => (Some(Consonant), None),                               // Lla
        0x0DC6 => (Some(Consonant), None),                               // Fa
        0x0DC7 => (None, None),                                          // unassigned
        0x0DC8 => (None, None),                                          // unassigned
        0x0DC9 => (None, None),                                          // unassigned
        0x0DCA => (Some(Virama), Some(TopPosition)),                     // Virama
        0x0DCB => (None, None),                                          // unassigned
        0x0DCC => (None, None),                                          // unassigned
        0x0DCD => (None, None),                                          // unassigned
        0x0DCE => (None, None),                                          // unassigned
        0x0DCF => (Some(VowelDependent), Some(RightPosition)),           // Sign Aa
        0x0DD0 => (Some(VowelDependent), Some(RightPosition)),           // Sign Ae
        0x0DD1 => (Some(VowelDependent), Some(RightPosition)),           // Sign Aae
        0x0DD2 => (Some(VowelDependent), Some(TopPosition)),             // Sign I
        0x0DD3 => (Some(VowelDependent), Some(TopPosition)),             // Sign Ii
        0x0DD4 => (Some(VowelDependent), Some(BottomPosition)),          // Sign U
        0x0DD5 => (None, None),                                          // unassigned
        0x0DD6 => (Some(VowelDependent), Some(BottomPosition)),          // Sign Uu
        0x0DD7 => (None, None),                                          // unassigned
        0x0DD8 => (Some(VowelDependent), Some(RightPosition)),           // Sign Vocalic R
        0x0DD9 => (Some(VowelDependent), Some(LeftPosition)),            // Sign E
        0x0DDA => (Some(VowelDependent), Some(TopAndLeftPosition)),      // Sign Ee
        0x0DDB => (Some(VowelDependent), Some(LeftPosition)),            // Sign Ai
        0x0DDC => (Some(VowelDependent), Some(LeftAndRightPosition)),    // Sign O
        0x0DDD => (Some(VowelDependent), Some(TopLeftAndRightPosition)), // Sign Oo
        0x0DDE => (Some(VowelDependent), Some(LeftAndRightPosition)),    // Sign Au
        0x0DDF => (Some(VowelDependent), Some(RightPosition)),           // Sign Vocalic L
        0x0DE0 => (None, None),                                          // unassigned
        0x0DE1 => (None, None),                                          // unassigned
        0x0DE2 => (None, None),                                          // unassigned
        0x0DE3 => (None, None),                                          // unassigned
        0x0DE4 => (None, None),                                          // unassigned
        0x0DE5 => (None, None),                                          // unassigned
        0x0DE6 => (Some(Number), None),                                  // Digit Zero
        0x0DE7 => (Some(Number), None),                                  // Digit One
        0x0DE8 => (Some(Number), None),                                  // Digit Two
        0x0DE9 => (Some(Number), None),                                  // Digit Three
        0x0DEA => (Some(Number), None),                                  // Digit Four
        0x0DEB => (Some(Number), None),                                  // Digit Five
        0x0DEC => (Some(Number), None),                                  // Digit Six
        0x0DED => (Some(Number), None),                                  // Digit Seven
        0x0DEE => (Some(Number), None),                                  // Digit Eight
        0x0DEF => (Some(Number), None),                                  // Digit Nine
        0x0DF0 => (None, None),                                          // unassigned
        0x0DF1 => (None, None),                                          // unassigned
        0x0DF2 => (Some(VowelDependent), Some(RightPosition)),           // Sign Vocalic Rr
        0x0DF3 => (Some(VowelDependent), Some(RightPosition)),           // Sign Vocalic Ll
        0x0DF4 => (None, None),                                          // Kunddaliya
        0x0DF5 => (None, None),                                          // unassigned
        0x0DF6 => (None, None),                                          // unassigned
        0x0DF7 => (None, None),                                          // unassigned
        0x0DF8 => (None, None),                                          // unassigned
        0x0DF9 => (None, None),                                          // unassigned
        0x0DFA => (None, None),                                          // unassigned
        0x0DFB => (None, None),                                          // unassigned
        0x0DFC => (None, None),                                          // unassigned
        0x0DFD => (None, None),                                          // unassigned
        0x0DFE => (None, None),                                          // unassigned
        0x0DFF => (None, None),                                          // unassigned

        // Vedic Extensions character table
        0x1CD0 => (Some(Cantillation), Some(TopPosition)),    // Tone Karshana
        0x1CD1 => (Some(Cantillation), Some(TopPosition)),    // Tone Shara
        0x1CD2 => (Some(Cantillation), Some(TopPosition)),    // Tone Prenkha
        0x1CD3 => (None, None),                               // Sign Nihshvasa
        0x1CD4 => (Some(Cantillation), Some(Overstruck)),     // Tone Midline Svarita
        0x1CD5 => (Some(Cantillation), Some(BottomPosition)), // Tone Aggravated Independent Svarita
        0x1CD6 => (Some(Cantillation), Some(BottomPosition)), // Tone Independent Svarita
        0x1CD7 => (Some(Cantillation), Some(BottomPosition)), // Tone Kathaka Independent Svarita
        0x1CD8 => (Some(Cantillation), Some(BottomPosition)), // Tone Candra Below
        0x1CD9 => (Some(Cantillation), Some(BottomPosition)), // Tone Kathaka Independent Svarita Schroeder
        0x1CDA => (Some(Cantillation), Some(TopPosition)),    // Tone Double Svarita
        0x1CDB => (Some(Cantillation), Some(TopPosition)),    // Tone Triple Svarita
        0x1CDC => (Some(Cantillation), Some(BottomPosition)), // Tone Kathaka Anudatta
        0x1CDD => (Some(Cantillation), Some(BottomPosition)), // Tone Dot Below
        0x1CDE => (Some(Cantillation), Some(BottomPosition)), // Tone Two Dots Below
        0x1CDF => (Some(Cantillation), Some(BottomPosition)), // Tone Three Dots Below
        0x1CE0 => (Some(Cantillation), Some(TopPosition)),    // Tone Rigvedic Kashmiri Independent Svarita
        0x1CE1 => (Some(Cantillation), Some(RightPosition)),  // Tone Atharavedic Independent Svarita
        0x1CE2 => (Some(Avagraha), Some(Overstruck)),         // Sign Visarga Svarita
        0x1CE3 => (None, Some(Overstruck)),                   // Sign Visarga Udatta
        0x1CE4 => (None, Some(Overstruck)),                   // Sign Reversed Visarga Udatta
        0x1CE5 => (None, Some(Overstruck)),                   // Sign Visarga Anudatta
        0x1CE6 => (None, Some(Overstruck)),                   // Sign Reversed Visarga Anudatta
        0x1CE7 => (None, Some(Overstruck)),                   // Sign Visarga Udatta With Tail
        0x1CE8 => (Some(Avagraha), Some(Overstruck)),         // Sign Visarga Anudatta With Tail
        0x1CE9 => (Some(Symbol), None),                       // Sign Anusvara Antargomukha
        0x1CEA => (None, None),                               // Sign Anusvara Bahirgomukha
        0x1CEB => (None, None),                               // Sign Anusvara Vamagomukha
        0x1CEC => (Some(Symbol), None),                       // Sign Anusvara Vamagomukha With Tail
        0x1CED => (Some(Avagraha), Some(BottomPosition)),     // Sign Tiryak
        0x1CEE => (Some(Symbol), None),                       // Sign Hexiform Long Anusvara
        0x1CEF => (None, None),                               // Sign Long Anusvara
        0x1CF0 => (None, None),                               // Sign Rthang Long Anusvara
        0x1CF1 => (Some(Symbol), None),                       // Sign Anusvara Ubhayato Mukha
        0x1CF2 => (Some(Visarga), None),                      // Sign Ardhavisarga
        0x1CF3 => (Some(Visarga), None),                      // Sign Rotated Ardhavisarga
        0x1CF4 => (Some(Cantillation), Some(TopPosition)),    // Tone Candra Above
        0x1CF5 => (Some(Consonant), None),                    // Sign Jihvamuliya
        0x1CF6 => (Some(Consonant), None),                    // Sign Upadhmaniya
        0x1CF7 => (None, None),                               // Sign Atikrama
        0x1CF8 => (Some(Cantillation), None),                 // Tone Ring Above
        0x1CF9 => (Some(Cantillation), None),                 // Tone Double Ring Above

        // Devanagari Extended character table
        0xA8E0 => (Some(Cantillation), Some(TopPosition)),   // Combining Zero
        0xA8E1 => (Some(Cantillation), Some(TopPosition)),   // Combining One
        0xA8E2 => (Some(Cantillation), Some(TopPosition)),   // Combining Two
        0xA8E3 => (Some(Cantillation), Some(TopPosition)),   // Combining Three
        0xA8E4 => (Some(Cantillation), Some(TopPosition)),   // Combining Four
        0xA8E5 => (Some(Cantillation), Some(TopPosition)),   // Combining Five
        0xA8E6 => (Some(Cantillation), Some(TopPosition)),   // Combining Six
        0xA8E7 => (Some(Cantillation), Some(TopPosition)),   // Combining Seven
        0xA8E8 => (Some(Cantillation), Some(TopPosition)),   // Combining Eight
        0xA8E9 => (Some(Cantillation), Some(TopPosition)),   // Combining Nine
        0xA8EA => (Some(Cantillation), Some(TopPosition)),   // Combining A
        0xA8EB => (Some(Cantillation), Some(TopPosition)),   // Combining U
        0xA8EC => (Some(Cantillation), Some(TopPosition)),   // Combining Ka
        0xA8ED => (Some(Cantillation), Some(TopPosition)),   // Combining Na
        0xA8EE => (Some(Cantillation), Some(TopPosition)),   // Combining Pa
        0xA8EF => (Some(Cantillation), Some(TopPosition)),   // Combining Ra
        0xA8F0 => (Some(Cantillation), Some(TopPosition)),   // Combining Vi
        0xA8F1 => (Some(Cantillation), Some(TopPosition)),   // Combining Avagraha
        0xA8F2 => (Some(Bindu), None),                       // Spacing Candrabindu
        0xA8F3 => (Some(Bindu), None),                       // Candrabindu Virama
        0xA8F4 => (None, None),                              // Double Candrabindu Virama
        0xA8F5 => (None, None),                              // Candrabindu Two
        0xA8F6 => (None, None),                              // Candrabindu Three
        0xA8F7 => (None, None),                              // Candrabindu Avagraha
        0xA8F8 => (None, None),                              // Pushpika
        0xA8F9 => (None, None),                              // Gap Filler
        0xA8FA => (None, None),                              // Caret
        0xA8FB => (None, None),                              // Headstroke
        0xA8FC => (None, None),                              // Siddham
        0xA8FD => (None, None),                              // Jain Om
        0xA8FE => (Some(VowelIndependent), None),            // Ay
        0xA8FF => (Some(VowelDependent), Some(TopPosition)), // Sign Ay

        // Sinhala Archaic Numbers character table
        0x111E0 => (None, None),         // unassigned
        0x111E1 => (Some(Number), None), // Archaic Digit One
        0x111E2 => (Some(Number), None), // Archaic Digit Two
        0x111E3 => (Some(Number), None), // Archaic Digit Three
        0x111E4 => (Some(Number), None), // Archaic Digit Four
        0x111E5 => (Some(Number), None), // Archaic Digit Five
        0x111E6 => (Some(Number), None), // Archaic Digit Six
        0x111E7 => (Some(Number), None), // Archaic Digit Seven
        0x111E8 => (Some(Number), None), // Archaic Digit Eight
        0x111E9 => (Some(Number), None), // Archaic Digit Nine
        0x111EA => (Some(Number), None), // Archaic Number Ten
        0x111EB => (Some(Number), None), // Archaic Number 20
        0x111EC => (Some(Number), None), // Archaic Number 30
        0x111ED => (Some(Number), None), // Archaic Number 40
        0x111EE => (Some(Number), None), // Archaic Number 50
        0x111EF => (Some(Number), None), // Archaic Number 60
        0x111F0 => (Some(Number), None), // Archaic Number 70
        0x111F1 => (Some(Number), None), // Archaic Number 80
        0x111F2 => (Some(Number), None), // Archaic Number 90
        0x111F3 => (Some(Number), None), // Archaic Number 100
        0x111F4 => (Some(Number), None), // Archaic Number 1000
        0x111F5 => (None, None),         // unassigned
        0x111F6 => (None, None),         // unassigned
        0x111F7 => (None, None),         // unassigned
        0x111F8 => (None, None),         // unassigned
        0x111F9 => (None, None),         // unassigned
        0x111FA => (None, None),         // unassigned
        0x111FB => (None, None),         // unassigned
        0x111FC => (None, None),         // unassigned
        0x111FD => (None, None),         // unassigned
        0x111FE => (None, None),         // unassigned
        0x111FF => (None, None),         // unassigned

        // Grantha marks character table
        0x11301 => (Some(Bindu), Some(TopPosition)),     // Grantha Candrabindu
        0x11303 => (Some(Visarga), Some(RightPosition)), // Grantha Visarga
        0x1133C => (Some(Nukta), Some(BottomPosition)),  // Grantha Nukta

        // Miscellaneous character table
        0x00A0 => (Some(Placeholder), None),      // No-break space
        0x00B2 => (Some(SyllableModifier), None), // Superscript Two (used in Tamil)
        0x00B3 => (Some(SyllableModifier), None), // Superscript Three (used in Tamil)
        0x200C => (Some(NonJoiner), None),        // Zero-width non-joiner
        0x200D => (Some(Joiner), None),           // Zero-width joiner
        0x2010 => (Some(Placeholder), None),      // Hyphen
        0x2011 => (Some(Placeholder), None),      // No-break hyphen
        0x2012 => (Some(Placeholder), None),      // Figure dash
        0x2013 => (Some(Placeholder), None),      // En dash
        0x2014 => (Some(Placeholder), None),      // Em dash
        0x2074 => (Some(SyllableModifier), None), // Superscript Four (used in Tamil)
        0x2082 => (Some(SyllableModifier), None), // Subscript Two (used in Tamil)
        0x2083 => (Some(SyllableModifier), None), // Subscript Three (used in Tamil)
        0x2084 => (Some(SyllableModifier), None), // Subscript Four (used in Tamil)
        0x25CC => (Some(DottedCircle), None),     // Dotted circle

        _ => (None, None),
    }
}

fn indic_name(ch: char) -> Option<&'static str> {
    match ch as u32 {
        // Devanagari character table
        0x0900 => Some("Inverted Candrabindu"),
        0x0901 => Some("Candrabindu"),
        0x0902 => Some("Anusvara"),
        0x0903 => Some("Visarga"),
        0x0904 => Some("Short A"),
        0x0905 => Some("A"),
        0x0906 => Some("Aa"),
        0x0907 => Some("I"),
        0x0908 => Some("Ii"),
        0x0909 => Some("U"),
        0x090A => Some("Uu"),
        0x090B => Some("Vocalic R"),
        0x090C => Some("Vocalic L"),
        0x090D => Some("Candra E"),
        0x090E => Some("Short E"),
        0x090F => Some("E"),
        0x0910 => Some("Ai"),
        0x0911 => Some("Candra O"),
        0x0912 => Some("Short O"),
        0x0913 => Some("O"),
        0x0914 => Some("Au"),
        0x0915 => Some("Ka"),
        0x0916 => Some("Kha"),
        0x0917 => Some("Ga"),
        0x0918 => Some("Gha"),
        0x0919 => Some("Nga"),
        0x091A => Some("Ca"),
        0x091B => Some("Cha"),
        0x091C => Some("Ja"),
        0x091D => Some("Jha"),
        0x091E => Some("Nya"),
        0x091F => Some("Tta"),
        0x0920 => Some("Ttha"),
        0x0921 => Some("Dda"),
        0x0922 => Some("Ddha"),
        0x0923 => Some("Nna"),
        0x0924 => Some("Ta"),
        0x0925 => Some("Tha"),
        0x0926 => Some("Da"),
        0x0927 => Some("Dha"),
        0x0928 => Some("Na"),
        0x0929 => Some("Nnna"),
        0x092A => Some("Pa"),
        0x092B => Some("Pha"),
        0x092C => Some("Ba"),
        0x092D => Some("Bha"),
        0x092E => Some("Ma"),
        0x092F => Some("Ya"),
        0x0930 => Some("Ra"),
        0x0931 => Some("Rra"),
        0x0932 => Some("La"),
        0x0933 => Some("Lla"),
        0x0934 => Some("Llla"),
        0x0935 => Some("Va"),
        0x0936 => Some("Sha"),
        0x0937 => Some("Ssa"),
        0x0938 => Some("Sa"),
        0x0939 => Some("Ha"),
        0x093A => Some("Sign Oe"),
        0x093B => Some("Sign Ooe"),
        0x093C => Some("Nukta"),
        0x093D => Some("Avagraha"),
        0x093E => Some("Sign Aa"),
        0x093F => Some("Sign I"),
        0x0940 => Some("Sign Ii"),
        0x0941 => Some("Sign U"),
        0x0942 => Some("Sign Uu"),
        0x0943 => Some("Sign Vocalic R"),
        0x0944 => Some("Sign Vocalic Rr"),
        0x0945 => Some("Sign Candra E"),
        0x0946 => Some("Sign Short E"),
        0x0947 => Some("Sign E"),
        0x0948 => Some("Sign Ai"),
        0x0949 => Some("Sign Candra O"),
        0x094A => Some("Sign Short O"),
        0x094B => Some("Sign O"),
        0x094C => Some("Sign Au"),
        0x094D => Some("Virama"),
        0x094E => Some("Sign Prishthamatra E"),
        0x094F => Some("Sign Aw"),
        0x0950 => Some("Om"),
        0x0951 => Some("Udatta"),
        0x0952 => Some("Anudatta"),
        0x0953 => Some("Grave accent"),
        0x0954 => Some("Acute accent"),
        0x0955 => Some("Sign Candra Long E"),
        0x0956 => Some("Sign Ue"),
        0x0957 => Some("Sign Uue"),
        0x0958 => Some("Qa"),
        0x0959 => Some("Khha"),
        0x095A => Some("Ghha"),
        0x095B => Some("Za"),
        0x095C => Some("Dddha"),
        0x095D => Some("Rha"),
        0x095E => Some("Fa"),
        0x095F => Some("Yya"),
        0x0960 => Some("Vocalic Rr"),
        0x0961 => Some("Vocalic Ll"),
        0x0962 => Some("Sign Vocalic L"),
        0x0963 => Some("Sign Vocalic Ll"),
        0x0964 => Some("Danda"),
        0x0965 => Some("Double Danda"),
        0x0966 => Some("Digit Zero"),
        0x0967 => Some("Digit One"),
        0x0968 => Some("Digit Two"),
        0x0969 => Some("Digit Three"),
        0x096A => Some("Digit Four"),
        0x096B => Some("Digit Five"),
        0x096C => Some("Digit Six"),
        0x096D => Some("Digit Seven"),
        0x096E => Some("Digit Eight"),
        0x096F => Some("Digit Nine"),
        0x0970 => Some("Abbreviation Sign"),
        0x0971 => Some("Sign High Spacing Dot"),
        0x0972 => Some("Candra Aa"),
        0x0973 => Some("Oe"),
        0x0974 => Some("Ooe"),
        0x0975 => Some("Aw"),
        0x0976 => Some("Ue"),
        0x0977 => Some("Uue"),
        0x0978 => Some("Marwari Dda"),
        0x0979 => Some("Zha"),
        0x097A => Some("Heavy Ya"),
        0x097B => Some("Gga"),
        0x097C => Some("Jja"),
        0x097D => Some("Glottal Stop"),
        0x097E => Some("Ddda"),
        0x097F => Some("Bba"),

        // Bengali character table
        0x0980 => Some("Anji"),
        0x0981 => Some("Candrabindu"),
        0x0982 => Some("Anusvara"),
        0x0983 => Some("Visarga"),
        0x0984 => None,
        0x0985 => Some("A"),
        0x0986 => Some("Aa"),
        0x0987 => Some("I"),
        0x0988 => Some("Ii"),
        0x0989 => Some("U"),
        0x098A => Some("Uu"),
        0x098B => Some("Vocalic R"),
        0x098C => Some("Vocalic L"),
        0x098D => None,
        0x098E => None,
        0x098F => Some("E"),
        0x0990 => Some("Ai"),
        0x0991 => None,
        0x0992 => None,
        0x0993 => Some("O"),
        0x0994 => Some("Au"),
        0x0995 => Some("Ka"),
        0x0996 => Some("Kha"),
        0x0997 => Some("Ga"),
        0x0998 => Some("Gha"),
        0x0999 => Some("Nga"),
        0x099A => Some("Ca"),
        0x099B => Some("Cha"),
        0x099C => Some("Ja"),
        0x099D => Some("Jha"),
        0x099E => Some("Nya"),
        0x099F => Some("Tta"),
        0x09A0 => Some("Ttha"),
        0x09A1 => Some("Dda"),
        0x09A2 => Some("Ddha"),
        0x09A3 => Some("Nna"),
        0x09A4 => Some("Ta"),
        0x09A5 => Some("Tha"),
        0x09A6 => Some("Da"),
        0x09A7 => Some("Dha"),
        0x09A8 => Some("Na"),
        0x09A9 => None,
        0x09AA => Some("Pa"),
        0x09AB => Some("Pha"),
        0x09AC => Some("Ba"),
        0x09AD => Some("Bha"),
        0x09AE => Some("Ma"),
        0x09AF => Some("Ya"),
        0x09B0 => Some("Ra"),
        0x09B1 => None,
        0x09B2 => Some("La"),
        0x09B3 => None,
        0x09B4 => None,
        0x09B5 => None,
        0x09B6 => Some("Sha"),
        0x09B7 => Some("Ssa"),
        0x09B8 => Some("Sa"),
        0x09B9 => Some("Ha"),
        0x09BA => None,
        0x09BB => None,
        0x09BC => Some("Nukta"),
        0x09BD => Some("Avagraha"),
        0x09BE => Some("Sign Aa"),
        0x09BF => Some("Sign I"),
        0x09C0 => Some("Sign Ii"),
        0x09C1 => Some("Sign U"),
        0x09C2 => Some("Sign Uu"),
        0x09C3 => Some("Sign Vocalic R"),
        0x09C4 => Some("Sign Vocalic Rr"),
        0x09C5 => None,
        0x09C6 => None,
        0x09C7 => Some("Sign E"),
        0x09C8 => Some("Sign Ai"),
        0x09C9 => None,
        0x09CA => None,
        0x09CB => Some("Sign O"),
        0x09CC => Some("Sign Au"),
        0x09CD => Some("Virama"),
        0x09CE => Some("Khanda Ta"),
        0x09CF => None,
        0x09D0 => None,
        0x09D1 => None,
        0x09D2 => None,
        0x09D3 => None,
        0x09D4 => None,
        0x09D5 => None,
        0x09D6 => None,
        0x09D7 => Some("Au Length Mark"),
        0x09D8 => None,
        0x09D9 => None,
        0x09DA => None,
        0x09DB => None,
        0x09DC => Some("Rra"),
        0x09DD => Some("Rha"),
        0x09DE => None,
        0x09DF => Some("Yya"),
        0x09E0 => Some("Vocalic Rr"),
        0x09E1 => Some("Vocalic Ll"),
        0x09E2 => Some("Sign Vocalic L"),
        0x09E3 => Some("Sign Vocalic Ll"),
        0x09E4 => None,
        0x09E5 => None,
        0x09E6 => Some("Digit Zero"),
        0x09E7 => Some("Digit One"),
        0x09E8 => Some("Digit Two"),
        0x09E9 => Some("Digit Three"),
        0x09EA => Some("Digit Four"),
        0x09EB => Some("Digit Five"),
        0x09EC => Some("Digit Six"),
        0x09ED => Some("Digit Seven"),
        0x09EE => Some("Digit Eight"),
        0x09EF => Some("Digit Nine"),
        0x09F0 => Some("Assamese Ra"),
        0x09F1 => Some("Assamese Wa"),
        0x09F2 => Some("Rupee Mark"),
        0x09F3 => Some("Rupee Sign"),
        0x09F4 => Some("Numerator One"),
        0x09F5 => Some("Numerator Two"),
        0x09F6 => Some("Numerator Three"),
        0x09F7 => Some("Numerator Four"),
        0x09F8 => Some("Numerator One Less Than Denominator"),
        0x09F9 => Some("Denominator Sixteen"),
        0x09FA => Some("Isshar"),
        0x09FB => Some("Ganda Mark"),
        0x09FC => Some("Vedic Anusvara"),
        0x09FD => Some("Abbreviation Sign"),
        0x09FE => Some("Sandhi Mark"),

        // Gurmukhi character table
        0x0A00 => None,
        0x0A01 => Some("Adak Bindi"),
        0x0A02 => Some("Bindi"),
        0x0A03 => Some("Visarga"),
        0x0A04 => None,
        0x0A05 => Some("A"),
        0x0A06 => Some("Aa"),
        0x0A07 => Some("I"),
        0x0A08 => Some("Ii"),
        0x0A09 => Some("U"),
        0x0A0A => Some("Uu"),
        0x0A0B => None,
        0x0A0C => None,
        0x0A0D => None,
        0x0A0E => None,
        0x0A0F => Some("Ee"),
        0x0A10 => Some("Ai"),
        0x0A11 => None,
        0x0A12 => None,
        0x0A13 => Some("Oo"),
        0x0A14 => Some("Au"),
        0x0A15 => Some("Ka"),
        0x0A16 => Some("Kha"),
        0x0A17 => Some("Ga"),
        0x0A18 => Some("Gha"),
        0x0A19 => Some("Nga"),
        0x0A1A => Some("Ca"),
        0x0A1B => Some("Cha"),
        0x0A1C => Some("Ja"),
        0x0A1D => Some("Jha"),
        0x0A1E => Some("Nya"),
        0x0A1F => Some("Tta"),
        0x0A20 => Some("Ttha"),
        0x0A21 => Some("Dda"),
        0x0A22 => Some("Ddha"),
        0x0A23 => Some("Nna"),
        0x0A24 => Some("Ta"),
        0x0A25 => Some("Tha"),
        0x0A26 => Some("Da"),
        0x0A27 => Some("Dha"),
        0x0A28 => Some("Na"),
        0x0A29 => None,
        0x0A2A => Some("Pa"),
        0x0A2B => Some("Pha"),
        0x0A2C => Some("Ba"),
        0x0A2D => Some("Bha"),
        0x0A2E => Some("Ma"),
        0x0A2F => Some("Ya"),
        0x0A30 => Some("Ra"),
        0x0A31 => None,
        0x0A32 => Some("La"),
        0x0A33 => Some("Lla"),
        0x0A34 => None,
        0x0A35 => Some("Va"),
        0x0A36 => Some("Sha"),
        0x0A37 => None,
        0x0A38 => Some("Sa"),
        0x0A39 => Some("Ha"),
        0x0A3A => None,
        0x0A3B => None,
        0x0A3C => Some("Nukta"),
        0x0A3D => None,
        0x0A3E => Some("Sign Aa"),
        0x0A3F => Some("Sign I"),
        0x0A40 => Some("Sign Ii"),
        0x0A41 => Some("Sign U"),
        0x0A42 => Some("Sign Uu"),
        0x0A43 => None,
        0x0A44 => None,
        0x0A45 => None,
        0x0A46 => None,
        0x0A47 => Some("Sign Ee"),
        0x0A48 => Some("Sign Ai"),
        0x0A49 => None,
        0x0A4A => None,
        0x0A4B => Some("Sign Oo"),
        0x0A4C => Some("Sign Au"),
        0x0A4D => Some("Virama"),
        0x0A4E => None,
        0x0A4F => None,
        0x0A50 => None,
        0x0A51 => Some("Udaat"),
        0x0A52 => None,
        0x0A53 => None,
        0x0A54 => None,
        0x0A55 => None,
        0x0A56 => None,
        0x0A57 => None,
        0x0A58 => None,
        0x0A59 => Some("Khha"),
        0x0A5A => Some("Ghha"),
        0x0A5B => Some("Za"),
        0x0A5C => Some("Rra"),
        0x0A5D => None,
        0x0A5E => Some("Fa"),
        0x0A5F => None,
        0x0A60 => None,
        0x0A61 => None,
        0x0A62 => None,
        0x0A63 => None,
        0x0A64 => None,
        0x0A65 => None,
        0x0A66 => Some("Digit Zero"),
        0x0A67 => Some("Digit One"),
        0x0A68 => Some("Digit Two"),
        0x0A69 => Some("Digit Three"),
        0x0A6A => Some("Digit Four"),
        0x0A6B => Some("Digit Five"),
        0x0A6C => Some("Digit Six"),
        0x0A6D => Some("Digit Seven"),
        0x0A6E => Some("Digit Eight"),
        0x0A6F => Some("Digit Nine"),
        0x0A70 => Some("Tippi"),
        0x0A71 => Some("Addak"),
        0x0A72 => Some("Iri"),
        0x0A73 => Some("Ura"),
        0x0A74 => Some("Ek Onkar"),
        0x0A75 => Some("Yakash"),
        0x0A76 => Some("Abbreviation Sign"),

        // Gujarati character table
        0x0A81 => Some("Candrabindu"),
        0x0A82 => Some("Anusvara"),
        0x0A83 => Some("Visarga"),
        0x0A84 => None,
        0x0A85 => Some("A"),
        0x0A86 => Some("Aa"),
        0x0A87 => Some("I"),
        0x0A88 => Some("Ii"),
        0x0A89 => Some("U"),
        0x0A8A => Some("Uu"),
        0x0A8B => Some("Vocalic R"),
        0x0A8C => Some("Vocalic L"),
        0x0A8D => Some("Candra E"),
        0x0A8E => None,
        0x0A8F => Some("E"),
        0x0A90 => Some("Ai"),
        0x0A91 => Some("Candra O"),
        0x0A92 => None,
        0x0A93 => Some("O"),
        0x0A94 => Some("Au"),
        0x0A95 => Some("Ka"),
        0x0A96 => Some("Kha"),
        0x0A97 => Some("Ga"),
        0x0A98 => Some("Gha"),
        0x0A99 => Some("Nga"),
        0x0A9A => Some("Ca"),
        0x0A9B => Some("Cha"),
        0x0A9C => Some("Ja"),
        0x0A9D => Some("Jha"),
        0x0A9E => Some("Nya"),
        0x0A9F => Some("Tta"),
        0x0AA0 => Some("Ttha"),
        0x0AA1 => Some("Dda"),
        0x0AA2 => Some("Ddha"),
        0x0AA3 => Some("Nna"),
        0x0AA4 => Some("Ta"),
        0x0AA5 => Some("Tha"),
        0x0AA6 => Some("Da"),
        0x0AA7 => Some("Dha"),
        0x0AA8 => Some("Na"),
        0x0AA9 => None,
        0x0AAA => Some("Pa"),
        0x0AAB => Some("Pha"),
        0x0AAC => Some("Ba"),
        0x0AAD => Some("Bha"),
        0x0AAE => Some("Ma"),
        0x0AAF => Some("Ya"),
        0x0AB0 => Some("Ra"),
        0x0AB1 => None,
        0x0AB2 => Some("La"),
        0x0AB3 => Some("Lla"),
        0x0AB4 => None,
        0x0AB5 => Some("Va"),
        0x0AB6 => Some("Sha"),
        0x0AB7 => Some("Ssa"),
        0x0AB8 => Some("Sa"),
        0x0AB9 => Some("Ha"),
        0x0ABA => None,
        0x0ABB => None,
        0x0ABC => Some("Nukta"),
        0x0ABD => Some("Avagraha"),
        0x0ABE => Some("Sign Aa"),
        0x0ABF => Some("Sign I"),
        0x0AC0 => Some("Sign Ii"),
        0x0AC1 => Some("Sign U"),
        0x0AC2 => Some("Sign Uu"),
        0x0AC3 => Some("Sign Vocalic R"),
        0x0AC4 => Some("Sign Vocalic Rr"),
        0x0AC5 => Some("Sign Candra E"),
        0x0AC6 => None,
        0x0AC7 => Some("Sign E"),
        0x0AC8 => Some("Sign Ai"),
        0x0AC9 => Some("Sign Candra O"),
        0x0ACA => None,
        0x0ACB => Some("Sign O"),
        0x0ACC => Some("Sign Au"),
        0x0ACD => Some("Virama"),
        0x0ACE => None,
        0x0ACF => None,
        0x0AD0 => Some("Om"),
        0x0AD1 => None,
        0x0AD2 => None,
        0x0AD3 => None,
        0x0AD4 => None,
        0x0AD5 => None,
        0x0AD6 => None,
        0x0AD7 => None,
        0x0AD8 => None,
        0x0AD9 => None,
        0x0ADA => None,
        0x0ADB => None,
        0x0ADC => None,
        0x0ADD => None,
        0x0ADE => None,
        0x0ADF => None,
        0x0AE0 => Some("Vocalic Rr"),
        0x0AE1 => Some("Vocalic Ll"),
        0x0AE2 => Some("Sign Vocalic L"),
        0x0AE3 => Some("Sign Vocalic Ll"),
        0x0AE4 => None,
        0x0AE5 => None,
        0x0AE6 => Some("Digit Zero"),
        0x0AE7 => Some("Digit One"),
        0x0AE8 => Some("Digit Two"),
        0x0AE9 => Some("Digit Three"),
        0x0AEA => Some("Digit Four"),
        0x0AEB => Some("Digit Five"),
        0x0AEC => Some("Digit Six"),
        0x0AED => Some("Digit Seven"),
        0x0AEE => Some("Digit Eight"),
        0x0AEF => Some("Digit Nine"),
        0x0AF0 => Some("Abbreviation"),
        0x0AF1 => Some("Rupee Sign"),
        0x0AF2 => None,
        0x0AF3 => None,
        0x0AF4 => None,
        0x0AF5 => None,
        0x0AF6 => None,
        0x0AF7 => None,
        0x0AF8 => None,
        0x0AF9 => Some("Zha"),
        0x0AFA => Some("Sukun"),
        0x0AFB => Some("Shadda"),
        0x0AFC => Some("Maddah"),
        0x0AFD => Some("Three-Dot Nukta Above"),
        0x0AFE => Some("Circle Nukta Above"),
        0x0AFF => Some("Two-Circle Nukta Above"),

        // Oriya character table
        0x0B00 => None,
        0x0B01 => Some("Candrabindu"),
        0x0B02 => Some("Anusvara"),
        0x0B03 => Some("Visarga"),
        0x0B04 => None,
        0x0B05 => Some("A"),
        0x0B06 => Some("Aa"),
        0x0B07 => Some("I"),
        0x0B08 => Some("Ii"),
        0x0B09 => Some("U"),
        0x0B0A => Some("Uu"),
        0x0B0B => Some("Vocalic R"),
        0x0B0C => Some("Vocalic L"),
        0x0B0D => None,
        0x0B0E => None,
        0x0B0F => Some("E"),
        0x0B10 => Some("Ai"),
        0x0B11 => None,
        0x0B12 => None,
        0x0B13 => Some("O"),
        0x0B14 => Some("Au"),
        0x0B15 => Some("Ka"),
        0x0B16 => Some("Kha"),
        0x0B17 => Some("Ga"),
        0x0B18 => Some("Gha"),
        0x0B19 => Some("Nga"),
        0x0B1A => Some("Ca"),
        0x0B1B => Some("Cha"),
        0x0B1C => Some("Ja"),
        0x0B1D => Some("Jha"),
        0x0B1E => Some("Nya"),
        0x0B1F => Some("Tta"),
        0x0B20 => Some("Ttha"),
        0x0B21 => Some("Dda"),
        0x0B22 => Some("Ddha"),
        0x0B23 => Some("Nna"),
        0x0B24 => Some("Ta"),
        0x0B25 => Some("Tha"),
        0x0B26 => Some("Da"),
        0x0B27 => Some("Dha"),
        0x0B28 => Some("Na"),
        0x0B29 => None,
        0x0B2A => Some("Pa"),
        0x0B2B => Some("Pha"),
        0x0B2C => Some("Ba"),
        0x0B2D => Some("Bha"),
        0x0B2E => Some("Ma"),
        0x0B2F => Some("Ya"),
        0x0B30 => Some("Ra"),
        0x0B31 => None,
        0x0B32 => Some("La"),
        0x0B33 => Some("Lla"),
        0x0B34 => None,
        0x0B35 => Some("Va"),
        0x0B36 => Some("Sha"),
        0x0B37 => Some("Ssa"),
        0x0B38 => Some("Sa"),
        0x0B39 => Some("Ha"),
        0x0B3A => None,
        0x0B3B => None,
        0x0B3C => Some("Nukta"),
        0x0B3D => Some("Avagraha"),
        0x0B3E => Some("Sign Aa"),
        0x0B3F => Some("Sign I"),
        0x0B40 => Some("Sign Ii"),
        0x0B41 => Some("Sign U"),
        0x0B42 => Some("Sign Uu"),
        0x0B43 => Some("Sign Vocalic R"),
        0x0B44 => Some("Sign Vocalic Rr"),
        0x0B45 => None,
        0x0B46 => None,
        0x0B47 => Some("Sign E"),
        0x0B48 => Some("Sign Ai"),
        0x0B49 => None,
        0x0B4A => None,
        0x0B4B => Some("Sign O"),
        0x0B4C => Some("Sign Au"),
        0x0B4D => Some("Virama"),
        0x0B4E => None,
        0x0B4F => None,
        0x0B50 => None,
        0x0B51 => None,
        0x0B52 => None,
        0x0B53 => None,
        0x0B54 => None,
        0x0B55 => None,
        0x0B56 => Some("Ai Length Mark"),
        0x0B57 => Some("Au Length Mark"),
        0x0B58 => None,
        0x0B59 => None,
        0x0B5A => None,
        0x0B5B => None,
        0x0B5C => Some("Rra"),
        0x0B5D => Some("Rha"),
        0x0B5E => None,
        0x0B5F => Some("Yya"),
        0x0B60 => Some("Vocalic Rr"),
        0x0B61 => Some("Vocalic Ll"),
        0x0B62 => Some("Sign Vocalic L"),
        0x0B63 => Some("Sign Vocalic Ll"),
        0x0B64 => None,
        0x0B65 => None,
        0x0B66 => Some("Digit Zero"),
        0x0B67 => Some("Digit One"),
        0x0B68 => Some("Digit Two"),
        0x0B69 => Some("Digit Three"),
        0x0B6A => Some("Digit Four"),
        0x0B6B => Some("Digit Five"),
        0x0B6C => Some("Digit Six"),
        0x0B6D => Some("Digit Seven"),
        0x0B6E => Some("Digit Eight"),
        0x0B6F => Some("Digit Nine"),
        0x0B70 => Some("Isshar"),
        0x0B71 => Some("Wa"),
        0x0B72 => Some("Fraction 1/4"),
        0x0B73 => Some("Fraction 1/2"),
        0x0B74 => Some("Fraction 3/4"),
        0x0B75 => Some("Fraction 1/16"),
        0x0B76 => Some("Fraction 1/8"),
        0x0B77 => Some("Fraction 3/16"),
        0x0B78 => None,
        0x0B79 => None,
        0x0B7A => None,
        0x0B7B => None,
        0x0B7C => None,
        0x0B7D => None,
        0x0B7E => None,
        0x0B7F => None,

        // Tamil character table
        0x0B80 => None,
        0x0B81 => None,
        0x0B82 => Some("Anusvara"),
        0x0B83 => Some("Visarga"),
        0x0B84 => None,
        0x0B85 => Some("A"),
        0x0B86 => Some("Aa"),
        0x0B87 => Some("I"),
        0x0B88 => Some("Ii"),
        0x0B89 => Some("U"),
        0x0B8A => Some("Uu"),
        0x0B8B => None,
        0x0B8C => None,
        0x0B8D => None,
        0x0B8E => Some("E"),
        0x0B8F => Some("Ee"),
        0x0B90 => Some("Ai"),
        0x0B91 => None,
        0x0B92 => Some("O"),
        0x0B93 => Some("Oo"),
        0x0B94 => Some("Au"),
        0x0B95 => Some("Ka"),
        0x0B96 => None,
        0x0B97 => None,
        0x0B98 => None,
        0x0B99 => Some("Nga"),
        0x0B9A => Some("Ca"),
        0x0B9B => None,
        0x0B9C => Some("Ja"),
        0x0B9D => None,
        0x0B9E => Some("Nya"),
        0x0B9F => Some("Tta"),
        0x0BA0 => None,
        0x0BA1 => None,
        0x0BA2 => None,
        0x0BA3 => Some("Nna"),
        0x0BA4 => Some("Ta"),
        0x0BA5 => None,
        0x0BA6 => None,
        0x0BA7 => None,
        0x0BA8 => Some("Na"),
        0x0BA9 => Some("Nnna"),
        0x0BAA => Some("Pa"),
        0x0BAB => None,
        0x0BAC => None,
        0x0BAD => None,
        0x0BAE => Some("Ma"),
        0x0BAF => Some("Ya"),
        0x0BB0 => Some("Ra"),
        0x0BB1 => Some("Rra"),
        0x0BB2 => Some("La"),
        0x0BB3 => Some("Lla"),
        0x0BB4 => Some("Llla"),
        0x0BB5 => Some("Va"),
        0x0BB6 => Some("Sha"),
        0x0BB7 => Some("Ssa"),
        0x0BB8 => Some("Sa"),
        0x0BB9 => Some("Ha"),
        0x0BBA => None,
        0x0BBB => None,
        0x0BBC => None,
        0x0BBD => None,
        0x0BBE => Some("Sign Aa"),
        0x0BBF => Some("Sign I"),
        0x0BC0 => Some("Sign Ii"),
        0x0BC1 => Some("Sign U"),
        0x0BC2 => Some("Sign Uu"),
        0x0BC3 => None,
        0x0BC4 => None,
        0x0BC5 => None,
        0x0BC6 => Some("Sign E"),
        0x0BC7 => Some("Sign Ee"),
        0x0BC8 => Some("Sign Ai"),
        0x0BC9 => None,
        0x0BCA => Some("Sign O"),
        0x0BCB => Some("Sign Oo"),
        0x0BCC => Some("Sign Au"),
        0x0BCD => Some("Virama"),
        0x0BCE => None,
        0x0BCF => None,
        0x0BD0 => Some("Om"),
        0x0BD1 => None,
        0x0BD2 => None,
        0x0BD3 => None,
        0x0BD4 => None,
        0x0BD5 => None,
        0x0BD6 => None,
        0x0BD7 => Some("Au Length Mark"),
        0x0BD8 => None,
        0x0BD9 => None,
        0x0BDA => None,
        0x0BDB => None,
        0x0BDC => None,
        0x0BDD => None,
        0x0BDE => None,
        0x0BDF => None,
        0x0BE0 => None,
        0x0BE1 => None,
        0x0BE2 => None,
        0x0BE3 => None,
        0x0BE4 => None,
        0x0BE5 => None,
        0x0BE6 => Some("Digit Zero"),
        0x0BE7 => Some("Digit One"),
        0x0BE8 => Some("Digit Two"),
        0x0BE9 => Some("Digit Three"),
        0x0BEA => Some("Digit Four"),
        0x0BEB => Some("Digit Five"),
        0x0BEC => Some("Digit Six"),
        0x0BED => Some("Digit Seven"),
        0x0BEE => Some("Digit Eight"),
        0x0BEF => Some("Digit Nine"),
        0x0BF0 => Some("Number Ten"),
        0x0BF1 => Some("Number One Hundred"),
        0x0BF2 => Some("Number One Thousand"),
        0x0BF3 => Some("Day Sign"),
        0x0BF4 => Some("Month Sign"),
        0x0BF5 => Some("Year Sign"),
        0x0BF6 => Some("Debit Sign"),
        0x0BF7 => Some("Credit Sign"),
        0x0BF8 => Some("As Above Sign"),
        0x0BF9 => Some("Tamil Rupee Sign"),
        0x0BFA => Some("Number Sign"),

        // Telugu character table
        0x0C00 => Some("Combining Candrabindu Above"),
        0x0C01 => Some("Candrabindu"),
        0x0C02 => Some("Anusvara"),
        0x0C03 => Some("Visarga"),
        0x0C04 => Some("Combining Anusvara Above"),
        0x0C05 => Some("A"),
        0x0C06 => Some("Aa"),
        0x0C07 => Some("I"),
        0x0C08 => Some("Ii"),
        0x0C09 => Some("U"),
        0x0C0A => Some("Uu"),
        0x0C0B => Some("Vocalic R"),
        0x0C0C => Some("Vocalic L"),
        0x0C0D => None,
        0x0C0E => Some("E"),
        0x0C0F => Some("Ee"),
        0x0C10 => Some("Ai"),
        0x0C11 => None,
        0x0C12 => Some("O"),
        0x0C13 => Some("Oo"),
        0x0C14 => Some("Au"),
        0x0C15 => Some("Ka"),
        0x0C16 => Some("Kha"),
        0x0C17 => Some("Ga"),
        0x0C18 => Some("Gha"),
        0x0C19 => Some("Nga"),
        0x0C1A => Some("Ca"),
        0x0C1B => Some("Cha"),
        0x0C1C => Some("Ja"),
        0x0C1D => Some("Jha"),
        0x0C1E => Some("Nya"),
        0x0C1F => Some("Tta"),
        0x0C20 => Some("Ttha"),
        0x0C21 => Some("Dda"),
        0x0C22 => Some("Ddha"),
        0x0C23 => Some("Nna"),
        0x0C24 => Some("Ta"),
        0x0C25 => Some("Tha"),
        0x0C26 => Some("Da"),
        0x0C27 => Some("Dha"),
        0x0C28 => Some("Na"),
        0x0C29 => None,
        0x0C2A => Some("Pa"),
        0x0C2B => Some("Pha"),
        0x0C2C => Some("Ba"),
        0x0C2D => Some("Bha"),
        0x0C2E => Some("Ma"),
        0x0C2F => Some("Ya"),
        0x0C30 => Some("Ra"),
        0x0C31 => Some("Rra"),
        0x0C32 => Some("La"),
        0x0C33 => Some("Lla"),
        0x0C34 => Some("Llla"),
        0x0C35 => Some("Va"),
        0x0C36 => Some("Sha"),
        0x0C37 => Some("Ssa"),
        0x0C38 => Some("Sa"),
        0x0C39 => Some("Ha"),
        0x0C3A => None,
        0x0C3B => None,
        0x0C3C => None,
        0x0C3D => Some("Avagraha"),
        0x0C3E => Some("Sign Aa"),
        0x0C3F => Some("Sign I"),
        0x0C40 => Some("Sign Ii"),
        0x0C41 => Some("Sign U"),
        0x0C42 => Some("Sign Uu"),
        0x0C43 => Some("Sign Vocalic R"),
        0x0C44 => Some("Sign Vocalic Rr"),
        0x0C45 => None,
        0x0C46 => Some("Sign E"),
        0x0C47 => Some("Sign Ee"),
        0x0C48 => Some("Sign Ai"),
        0x0C49 => None,
        0x0C4A => Some("Sign O"),
        0x0C4B => Some("Sign Oo"),
        0x0C4C => Some("Sign Au"),
        0x0C4D => Some("Virama"),
        0x0C4E => None,
        0x0C4F => None,
        0x0C50 => None,
        0x0C51 => None,
        0x0C52 => None,
        0x0C53 => None,
        0x0C54 => None,
        0x0C55 => Some("Length Mark"),
        0x0C56 => Some("Ai Length Mark"),
        0x0C57 => None,
        0x0C58 => Some("Tsa"),
        0x0C59 => Some("Dza"),
        0x0C5A => Some("Rrra"),
        0x0C5B => None,
        0x0C5C => None,
        0x0C5D => None,
        0x0C5E => None,
        0x0C5F => None,
        0x0C60 => Some("Vocalic Rr"),
        0x0C61 => Some("Vocalic Ll"),
        0x0C62 => Some("Sign Vocalic L"),
        0x0C63 => Some("Sign Vocalic Ll"),
        0x0C64 => None,
        0x0C65 => None,
        0x0C66 => Some("Digit Zero"),
        0x0C67 => Some("Digit One"),
        0x0C68 => Some("Digit Two"),
        0x0C69 => Some("Digit Three"),
        0x0C6A => Some("Digit Four"),
        0x0C6B => Some("Digit Five"),
        0x0C6C => Some("Digit Six"),
        0x0C6D => Some("Digit Seven"),
        0x0C6E => Some("Digit Eight"),
        0x0C6F => Some("Digit Nine"),
        0x0C70 => None,
        0x0C71 => None,
        0x0C72 => None,
        0x0C73 => None,
        0x0C74 => None,
        0x0C75 => None,
        0x0C76 => None,
        0x0C77 => None,
        0x0C78 => Some("Fraction Zero Odd P"),
        0x0C79 => Some("Fraction One Odd P"),
        0x0C7A => Some("Fraction Two Odd P"),
        0x0C7B => Some("Fraction Three Odd P"),
        0x0C7C => Some("Fraction One Even P"),
        0x0C7D => Some("Fraction Two Even P"),
        0x0C7E => Some("Fraction Three Even P"),
        0x0C7F => Some("Tuumu"),

        // Kannada character table
        0x0C80 => Some("Spacing Candrabindu"),
        0x0C81 => Some("Candrabindu"),
        0x0C82 => Some("Anusvara"),
        0x0C83 => Some("Visarga"),
        0x0C84 => Some("Siddham"),
        0x0C85 => Some("A"),
        0x0C86 => Some("Aa"),
        0x0C87 => Some("I"),
        0x0C88 => Some("Ii"),
        0x0C89 => Some("U"),
        0x0C8A => Some("Uu"),
        0x0C8B => Some("Vocalic R"),
        0x0C8C => Some("Vocalic L"),
        0x0C8D => None,
        0x0C8E => Some("E"),
        0x0C8F => Some("Ee"),
        0x0C90 => Some("Ai"),
        0x0C91 => None,
        0x0C92 => Some("O"),
        0x0C93 => Some("Oo"),
        0x0C94 => Some("Au"),
        0x0C95 => Some("Ka"),
        0x0C96 => Some("Kha"),
        0x0C97 => Some("Ga"),
        0x0C98 => Some("Gha"),
        0x0C99 => Some("Nga"),
        0x0C9A => Some("Ca"),
        0x0C9B => Some("Cha"),
        0x0C9C => Some("Ja"),
        0x0C9D => Some("Jha"),
        0x0C9E => Some("Nya"),
        0x0C9F => Some("Tta"),
        0x0CA0 => Some("Ttha"),
        0x0CA1 => Some("Dda"),
        0x0CA2 => Some("Ddha"),
        0x0CA3 => Some("Nna"),
        0x0CA4 => Some("Ta"),
        0x0CA5 => Some("Tha"),
        0x0CA6 => Some("Da"),
        0x0CA7 => Some("Dha"),
        0x0CA8 => Some("Na"),
        0x0CA9 => None,
        0x0CAA => Some("Pa"),
        0x0CAB => Some("Pha"),
        0x0CAC => Some("Ba"),
        0x0CAD => Some("Bha"),
        0x0CAE => Some("Ma"),
        0x0CAF => Some("Ya"),
        0x0CB0 => Some("Ra"),
        0x0CB1 => Some("Rra"),
        0x0CB2 => Some("La"),
        0x0CB3 => Some("Lla"),
        0x0CB4 => None,
        0x0CB5 => Some("Va"),
        0x0CB6 => Some("Sha"),
        0x0CB7 => Some("Ssa"),
        0x0CB8 => Some("Sa"),
        0x0CB9 => Some("Ha"),
        0x0CBA => None,
        0x0CBB => None,
        0x0CBC => Some("Nukta"),
        0x0CBD => Some("Avagraha"),
        0x0CBE => Some("Sign Aa"),
        0x0CBF => Some("Sign I"),
        0x0CC0 => Some("Sign Ii"),
        0x0CC1 => Some("Sign U"),
        0x0CC2 => Some("Sign Uu"),
        0x0CC3 => Some("Sign Vocalic R"),
        0x0CC4 => Some("Sign Vocalic Rr"),
        0x0CC5 => None,
        0x0CC6 => Some("Sign E"),
        0x0CC7 => Some("Sign Ee"),
        0x0CC8 => Some("Sign Ai"),
        0x0CC9 => None,
        0x0CCA => Some("Sign O"),
        0x0CCB => Some("Sign Oo"),
        0x0CCC => Some("Sign Au"),
        0x0CCD => Some("Virama"),
        0x0CCE => None,
        0x0CCF => None,
        0x0CD0 => None,
        0x0CD1 => None,
        0x0CD2 => None,
        0x0CD3 => None,
        0x0CD4 => None,
        0x0CD5 => Some("Length Mark"),
        0x0CD6 => Some("Ai Length Mark"),
        0x0CD7 => None,
        0x0CD8 => None,
        0x0CD9 => None,
        0x0CDA => None,
        0x0CDB => None,
        0x0CDC => None,
        0x0CDD => None,
        0x0CDE => Some("Fa"),
        0x0CDF => None,
        0x0CE0 => Some("Vocalic Rr"),
        0x0CE1 => Some("Vocalic Ll"),
        0x0CE2 => Some("Sign Vocalic L"),
        0x0CE3 => Some("Sign Vocalic Ll"),
        0x0CE4 => None,
        0x0CE5 => None,
        0x0CE6 => Some("Digit Zero"),
        0x0CE7 => Some("Digit One"),
        0x0CE8 => Some("Digit Two"),
        0x0CE9 => Some("Digit Three"),
        0x0CEA => Some("Digit Four"),
        0x0CEB => Some("Digit Five"),
        0x0CEC => Some("Digit Six"),
        0x0CED => Some("Digit Seven"),
        0x0CEE => Some("Digit Eight"),
        0x0CEF => Some("Digit Nine"),
        0x0CF0 => None,
        0x0CF1 => Some("Jihvamuliya"),
        0x0CF2 => Some("Upadhmaniya"),

        // Malayalam character table
        0x0D00 => Some("Combining Anusvara Above"),
        0x0D01 => Some("Candrabindu"),
        0x0D02 => Some("Anusvara"),
        0x0D03 => Some("Visarga"),
        0x0D04 => None,
        0x0D05 => Some("A"),
        0x0D06 => Some("Aa"),
        0x0D07 => Some("I"),
        0x0D08 => Some("Ii"),
        0x0D09 => Some("U"),
        0x0D0A => Some("Uu"),
        0x0D0B => Some("Vocalic R"),
        0x0D0C => Some("Vocalic L"),
        0x0D0D => None,
        0x0D0E => Some("E"),
        0x0D0F => Some("Ee"),
        0x0D10 => Some("Ai"),
        0x0D11 => None,
        0x0D12 => Some("O"),
        0x0D13 => Some("Oo"),
        0x0D14 => Some("Au"),
        0x0D15 => Some("Ka"),
        0x0D16 => Some("Kha"),
        0x0D17 => Some("Ga"),
        0x0D18 => Some("Gha"),
        0x0D19 => Some("Nga"),
        0x0D1A => Some("Ca"),
        0x0D1B => Some("Cha"),
        0x0D1C => Some("Ja"),
        0x0D1D => Some("Jha"),
        0x0D1E => Some("Nya"),
        0x0D1F => Some("Tta"),
        0x0D20 => Some("Ttha"),
        0x0D21 => Some("Dda"),
        0x0D22 => Some("Ddha"),
        0x0D23 => Some("Nna"),
        0x0D24 => Some("Ta"),
        0x0D25 => Some("Tha"),
        0x0D26 => Some("Da"),
        0x0D27 => Some("Dha"),
        0x0D28 => Some("Na"),
        0x0D29 => Some("Nnna"),
        0x0D2A => Some("Pa"),
        0x0D2B => Some("Pha"),
        0x0D2C => Some("Ba"),
        0x0D2D => Some("Bha"),
        0x0D2E => Some("Ma"),
        0x0D2F => Some("Ya"),
        0x0D30 => Some("Ra"),
        0x0D31 => Some("Rra"),
        0x0D32 => Some("La"),
        0x0D33 => Some("Lla"),
        0x0D34 => Some("Llla"),
        0x0D35 => Some("Va"),
        0x0D36 => Some("Sha"),
        0x0D37 => Some("Ssa"),
        0x0D38 => Some("Sa"),
        0x0D39 => Some("Ha"),
        0x0D3A => Some("Ttta"),
        0x0D3B => Some("Vertical Bar Virama"),
        0x0D3C => Some("Circular Virama"),
        0x0D3D => Some("Avagraha"),
        0x0D3E => Some("Sign Aa"),
        0x0D3F => Some("Sign I"),
        0x0D40 => Some("Sign Ii"),
        0x0D41 => Some("Sign U"),
        0x0D42 => Some("Sign Uu"),
        0x0D43 => Some("Sign Vocalic R"),
        0x0D44 => Some("Sign Vocalic Rr"),
        0x0D45 => None,
        0x0D46 => Some("Sign E"),
        0x0D47 => Some("Sign Ee"),
        0x0D48 => Some("Sign Ai"),
        0x0D49 => None,
        0x0D4A => Some("Sign O"),
        0x0D4B => Some("Sign Oo"),
        0x0D4C => Some("Sign Au"),
        0x0D4D => Some("Virama"),
        0x0D4E => Some("Dot Reph"),
        0x0D4F => Some("Para"),
        0x0D50 => None,
        0x0D51 => None,
        0x0D52 => None,
        0x0D53 => None,
        0x0D54 => Some("Chillu M"),
        0x0D55 => Some("Chillu Y"),
        0x0D56 => Some("Chillu Lll"),
        0x0D57 => Some("Au Length Mark"),
        0x0D58 => Some("Fraction 1/160"),
        0x0D59 => Some("Fraction 1/40"),
        0x0D5A => Some("Fraction 3/80"),
        0x0D5B => Some("Fraction 1/20"),
        0x0D5C => Some("Fraction 1/10"),
        0x0D5D => Some("Fraction 3/20"),
        0x0D5E => Some("Fraction 1/5"),
        0x0D5F => Some("Archaic Ii"),
        0x0D60 => Some("Vocalic Rr"),
        0x0D61 => Some("Vocalic Ll"),
        0x0D62 => Some("Sign Vocalic L"),
        0x0D63 => Some("Sign Vocalic Ll"),
        0x0D64 => None,
        0x0D65 => None,
        0x0D66 => Some("Digit Zero"),
        0x0D67 => Some("Digit One"),
        0x0D68 => Some("Digit Two"),
        0x0D69 => Some("Digit Three"),
        0x0D6A => Some("Digit Four"),
        0x0D6B => Some("Digit Five"),
        0x0D6C => Some("Digit Six"),
        0x0D6D => Some("Digit Seven"),
        0x0D6E => Some("Digit Eight"),
        0x0D6F => Some("Digit Nine"),
        0x0D70 => Some("Number Ten"),
        0x0D71 => Some("Number One Hundred"),
        0x0D72 => Some("Number One Thousand"),
        0x0D73 => Some("Fraction 1/4"),
        0x0D74 => Some("Fraction 1/2"),
        0x0D75 => Some("Fraction 3/4"),
        0x0D76 => Some("Fraction 1/16"),
        0x0D77 => Some("Fraction 1/8"),
        0x0D78 => Some("Fraction 3/16"),
        0x0D79 => Some("Date Mark"),
        0x0D7A => Some("Chillu Nn"),
        0x0D7B => Some("Chillu N"),
        0x0D7C => Some("Chillu Rr"),
        0x0D7D => Some("Chillu L"),
        0x0D7E => Some("Chillu Ll"),
        0x0D7F => Some("Chillu K"),

        // Sinhala character table
        0x0D80 => None,
        0x0D81 => None,
        0x0D82 => Some("Anusvara"),
        0x0D83 => Some("Visarga"),
        0x0D84 => None,
        0x0D85 => Some("A"),
        0x0D86 => Some("Aa"),
        0x0D87 => Some("Ae"),
        0x0D88 => Some("Aae"),
        0x0D89 => Some("I"),
        0x0D8A => Some("Ii"),
        0x0D8B => Some("U"),
        0x0D8C => Some("Uu"),
        0x0D8D => Some("Vocalic R"),
        0x0D8E => Some("Vocalic Rr"),
        0x0D8F => Some("Vocalic L"),
        0x0D90 => Some("Vocalic Ll"),
        0x0D91 => Some("E"),
        0x0D92 => Some("Ee"),
        0x0D93 => Some("Ai"),
        0x0D94 => Some("O"),
        0x0D95 => Some("Oo"),
        0x0D96 => Some("Au"),
        0x0D97 => None,
        0x0D98 => None,
        0x0D99 => None,
        0x0D9A => Some("Ka"),
        0x0D9B => Some("Kha"),
        0x0D9C => Some("Ga"),
        0x0D9D => Some("Gha"),
        0x0D9E => Some("Nga"),
        0x0D9F => Some("Nnga"),
        0x0DA0 => Some("Ca"),
        0x0DA1 => Some("Cha"),
        0x0DA2 => Some("Ja"),
        0x0DA3 => Some("Jha"),
        0x0DA4 => Some("Nya"),
        0x0DA5 => Some("Jnya"),
        0x0DA6 => Some("Nyja"),
        0x0DA7 => Some("Tta"),
        0x0DA8 => Some("Ttha"),
        0x0DA9 => Some("Dda"),
        0x0DAA => Some("Ddha"),
        0x0DAB => Some("Nna"),
        0x0DAC => Some("Nndda"),
        0x0DAD => Some("Ta"),
        0x0DAE => Some("Tha"),
        0x0DAF => Some("Da"),
        0x0DB0 => Some("Dha"),
        0x0DB1 => Some("Na"),
        0x0DB2 => None,
        0x0DB3 => Some("Nda"),
        0x0DB4 => Some("Pa"),
        0x0DB5 => Some("Pha"),
        0x0DB6 => Some("Ba"),
        0x0DB7 => Some("Bha"),
        0x0DB8 => Some("Ma"),
        0x0DB9 => Some("Mba"),
        0x0DBA => Some("Ya"),
        0x0DBB => Some("Ra"),
        0x0DBC => None,
        0x0DBD => Some("La"),
        0x0DBE => None,
        0x0DBF => None,
        0x0DC0 => Some("Va"),
        0x0DC1 => Some("Sha"),
        0x0DC2 => Some("Ssa"),
        0x0DC3 => Some("Sa"),
        0x0DC4 => Some("Ha"),
        0x0DC5 => Some("Lla"),
        0x0DC6 => Some("Fa"),
        0x0DC7 => None,
        0x0DC8 => None,
        0x0DC9 => None,
        0x0DCA => Some("Virama"),
        0x0DCB => None,
        0x0DCC => None,
        0x0DCD => None,
        0x0DCE => None,
        0x0DCF => Some("Sign Aa"),
        0x0DD0 => Some("Sign Ae"),
        0x0DD1 => Some("Sign Aae"),
        0x0DD2 => Some("Sign I"),
        0x0DD3 => Some("Sign Ii"),
        0x0DD4 => Some("Sign U"),
        0x0DD5 => None,
        0x0DD6 => Some("Sign Uu"),
        0x0DD7 => None,
        0x0DD8 => Some("Sign Vocalic R"),
        0x0DD9 => Some("Sign E"),
        0x0DDA => Some("Sign Ee"),
        0x0DDB => Some("Sign Ai"),
        0x0DDC => Some("Sign O"),
        0x0DDD => Some("Sign Oo"),
        0x0DDE => Some("Sign Au"),
        0x0DDF => Some("Sign Vocalic L"),
        0x0DE0 => None,
        0x0DE1 => None,
        0x0DE2 => None,
        0x0DE3 => None,
        0x0DE4 => None,
        0x0DE5 => None,
        0x0DE6 => Some("Digit Zero"),
        0x0DE7 => Some("Digit One"),
        0x0DE8 => Some("Digit Two"),
        0x0DE9 => Some("Digit Three"),
        0x0DEA => Some("Digit Four"),
        0x0DEB => Some("Digit Five"),
        0x0DEC => Some("Digit Six"),
        0x0DED => Some("Digit Seven"),
        0x0DEE => Some("Digit Eight"),
        0x0DEF => Some("Digit Nine"),
        0x0DF0 => None,
        0x0DF1 => None,
        0x0DF2 => Some("Sign Vocalic Rr"),
        0x0DF3 => Some("Sign Vocalic Ll"),
        0x0DF4 => Some("Kunddaliya"),
        0x0DF5 => None,
        0x0DF6 => None,
        0x0DF7 => None,
        0x0DF8 => None,
        0x0DF9 => None,
        0x0DFA => None,
        0x0DFB => None,
        0x0DFC => None,
        0x0DFD => None,
        0x0DFE => None,
        0x0DFF => None,

        // Vedic Extensions character table
        0x1CD0 => Some("Tone Karshana"),
        0x1CD1 => Some("Tone Shara"),
        0x1CD2 => Some("Tone Prenkha"),
        0x1CD3 => Some("Sign Nihshvasa"),
        0x1CD4 => Some("Tone Midline Svarita"),
        0x1CD5 => Some("Tone Aggravated Independent Svarita"),
        0x1CD6 => Some("Tone Independent Svarita"),
        0x1CD7 => Some("Tone Kathaka Independent Svarita"),
        0x1CD8 => Some("Tone Candra Below"),
        0x1CD9 => Some("Tone Kathaka Independent Svarita Schroeder"),
        0x1CDA => Some("Tone Double Svarita"),
        0x1CDB => Some("Tone Triple Svarita"),
        0x1CDC => Some("Tone Kathaka Anudatta"),
        0x1CDD => Some("Tone Dot Below"),
        0x1CDE => Some("Tone Two Dots Below"),
        0x1CDF => Some("Tone Three Dots Below"),
        0x1CE0 => Some("Tone Rigvedic Kashmiri Independent Svarita"),
        0x1CE1 => Some("Tone Atharavedic Independent Svarita"),
        0x1CE2 => Some("Sign Visarga Svarita"),
        0x1CE3 => Some("Sign Visarga Udatta"),
        0x1CE4 => Some("Sign Reversed Visarga Udatta"),
        0x1CE5 => Some("Sign Visarga Anudatta"),
        0x1CE6 => Some("Sign Reversed Visarga Anudatta"),
        0x1CE7 => Some("Sign Visarga Udatta With Tail"),
        0x1CE8 => Some("Sign Visarga Anudatta With Tail"),
        0x1CE9 => Some("Sign Anusvara Antargomukha"),
        0x1CEA => Some("Sign Anusvara Bahirgomukha"),
        0x1CEB => Some("Sign Anusvara Vamagomukha"),
        0x1CEC => Some("Sign Anusvara Vamagomukha With Tail"),
        0x1CED => Some("Sign Tiryak"),
        0x1CEE => Some("Sign Hexiform Long Anusvara"),
        0x1CEF => Some("Sign Long Anusvara"),
        0x1CF0 => Some("Sign Rthang Long Anusvara"),
        0x1CF1 => Some("Sign Anusvara Ubhayato Mukha"),
        0x1CF2 => Some("Sign Ardhavisarga"),
        0x1CF3 => Some("Sign Rotated Ardhavisarga"),
        0x1CF4 => Some("Tone Candra Above"),
        0x1CF5 => Some("Sign Jihvamuliya"),
        0x1CF6 => Some("Sign Upadhmaniya"),
        0x1CF7 => Some("Sign Atikrama"),
        0x1CF8 => Some("Tone Ring Above"),
        0x1CF9 => Some("Tone Double Ring Above"),

        // Devanagari Extended character table
        0xA8E0 => Some("Combining Zero"),
        0xA8E1 => Some("Combining One"),
        0xA8E2 => Some("Combining Two"),
        0xA8E3 => Some("Combining Three"),
        0xA8E4 => Some("Combining Four"),
        0xA8E5 => Some("Combining Five"),
        0xA8E6 => Some("Combining Six"),
        0xA8E7 => Some("Combining Seven"),
        0xA8E8 => Some("Combining Eight"),
        0xA8E9 => Some("Combining Nine"),
        0xA8EA => Some("Combining A"),
        0xA8EB => Some("Combining U"),
        0xA8EC => Some("Combining Ka"),
        0xA8ED => Some("Combining Na"),
        0xA8EE => Some("Combining Pa"),
        0xA8EF => Some("Combining Ra"),
        0xA8F0 => Some("Combining Vi"),
        0xA8F1 => Some("Combining Avagraha"),
        0xA8F2 => Some("Spacing Candrabindu"),
        0xA8F3 => Some("Candrabindu Virama"),
        0xA8F4 => Some("Double Candrabindu Virama"),
        0xA8F5 => Some("Candrabindu Two"),
        0xA8F6 => Some("Candrabindu Three"),
        0xA8F7 => Some("Candrabindu Avagraha"),
        0xA8F8 => Some("Pushpika"),
        0xA8F9 => Some("Gap Filler"),
        0xA8FA => Some("Caret"),
        0xA8FB => Some("Headstroke"),
        0xA8FC => Some("Siddham"),
        0xA8FD => Some("Jain Om"),
        0xA8FE => Some("Ay"),
        0xA8FF => Some("Sign Ay"),

        // Sinhala Archaic Numbers character table
        0x111E0 => None,
        0x111E1 => Some("Archaic Digit One"),
        0x111E2 => Some("Archaic Digit Two"),
        0x111E3 => Some("Archaic Digit Three"),
        0x111E4 => Some("Archaic Digit Four"),
        0x111E5 => Some("Archaic Digit Five"),
        0x111E6 => Some("Archaic Digit Six"),
        0x111E7 => Some("Archaic Digit Seven"),
        0x111E8 => Some("Archaic Digit Eight"),
        0x111E9 => Some("Archaic Digit Nine"),
        0x111EA => Some("Archaic Number Ten"),
        0x111EB => Some("Archaic Number 20"),
        0x111EC => Some("Archaic Number 30"),
        0x111ED => Some("Archaic Number 40"),
        0x111EE => Some("Archaic Number 50"),
        0x111EF => Some("Archaic Number 60"),
        0x111F0 => Some("Archaic Number 70"),
        0x111F1 => Some("Archaic Number 80"),
        0x111F2 => Some("Archaic Number 90"),
        0x111F3 => Some("Archaic Number 100"),
        0x111F4 => Some("Archaic Number 1000"),
        0x111F5 => None,
        0x111F6 => None,
        0x111F7 => None,
        0x111F8 => None,
        0x111F9 => None,
        0x111FA => None,
        0x111FB => None,
        0x111FC => None,
        0x111FD => None,
        0x111FE => None,
        0x111FF => None,

        // Grantha marks character table
        0x11301 => Some("Grantha Candrabindu"),
        0x11303 => Some("Grantha Visarga"),
        0x1133C => Some("Grantha Nukta"),

        // Miscellaneous character table
        0x00A0 => Some("No-break space"),
        0x00B2 => Some("Superscript Two (used in Tamil)"),
        0x00B3 => Some("Superscript Three (used in Tamil)"),
        0x200C => Some("Zero-width non-joiner"),
        0x200D => Some("Zero-width joiner"),
        0x2010 => Some("Hyphen"),
        0x2011 => Some("No-break hyphen"),
        0x2012 => Some("Figure dash"),
        0x2013 => Some("En dash"),
        0x2014 => Some("Em dash"),
        0x2074 => Some("Superscript Four (used in Tamil)"),
        0x2082 => Some("Subscript Two (used in Tamil)"),
        0x2083 => Some("Subscript Three (used in Tamil)"),
        0x2084 => Some("Subscript Four (used in Tamil)"),
        0x25CC => Some("Dotted circle"),

        _ => None,
    }
}
