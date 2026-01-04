#[derive(Default, Clone, Debug)]
pub enum LexerMode {
    #[default]
    Text,
    Markup,
    Quote,
}
