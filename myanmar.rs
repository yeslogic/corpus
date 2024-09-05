use super::Syllable;

// "A practical maximum cluster length is 31 characters."
// https://learn.microsoft.com/en-us/typography/script-development/use#cluster-length
const MAX_CLUSTER_LEN: usize = 31;

// A fairly arbitrary limit for match_repeat_upto since we don't have easy access to
//  the in-flight cluster length at the moment.
const MAX_REPEAT: usize = MAX_CLUSTER_LEN / 3;

#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum ShapingClass {
    Bindu,
    Visarga,
    PureKiller,
    Consonant,
    VowelIndependent,
    VowelDependent,
    ConsonantMedial,
    ConsonantPlaceholder,
    Number,
    Symbol,
    ToneMarker,
    InvisibleStacker,
    ConsonantWithStacker,
    Placeholder,
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
    TopLeftAndBottomPosition,
}

fn shaping_class(ch: char) -> Option<ShapingClass> {
    let (shaping, _) = myanmar_character(ch);
    shaping
}

// C
//
// The definition of _consonant_ in the shaping docs excludes _ra_ but the only place it's
// used, 'C', adds _ra_ back in, so we skip that.
fn consonant(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::Consonant | ShapingClass::ConsonantPlaceholder) => true,
        _ => false,
    }
}

// _vowel_
fn vowel(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::VowelIndependent) => true,
        _ => false,
    }
}

// _d_
fn digit(ch: char) -> bool {
    shaping_class(ch) == Some(ShapingClass::Number)
}

// _gb_
fn generic_base(ch: char) -> bool {
    matches!(
        ch,
        '\u{002D}'
            | '\u{00A0}'
            | '\u{00D7}'
            | '\u{2012}'
            | '\u{2013}'
            | '\u{2014}'
            | '\u{2015}'
            | '\u{2022}'
            | '\u{25CC}'
            | '\u{25FB}'
            | '\u{25FC}'
            | '\u{25FD}'
            | '\u{25FE}'
    )
}

// Simple non-compounding cluster
//
// <P | S | R | WJ| WS | O | D0 >
//
// Punctuation (P), symbols (S), reserved characters from the Myanmar block (R), word joiner (WJ), white space (WS), and other SCRIPT_COMMON charcters (O) contain one character per cluster.
fn standalone(ch: char) -> bool {
    let class = shaping_class(ch);
    matches!(ch,
        '\u{1000}'..='\u{109f}' | '\u{AA60}' ..= '\u{AA7F}' | '\u{A9E0}' ..= '\u{A9FF}'
    ) && (class.is_none() || class == Some(ShapingClass::Placeholder))
}

fn variation_selector(ch: char) -> bool {
    ch == '\u{FE00}'
}

