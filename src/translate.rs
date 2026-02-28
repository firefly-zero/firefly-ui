use firefly_rust::Language;

pub trait Translate<'a> {
    fn translate(&self, lang: Language) -> &'a str {
        match lang {
            Language::English => self.translate_english(),
            Language::Dutch => self.translate_dutch(),
            Language::French => self.translate_french(),
            Language::German => self.translate_german(),
            Language::Italian => self.translate_italian(),
            Language::Polish => self.translate_polish(),
            Language::Romanian => self.translate_romanian(),
            Language::Russian => self.translate_russian(),
            Language::Spanish => self.translate_spanish(),
            Language::Swedish => self.translate_swedish(),
            Language::Turkish => self.translate_turkish(),
            Language::Ukrainian => self.translate_ukrainian(),
            Language::TokiPona => self.translate_toki_pona(),
        }
    }

    fn translate_english(&self) -> &'a str;
    fn translate_dutch(&self) -> &'a str;
    fn translate_french(&self) -> &'a str;
    fn translate_german(&self) -> &'a str;
    fn translate_italian(&self) -> &'a str;
    fn translate_polish(&self) -> &'a str;
    fn translate_romanian(&self) -> &'a str;
    fn translate_russian(&self) -> &'a str;
    fn translate_spanish(&self) -> &'a str;
    fn translate_swedish(&self) -> &'a str;
    fn translate_turkish(&self) -> &'a str;
    fn translate_ukrainian(&self) -> &'a str;
    fn translate_toki_pona(&self) -> &'a str;
}
