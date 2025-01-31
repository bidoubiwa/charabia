use deunicode::deunicode_char;

use crate::token::SeparatorKind;

pub fn classify_separator(c: char) -> Option<SeparatorKind> {
    match deunicode_char(c)?.chars().next()? {
        // Prevent deunicoding cyrillic chars (e.g. ь -> ' is incorrect)
        _ if ('\u{0410}'..='\u{044f}').contains(&c) => None, // russian cyrillic letters [а-яА-Я]
        _ if c == '\u{00a0}' => None,                        // non-breaking space
        c if c.is_whitespace() => Some(SeparatorKind::Soft), // whitespaces
        '-' | '_' | '\'' | ':' | '/' | '\\' | '@' | '"' | '+' | '~' | '=' | '^' | '*' | '#' => {
            Some(SeparatorKind::Soft)
        }
        '.' | ';' | ',' | '!' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' => {
            Some(SeparatorKind::Hard)
        }
        _ => None,
    }
}

macro_rules! make_language {
    ($($language:tt), +) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum Language {
            $($language),+,
            Other,
        }
        impl From<whatlang::Lang> for Language {
            fn from(other: whatlang::Lang) -> Language {
                match other {
                    $(whatlang::Lang::$language => Language::$language), +
                }
            }
        }

        impl Default for Language {
            fn default() -> Self {
                Self::Other
            }
        }
    };
}

make_language! {
    Epo,
    Eng,
    Rus,
    Cmn,
    Spa,
    Por,
    Ita,
    Ben,
    Fra,
    Deu,
    Ukr,
    Kat,
    Ara,
    Hin,
    Jpn,
    Heb,
    Yid,
    Pol,
    Amh,
    Jav,
    Kor,
    Nob,
    Dan,
    Swe,
    Fin,
    Tur,
    Nld,
    Hun,
    Ces,
    Ell,
    Bul,
    Bel,
    Mar,
    Kan,
    Ron,
    Slv,
    Hrv,
    Srp,
    Mkd,
    Lit,
    Lav,
    Est,
    Tam,
    Vie,
    Urd,
    Tha,
    Guj,
    Uzb,
    Pan,
    Aze,
    Ind,
    Tel,
    Pes,
    Mal,
    Ori,
    Mya,
    Nep,
    Sin,
    Khm,
    Tuk,
    Aka,
    Zul,
    Sna,
    Afr,
    Lat,
    Slk,
    Cat,
    Tgl
}

macro_rules! make_script {
    ($($script:tt), +) => {
        #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
        pub enum Script {
            $($script),+,
            Other,
        }

        impl From<whatlang::Script> for Script {
            fn from(other: whatlang::Script) -> Script {
                match other {
                    $(whatlang::Script::$script => Script::$script), +
                }
            }

        }

        impl Default for Script {
            fn default() -> Self {
                Self::Other
            }
        }
    };
}

make_script! {
    Arabic,
    Bengali,
    Cyrillic,
    Devanagari,
    Ethiopic,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Hangul,
    Hebrew,
    Hiragana,
    Kannada,
    Katakana,
    Khmer,
    Latin,
    Malayalam,
    Mandarin,
    Myanmar,
    Oriya,
    Sinhala,
    Tamil,
    Telugu,
    Thai
}

pub struct StrDetection<'a> {
    inner: &'a str,
    pub script: Option<Script>,
    pub language: Option<Language>,
}

impl<'a> StrDetection<'a> {
    pub fn new(inner: &'a str) -> Self {
        Self { inner, script: None, language: None }
    }

    pub fn script(&mut self) -> Script {
        let inner = self.inner;
        *self.script.get_or_insert_with(|| Self::detect_script(inner))
    }

    pub fn language(&mut self) -> Language {
        let inner = self.inner;
        *self.language.get_or_insert_with(|| Self::detect_lang(inner))
    }

    /// detect script with whatlang,
    /// if no script is detected, return Script::Other
    fn detect_script(text: &str) -> Script {
        whatlang::detect_script(text).map(Script::from).unwrap_or_default()
    }

    /// detect lang with whatlang
    /// if no language is detected, return Language::Other
    fn detect_lang(text: &str) -> Language {
        whatlang::detect_lang(text).map(Language::from).unwrap_or_default()
    }
}

pub trait Detect {
    fn detect(&self) -> StrDetection;
}

impl Detect for &str {
    fn detect(&self) -> StrDetection {
        StrDetection::new(self)
    }
}