fn halant(ch: char) -> bool {
    shaping_class(ch) == Some(ShapingClass::InvisibleStacker)
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

fn joiner(ch: char) -> bool {
    zwj(ch) || zwnj(ch)
}

fn ra(ch: char) -> bool {
    match ch {
        '\u{101B}' => true, // Ra
        '\u{1004}' => true, // Nga
        '\u{105A}' => true, // Mon Nga
        _ => false,
    }
}

fn asat(ch: char) -> bool {
    ch == '\u{103A}' // Asat
}

fn consonant_with_stacker(ch: char) -> bool {
    match shaping_class(ch) {
        Some(ShapingClass::ConsonantWithStacker) => true,
        _ => false,
    }
}

fn matra_pre(ch: char) -> bool {
    matches!(
        myanmar_character(ch),
        (
            Some(ShapingClass::VowelDependent),
            Some(MarkPlacementSubclass::LeftPosition)
        )
    )
}

fn matra_post(ch: char) -> bool {
    matches!(
        myanmar_character(ch),
        (
            Some(ShapingClass::VowelDependent),
            Some(MarkPlacementSubclass::RightPosition)
        )
    )
}

// "Anusvara" | "Sign Ai"
fn a(ch: char) -> bool {
    // Note: "Sign Ai" is classified as a, not as matraabove, in order to implement orthographically correct behavior.
    ch == '\u{1036}' || ch == '\u{1032}'
}

fn dot_below(ch: char) -> bool {
    ch == '\u{1037}'
}

fn matra_above(ch: char) -> bool {
    !a(ch)
        && matches!(
            myanmar_character(ch),
            (
                Some(ShapingClass::VowelDependent),
                Some(MarkPlacementSubclass::TopPosition)
            )
        )
}

fn matra_below(ch: char) -> bool {
    matches!(
        myanmar_character(ch),
        (
            Some(ShapingClass::VowelDependent),
            Some(MarkPlacementSubclass::BottomPosition)
        )
    )
}

// "Medial Ha"
fn medial_ha(ch: char) -> bool {
    ch == '\u{103E}'
}

// "Mon Medial La"
fn medial_la(ch: char) -> bool {
    ch == '\u{1060}'
}

// Medial Ra
fn medial_ra(ch: char) -> bool {
    ch == '\u{103C}'
}

// "Medial Wa" | "Shan Medial Wa"
fn medial_wa(ch: char) -> bool {
    ch == '\u{103D}' || ch == '\u{1082}'
}

// "Medial Ya" | "Mon Medial Na" | "Mon Medial Ma"
fn medial_ya(ch: char) -> bool {
    ch == '\u{103B}' || ch == '\u{105E}' || ch == '\u{105F}'
}

// "Tone Sgaw Karen Hathi" | "Tone Sgaw Karen Ke Pho" | "Western Pwo Karen Tone 1" | "Western Pwo Karen Tone 2" | "Western Pwo Karen Tone 3" | "Western Pwo Karen Tone 4" | "Western Pwo Karen Tone 5" | "Pao Karen Tone"
fn pt(ch: char) -> bool {
    match ch {
        // U+1063 	Mark [Mc] 	TONE_MARKER 	RIGHT_POSITION 	ၣ Tone Sgaw Karen Hathi
        // U+1064 	Mark [Mc] 	TONE_MARKER 	RIGHT_POSITION 	ၤ Tone Sgaw Karen Ke Pho
        '\u{1063}' | '\u{1064}' => true,
        // U+1069 	Mark [Mc] 	TONE_MARKER 	RIGHT_POSITION 	ၩ Sign Western Pwo Karen Tone 1
        // U+106A 	Mark [Mc] 	TONE_MARKER 	RIGHT_POSITION 	ၪ Sign Western Pwo Karen Tone 2
        // U+106B 	Mark [Mc] 	TONE_MARKER 	RIGHT_POSITION 	ၫ Sign Western Pwo Karen Tone 3
        // U+106C 	Mark [Mc] 	TONE_MARKER 	RIGHT_POSITION 	ၬ Sign Western Pwo Karen Tone 4
        // U+106D 	Mark [Mc] 	TONE_MARKER 	RIGHT_POSITION 	ၭ Sign Western Pwo Karen Tone 5
        '\u{1069}'..='\u{106D}' => true,
        // U+AA7B	TONE_MARKER	RIGHT_POSITION	ꩻ Sign Pao Karen Tone
        '\u{AA7B}' => true,
        _ => false,
    }
}

// "Little Section" | "Section"
fn punc(ch: char) -> bool {
    ch == '\u{104A}' || ch == '\u{104B}'
}

// _ra_ _asat_ _halant_
fn match_kinzi<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_seq(match_one(ra), match_seq(match_one(asat), match_one(halant)))(cs)
}

fn match_z<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_one(joiner)(cs)
}

// _matrapre_* _matraabove_* _matrabelow_* _a_* (_db_ _asat_?)?
fn match_vmain<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_repeat_upto(
        MAX_REPEAT,
        match_one(matra_pre),
        match_repeat_upto(
            4,
            match_one(matra_above),
            match_repeat_upto(
                4,
                match_one(matra_below),
                match_repeat_upto(
                    4,
                    match_one(a),
                    match_optional(match_seq(
                        match_one(dot_below),
                        match_optional(match_one(asat)),
                    )),
                ),
            ),
        ),
    )(cs)
}

// _matrapost_ _mh_? _asat_* _matraabove_* _a_* (_db_ _asat_?)?
fn match_vpost<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_seq(
        match_one(matra_post),
        match_repeat_upto(
            4,
            match_optional(match_one(medial_ha)),
            match_repeat_upto(
                4,
                match_one(asat),
                match_repeat_upto(
                    4,
                    match_one(matra_above),
                    match_repeat_upto(
                        4,
                        match_one(a),
                        match_optional(match_seq(
                            match_one(dot_below),
                            match_optional(match_one(asat)),
                        )),
                    ),
                ),
            ),
        ),
    )(cs)
}

// _pt_ _a_* _db_? _asat_?
fn match_pwo<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_seq(
        match_one(pt),
        match_repeat_upto(
            MAX_REPEAT,
            match_one(a),
            match_seq(
                match_optional(match_one(dot_below)),
                match_optional(match_one(asat)),
            ),
        ),
    )(cs)
}

fn visarga(ch: char) -> bool {
    shaping_class(ch) == Some(ShapingClass::Visarga)
}

