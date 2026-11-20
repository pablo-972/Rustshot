// Which changes the results window shows.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq
)]
pub(crate) enum Filter {
    #[default]
    All,
    KeysAdded,
    KeysDeleted,
    ValuesAdded,
    ValuesDeleted,
    Modified,
}
