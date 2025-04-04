#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CannotConvert {
    from: String,
    to: String,
}

impl CannotConvert {
    pub fn new(from: &str, to: &str) -> Self {
        CannotConvert {
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}

impl std::fmt::Display for CannotConvert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot convert {} to {}", self.from, self.to)
    }
}