fn sm(ch: char) -> bool {
    match ch {
        // Shan Tone 2, 3, 5, 6, Shan Council Tone 2, 3, Emphatic
        '\u{1087}'..='\u{108D}' => true,
        // Rumai Palaung Tone 5
        '\u{108F}' => true,
        // Khamti Tone 1, 3, Aiton A
        '\u{109A}'..='\u{109C}' => true,
        // Visarga
        _ if visarga(ch) => true,
        _ => false,
    }
}

// Tcomplex= _asat_* Med Vmain Vpost* Pwo* _sm_* Z?
fn match_t_complex<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_repeat_upto(
        MAX_REPEAT,
        match_one(asat),
        match_seq(
            match_medial_group,
            match_seq(
                match_vmain,
                match_repeat_upto(
                    MAX_REPEAT,
                    match_vpost,
                    match_repeat_upto(
                        MAX_REPEAT,
                        match_pwo,
                        match_repeat_upto(MAX_REPEAT, match_one(sm), match_optional(match_z)),
                    ),
                ),
            ),
        ),
    )(cs)
}

// _halant_ | Tcomplex
fn match_syllable_tail<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_either(match_one(halant), match_t_complex)(cs)
}

// (_halant_ (C | _vowel_) _vs_?)
fn match_halant_group<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_seq(
        match_seq(
            match_one(halant),
            match_either(match_one(consonant), match_one(vowel)),
        ),
        match_optional(match_one(variation_selector)),
    )(cs)
}

// Med = _my_? _asat_? _mr_? ( (mw mh? ml? | mh ml? | ml) asat?)?
fn match_medial_group<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_optional_seq(
        match_one(medial_ya),
        match_optional_seq(
            match_one(asat),
            match_optional_seq(match_one(medial_ra), match_optional(match_medial_group2)),
        ),
    )(cs)
}

// (mw mh? ml? | mh ml? | ml) asat?
fn match_medial_group2<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_seq(
        match_either(
            match_medial_group2a,
            match_either(match_medial_group2b, match_one(medial_la)),
        ),
        match_optional(match_one(asat)),
    )(cs)
}

// mw mh? ml?
fn match_medial_group2a<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_seq(
        match_one(medial_wa),
        match_optional_seq(match_one(medial_ha), match_optional(match_one(medial_la))),
    )(cs)
}

// mh ml?
fn match_medial_group2b<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_seq(match_one(medial_ha), match_optional(match_one(medial_la)))(cs)
}

// G = _gb_ | _d_ | _punc_
fn match_g<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_either(
        match_one(generic_base),
        match_either(match_one(digit), match_one(punc)),
    )(cs)
}

// (C | _vowel_ | G)
fn match_initial_group<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_either(
        match_one(consonant),
        match_either(match_one(vowel), match_g),
    )(cs)
}

// (K | _cs_)? (C | _vowel_ | G) _vs_? (_halant_ (C | _vowel_) _vs_?)* Tail
fn match_consonant_syllable<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_optional_seq(
        match_either(match_kinzi, match_one(consonant_with_stacker)),
        match_seq(
            match_initial_group,
            match_optional_seq(
                match_one(variation_selector),
                match_repeat_upto(MAX_REPEAT, match_halant_group, match_syllable_tail),
            ),
        ),
    )(cs)
}

fn match_standalone<T: SyllableChar>(cs: &[T]) -> Option<usize> {
    match_one(standalone)(cs)
}

pub fn match_syllable<T: SyllableChar>(cs: &[T]) -> Option<(usize, Syllable)> {
    match match_consonant_syllable(cs) {
        Some(len) => Some((len, Syllable::Consonant)),
        None => match_standalone(cs).map(|len| (len, Syllable::Broken)),
    }
}

/////////////////////////////////////////////////////////////////////////////
// Myanmar character tables
/////////////////////////////////////////////////////////////////////////////

