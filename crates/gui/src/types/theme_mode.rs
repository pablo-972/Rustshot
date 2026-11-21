#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq
)]
pub(crate) enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}
