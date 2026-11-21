// Text of the bottom bar.
#[derive(Debug, Clone, Default)]
pub(crate) struct Status {
    pub text: String,
    pub error: bool,
}


impl Status {
    pub(crate) fn info(text: &str) -> Self {
        Status { text: text.to_string(), error: false }
    }

    pub(crate) fn error(text: &str) -> Self {
        Status { text: text.to_string(), error: true }
    }
}