fn myanmar_character(ch: char) -> (Option<ShapingClass>, Option<MarkPlacementSubclass>) {
    use self::MarkPlacementSubclass::*;
    use self::ShapingClass::*;

    match ch as u32 {
        // Myanmar character table
        0x1000 => (Some(Consonant), None),        // က Ka
        0x1001 => (Some(Consonant), None),        // ခ Kha
        0x1002 => (Some(Consonant), None),        // ဂ Ga
        0x1003 => (Some(Consonant), None),        // ဃ Gha
        0x1004 => (Some(Consonant), None),        // င Nga
        0x1005 => (Some(Consonant), None),        // စ Ca
        0x1006 => (Some(Consonant), None),        // ဆ Cha
        0x1007 => (Some(Consonant), None),        // ဇ Ja
        0x1008 => (Some(Consonant), None),        // ဈ Jha
        0x1009 => (Some(Consonant), None),        // ဉ Nya
        0x100A => (Some(Consonant), None),        // ည Nnya
        0x100B => (Some(Consonant), None),        // ဋ Tta
        0x100C => (Some(Consonant), None),        // ဌ Ttha
        0x100D => (Some(Consonant), None),        // ဍ Dda
        0x100E => (Some(Consonant), None),        // ဎ DDha
        0x100F => (Some(Consonant), None),        // ဏ Nna
        0x1010 => (Some(Consonant), None),        // တ Ta
        0x1011 => (Some(Consonant), None),        // ထ Tha
        0x1012 => (Some(Consonant), None),        // ဒ Da
        0x1013 => (Some(Consonant), None),        // ဓ Dha
        0x1014 => (Some(Consonant), None),        // န Na
        0x1015 => (Some(Consonant), None),        // ပ Pa
        0x1016 => (Some(Consonant), None),        // ဖ Pha
        0x1017 => (Some(Consonant), None),        // ဗ Ba
        0x1018 => (Some(Consonant), None),        // ဘ Bha
        0x1019 => (Some(Consonant), None),        // မ Ma
        0x101A => (Some(Consonant), None),        // ယ Ya
        0x101B => (Some(Consonant), None),        // ရ Ra
        0x101C => (Some(Consonant), None),        // လ La
        0x101D => (Some(Consonant), None),        // ဝ Wa
        0x101E => (Some(Consonant), None),        // သ Sa
        0x101F => (Some(Consonant), None),        // ဟ Ha
        0x1020 => (Some(Consonant), None),        // ဠ Lla
        0x1021 => (Some(VowelIndependent), None), // အ A
        0x1022 => (Some(VowelIndependent), None), // ဢ Shan A
        0x1023 => (Some(VowelIndependent), None), // ဣ I
        0x1024 => (Some(VowelIndependent), None), // ဤ Ii
        0x1025 => (Some(VowelIndependent), None), // ဥ U
        0x1026 => (Some(VowelIndependent), None), // ဦ Uu
        0x1027 => (Some(VowelIndependent), None), // ဧ E
        0x1028 => (Some(VowelIndependent), None), // ဨ Mon E
        0x1029 => (Some(VowelIndependent), None), // ဩ O
        0x102A => (Some(VowelIndependent), None), // ဪ Au
        0x102B => (Some(VowelDependent), Some(RightPosition)), // ါ Sign Tall Aa
        0x102C => (Some(VowelDependent), Some(RightPosition)), // ာ Sign Aa
        0x102D => (Some(VowelDependent), Some(TopPosition)), // ိ Sign I
        0x102E => (Some(VowelDependent), Some(TopPosition)), // ီ Sign Ii
        0x102F => (Some(VowelDependent), Some(BottomPosition)), // ု Sign U
        0x1030 => (Some(VowelDependent), Some(BottomPosition)), // ူ Sign Uu
        0x1031 => (Some(VowelDependent), Some(LeftPosition)), // ေ Sign E
        0x1032 => (Some(VowelDependent), Some(TopPosition)), // ဲ Sign Ai
        0x1033 => (Some(VowelDependent), Some(TopPosition)), // ဳ Sign Mon Ii
        0x1034 => (Some(VowelDependent), Some(TopPosition)), // ဴ Sign Mon O
        0x1035 => (Some(VowelDependent), Some(TopPosition)), // ဵ Sign E Above
        0x1036 => (Some(Bindu), Some(TopPosition)), // ံ Anusvara
        0x1037 => (Some(ToneMarker), Some(BottomPosition)), // ့ Dot Below
        0x1038 => (Some(Visarga), Some(RightPosition)), // း Visarga
        0x1039 => (Some(InvisibleStacker), None), // ္ Virama
        0x103A => (Some(PureKiller), Some(TopPosition)), // ် Asat
        0x103B => (Some(ConsonantMedial), Some(RightPosition)), // ျ Sign Medial Ya
        0x103C => (Some(ConsonantMedial), Some(TopLeftAndBottomPosition)), // ြ Sign Medial Ra
        0x103D => (Some(ConsonantMedial), Some(BottomPosition)), // ွ Sign Medial Wa
        0x103E => (Some(ConsonantMedial), Some(BottomPosition)), // ှ Sign Medial Ha
        0x103F => (Some(Consonant), None),        // ဿ Great Sa
        0x1040 => (Some(Number), None),           // ၀ Digit Zero
        0x1041 => (Some(Number), None),           // ၁ Digit One
        0x1042 => (Some(Number), None),           // ၂ Digit Two
        0x1043 => (Some(Number), None),           // ၃ Digit Three
        0x1044 => (Some(Number), None),           // ၄ Digit Four
        0x1045 => (Some(Number), None),           // ၅ Digit Five
        0x1046 => (Some(Number), None),           // ၆ Digit Six
        0x1047 => (Some(Number), None),           // ၇ Digit Seven
        0x1048 => (Some(Number), None),           // ၈ Digit Eight
        0x1049 => (Some(Number), None),           // ၉ Digit Nine
        0x104A => (None, None),                   // ၊ Little Section
        0x104B => (None, None),                   // ။ Section
        0x104C => (None, None),                   // ၌ Locative
        0x104D => (None, None),                   // ၍ Completed
        0x104E => (Some(ConsonantPlaceholder), None), // ၎ Aforementioned
        0x104F => (None, None),                   // ၏ Genitive
        0x1050 => (Some(Consonant), None),        // ၐ Sha
        0x1051 => (Some(Consonant), None),        // ၑ Ssa
        0x1052 => (Some(VowelIndependent), None), // ၒ Vocalic R
        0x1053 => (Some(VowelIndependent), None), // ၓ Vocalic Rr
        0x1054 => (Some(VowelIndependent), None), // ၔ Vocalic L
        0x1055 => (Some(VowelIndependent), None), // ၕ Vocalic Ll
        0x1056 => (Some(VowelDependent), Some(RightPosition)), // ၖ Sign Vocalic R
        0x1057 => (Some(VowelDependent), Some(RightPosition)), // ၗ Sign Vocalic Rr
        0x1058 => (Some(VowelDependent), Some(BottomPosition)), // ၘ Sign Vocalic L
        0x1059 => (Some(VowelDependent), Some(BottomPosition)), // ၙ Sign Vocalic Ll
        0x105A => (Some(Consonant), None),        // ၚ Mon Nga
        0x105B => (Some(Consonant), None),        // ၛ Mon Jha
        0x105C => (Some(Consonant), None),        // ၜ Mon Bba
        0x105D => (Some(Consonant), None),        // ၝ Mon Bbe
        0x105E => (Some(ConsonantMedial), Some(BottomPosition)), // ၞ Sign Mon Medial Na
        0x105F => (Some(ConsonantMedial), Some(BottomPosition)), // ၟ Sign Mon Medial Ma
        0x1060 => (Some(ConsonantMedial), Some(BottomPosition)), // ၠ Sign Mon Medial La
        0x1061 => (Some(Consonant), None),        // ၡ Sgaw Karen Sha
        0x1062 => (Some(VowelDependent), Some(RightPosition)), // ၢ Sign Sgaw Karen Eu
        0x1063 => (Some(ToneMarker), Some(RightPosition)), // ၣ Tone Sgaw Karen Hathi
        0x1064 => (Some(ToneMarker), Some(RightPosition)), // ၤ Tone Sgaw Karen Ke Pho
        0x1065 => (Some(Consonant), None),        // ၥ Western Pwo Karen Tha
        0x1066 => (Some(Consonant), None),        // ၦ Western Pwo Karen Pwa
        0x1067 => (Some(VowelDependent), Some(RightPosition)), // ၧ Sign Western Pwo Karen Eu
        0x1068 => (Some(VowelDependent), Some(RightPosition)), // ၨ Sign Western Pwo Karen Ue
        0x1069 => (Some(ToneMarker), Some(RightPosition)), // ၩ Sign Western Pwo Karen Tone 1
        0x106A => (Some(ToneMarker), Some(RightPosition)), // ၪ Sign Western Pwo Karen Tone 2
        0x106B => (Some(ToneMarker), Some(RightPosition)), // ၫ Sign Western Pwo Karen Tone 3
        0x106C => (Some(ToneMarker), Some(RightPosition)), // ၬ Sign Western Pwo Karen Tone 4
        0x106D => (Some(ToneMarker), Some(RightPosition)), // ၭ Sign Western Pwo Karen Tone 5
        0x106E => (Some(Consonant), None),        // ၮ Eastern Pwo Karen Nna
        0x106F => (Some(Consonant), None),        // ၯ Eastern Pwo Karen Ywa
        0x1070 => (Some(Consonant), None),        // ၰ Eastern Pwo Karen Ghwa
        0x1071 => (Some(VowelDependent), Some(TopPosition)), // ၱ Sign Geba Karen I
        0x1072 => (Some(VowelDependent), Some(TopPosition)), // ၲ Sign Kayah Oe
        0x1073 => (Some(VowelDependent), Some(TopPosition)), // ၳ Sign Kayah U
        0x1074 => (Some(VowelDependent), Some(TopPosition)), // ၴ Sign Kayah Ee
        0x1075 => (Some(Consonant), None),        // ၵ Shan Ka
        0x1076 => (Some(Consonant), None),        // ၶ Shan Kha
        0x1077 => (Some(Consonant), None),        // ၷ Shan Ga
        0x1078 => (Some(Consonant), None),        // ၸ Shan Ca
        0x1079 => (Some(Consonant), None),        // ၹ Shan Za
        0x107A => (Some(Consonant), None),        // ၺ Shan Nya
        0x107B => (Some(Consonant), None),        // ၻ Shan Da
        0x107C => (Some(Consonant), None),        // ၼ Shan Na
        0x107D => (Some(Consonant), None),        // ၽ Shan Pha
        0x107E => (Some(Consonant), None),        // ၾ Shan Fa
        0x107F => (Some(Consonant), None),        // ၿ Shan Ba
        0x1080 => (Some(Consonant), None),        // ႀ Shan Tha
        0x1081 => (Some(Consonant), None),        // ႁ Shan Ha
        0x1082 => (Some(ConsonantMedial), Some(BottomPosition)), // ႂ Sign Shan Medial Wa
        0x1083 => (Some(VowelDependent), Some(RightPosition)), // ႃ Sign Shan Aa
        0x1084 => (Some(VowelDependent), Some(LeftPosition)), // ႄ Sign Shan E
        0x1085 => (Some(VowelDependent), Some(TopPosition)), // ႅ Sign Shan E Above
        0x1086 => (Some(VowelDependent), Some(TopPosition)), // ႆ Sign Shan Final Y
        0x1087 => (Some(ToneMarker), Some(RightPosition)), // ႇ Sign Shan Tone 2
        0x1088 => (Some(ToneMarker), Some(RightPosition)), // ႈ Sign Shan Tone 3
        0x1089 => (Some(ToneMarker), Some(RightPosition)), // ႉ Sign Shan Tone 5
        0x108A => (Some(ToneMarker), Some(RightPosition)), // ႊ Sign Shan Tone 6
        0x108B => (Some(ToneMarker), Some(RightPosition)), // ႋ Sign Shan Council Tone 2
        0x108C => (Some(ToneMarker), Some(RightPosition)), // ႌ Sign Shan Council Tone 3
        0x108D => (Some(ToneMarker), Some(BottomPosition)), // ႍ Sign Shan Council Emphatic Tone
        0x108E => (Some(Consonant), None),        // ႎ Rumai Palaung Fa
        0x108F => (Some(ToneMarker), Some(RightPosition)), // ႏ Sign Rumai Palaung Tone 5
        0x1090 => (Some(Number), None),           // ႐ Shan Digit Zero
        0x1091 => (Some(Number), None),           // ႑ Shan Digit One
        0x1092 => (Some(Number), None),           // ႒ Shan Digit Two
        0x1093 => (Some(Number), None),           // ႓ Shan Digit Three
        0x1094 => (Some(Number), None),           // ႔ Shan Digit Four
        0x1095 => (Some(Number), None),           // ႕ Shan Digit Five
        0x1096 => (Some(Number), None),           // ႖ Shan Digit Six
        0x1097 => (Some(Number), None),           // ႗ Shan Digit Seven
        0x1098 => (Some(Number), None),           // ႘ Shan Digit Eight
        0x1099 => (Some(Number), None),           // ႙ Shan Digit Nine
        0x109A => (Some(ToneMarker), Some(RightPosition)), // ႚ Sign Khamti Tone 1
        0x109B => (Some(ToneMarker), Some(RightPosition)), // ႛ Sign Khamti Tone 3
        0x109C => (Some(VowelDependent), Some(RightPosition)), // ႜ Sign Aiton A
        0x109D => (Some(VowelDependent), Some(TopPosition)), // ႝ Sign Aiton Ai
        0x109E => (Some(Symbol), None),           // ႞ Shan One
        0x109F => (Some(Symbol), None),           // ႟ Shan Exclamation

        // Myanmar Extended A character table
        0xAA60 => (Some(Consonant), None), // ꩠ Khamti Ga
        0xAA61 => (Some(Consonant), None), // ꩡ Khamti Ca
        0xAA62 => (Some(Consonant), None), // ꩢ Khamti Cha
        0xAA63 => (Some(Consonant), None), // ꩣ Khamti Ja
        0xAA64 => (Some(Consonant), None), // ꩤ Khamti Jha
        0xAA65 => (Some(Consonant), None), // ꩥ Khamti Nya
        0xAA66 => (Some(Consonant), None), // ꩦ Khamti Tta
        0xAA67 => (Some(Consonant), None), // ꩧ Khamti Ttha
        0xAA68 => (Some(Consonant), None), // ꩨ Khamti Dda
        0xAA69 => (Some(Consonant), None), // ꩩ Khamti Ddha
        0xAA6A => (Some(Consonant), None), // ꩪ Khamti Dha
        0xAA6B => (Some(Consonant), None), // ꩫ Khamti Na
        0xAA6C => (Some(Consonant), None), // ꩬ Khamti Sa
        0xAA6D => (Some(Consonant), None), // ꩭ Khamti Ha
        0xAA6E => (Some(Consonant), None), // ꩮ Khamti Hha
        0xAA6F => (Some(Consonant), None), // ꩯ Khamti Fa
        0xAA70 => (None, None),            // ꩰ Khamti Reduplication
        0xAA71 => (Some(Consonant), None), // ꩱ Khamti Xa
        0xAA72 => (Some(Consonant), None), // ꩲ Khamti Za
        0xAA73 => (Some(Consonant), None), // ꩳ Khamti Ra
        0xAA74 => (Some(ConsonantPlaceholder), None), // ꩴ Khamti Oay
        0xAA75 => (Some(ConsonantPlaceholder), None), // ꩵ Khamti Qn
        0xAA76 => (Some(ConsonantPlaceholder), None), // ꩶ Khamti Hm
        0xAA77 => (Some(Symbol), None),    // ꩷ Khamti Aiton Exclamation
        0xAA78 => (Some(Symbol), None),    // ꩸ Khamti Aiton One
        0xAA79 => (Some(Symbol), None),    // ꩹ Khamti Aiton Two
        0xAA7A => (Some(Consonant), None), // ꩺ Khamti Aiton Ra
        0xAA7B => (Some(ToneMarker), Some(RightPosition)), // ꩻ Sign Pao Karen Tone
        0xAA7C => (Some(ToneMarker), Some(TopPosition)), // ꩼ Sign Tai Laing Tone 2
        0xAA7D => (Some(ToneMarker), Some(RightPosition)), // ꩽ Sign Tai Laing Tone 5
        0xAA7E => (Some(Consonant), None), // ꩾ Shwe Palaung Cha
        0xAA7F => (Some(Consonant), None), // ꩿ Shwe Palaung Sha

        // Myanmar Extended B character table
        0xA9E0 => (Some(Consonant), None), // ꧠ Shan Gha
        0xA9E1 => (Some(Consonant), None), // ꧡ Shan Cha
        0xA9E2 => (Some(Consonant), None), // ꧢ Shan Jha
        0xA9E3 => (Some(Consonant), None), // ꧣ Shan Nna
        0xA9E4 => (Some(Consonant), None), // ꧤ Shan Bha
        0xA9E5 => (Some(VowelDependent), Some(TopPosition)), // ꧥ Sign Shan Saw
        0xA9E6 => (None, None),            // ꧦ Shan Reduplication
        0xA9E7 => (Some(Consonant), None), // ꧧ Tai Laing Nya
        0xA9E8 => (Some(Consonant), None), // ꧨ Tai Laing Fa
        0xA9E9 => (Some(Consonant), None), // ꧩ Tai Laing Ga
        0xA9EA => (Some(Consonant), None), // ꧪ Tai Laing Gha
        0xA9EB => (Some(Consonant), None), // ꧫ Tai Laing Ja
        0xA9EC => (Some(Consonant), None), // ꧬ Tai Laing Jha
        0xA9ED => (Some(Consonant), None), // ꧭ Tai Laing Dda
        0xA9EE => (Some(Consonant), None), // ꧮ Tai Laing Ddha
        0xA9EF => (Some(Consonant), None), // ꧯ Tai Laing Nna
        0xA9F0 => (Some(Number), None),    // ꧰ Tai Laing Digit Zero
        0xA9F1 => (Some(Number), None),    // ꧱ Tai Laing Digit One
        0xA9F2 => (Some(Number), None),    // ꧲ Tai Laing Digit Two
        0xA9F3 => (Some(Number), None),    // ꧳ Tai Laing Digit Three
        0xA9F4 => (Some(Number), None),    // ꧴ Tai Laing Digit Four
        0xA9F5 => (Some(Number), None),    // ꧵ Tai Laing Digit Five
        0xA9F6 => (Some(Number), None),    // ꧶ Tai Laing Digit Six
        0xA9F7 => (Some(Number), None),    // ꧷ Tai Laing Digit Seven
        0xA9F8 => (Some(Number), None),    // ꧸ Tai Laing Digit Eight
        0xA9F9 => (Some(Number), None),    // ꧹ Tai Laing Digit Nine
        0xA9FA => (Some(Consonant), None), // ꧺ Tai Laing Lla
        0xA9FB => (Some(Consonant), None), // ꧻ Tai Laing Da
        0xA9FC => (Some(Consonant), None), // ꧼ Tai Laing Dha
        0xA9FD => (Some(Consonant), None), // ꧽ Tai Laing Ba
        0xA9FE => (Some(Consonant), None), // ꧾ Tai Laing Bha

        // Miscellaneous character table
        0x00A0 => (Some(Placeholder), None),  //   No-break space
        0x200C => (Some(NonJoiner), None),    // ‌ Zero-width non-joiner
        0x200D => (Some(Joiner), None),       // ‍ Zero-width joiner
        0x2010 => (Some(Placeholder), None),  // ‐ Hyphen
        0x2011 => (Some(Placeholder), None),  // ‑ No-break hyphen
        0x2012 => (Some(Placeholder), None),  // ‒ Figure dash
        0x2013 => (Some(Placeholder), None),  // – En dash
        0x2014 => (Some(Placeholder), None),  // — Em dash
        0x25CC => (Some(DottedCircle), None), // ◌ Dotted circle

        _ => (None, None),
    }
}

/////////////////////////

pub trait SyllableChar {
    fn char(&self) -> char;
}

impl SyllableChar for char {
    fn char(&self) -> char {
        *self
    }
}

/// Matches against a single character
pub fn match_one<T: SyllableChar>(f: impl Fn(char) -> bool) -> impl Fn(&[T]) -> Option<usize> {
    move |cs: &[T]| match cs.get(0) {
        Some(c) if f(c.char()) => Some(1),
        _ => None,
    }
}

/// Succeeds if `f` succeeds otherwise consumes nothing
pub fn match_optional<T: SyllableChar>(
    f: impl Fn(&[T]) -> Option<usize>,
) -> impl Fn(&[T]) -> Option<usize> {
    move |cs: &[T]| f(cs).or(Some(0))
}

/// `f? g`: matches either `g` or `f g`
pub fn match_optional_seq<T: SyllableChar>(
    f: impl Fn(&[T]) -> Option<usize>,
    g: impl Fn(&[T]) -> Option<usize>,
) -> impl Fn(&[T]) -> Option<usize> {
    move |cs: &[T]| match_either(&g, match_seq(&f, &g))(cs)
}

#[allow(dead_code)]
pub fn match_repeat_num<T: SyllableChar>(
    num: usize,
    f: impl Fn(&[T]) -> Option<usize>,
) -> impl Fn(&[T]) -> Option<usize> {
    move |mut cs: &[T]| {
        let mut total = 0;
        for _ in 0..num {
            let n = f(cs)?;
            total += n;
            cs = &cs[n..];
        }
        Some(total)
    }
}

/// Match up to `max` instances of `f`, followed by `g`
pub fn match_repeat_upto<T: SyllableChar>(
    max: usize,
    f: impl Fn(&[T]) -> Option<usize>,
    g: impl Fn(&[T]) -> Option<usize>,
) -> impl Fn(&[T]) -> Option<usize> {
    move |mut cs: &[T]| {
        // Initial case: zero f matches
        let mut best = g(cs);
        let mut nf = 0;
        for _ in 0..max {
            // Match up to max instances of f
            if let Some(n) = f(cs) {
                nf += n;
                cs = &cs[n..];
                // If f is followed by g update matching range
                if let Some(ng) = g(cs) {
                    best = Some(nf + ng);
                }
            } else {
                break;
            }
        }
        best
    }
}

/// Match `f1` followed by `f2`.
///
/// Fails if `f1` or `f2` fail.
pub fn match_seq<T: SyllableChar>(
    f1: impl Fn(&[T]) -> Option<usize>,
    f2: impl Fn(&[T]) -> Option<usize>,
) -> impl Fn(&[T]) -> Option<usize> {
    move |cs: &[T]| {
        let n1 = f1(cs)?;
        let n2 = f2(&cs[n1..])?;
        Some(n1 + n2)
    }
}

/// Matches whichever of `f1` or `f2` match the most input.
///
/// Uses `f2`'s match if they match the same input
pub fn match_either<T: SyllableChar>(
    f1: impl Fn(&[T]) -> Option<usize>,
    f2: impl Fn(&[T]) -> Option<usize>,
) -> impl Fn(&[T]) -> Option<usize> {
    move |cs: &[T]| {
        let n1 = f1(cs);
        let n2 = f2(cs);
        std::cmp::max(n1, n2)
    }
}