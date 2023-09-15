#[derive(Debug, thiserror::Error)]
pub enum UserConfigError {
    #[error("select {select:?} not in range {range:?}")]
    SelectOutOfRange {
        range: Box<[String]>,
        select: String,
    },
    #[error("unknown config type [{0:?}]")]
    UnknownConfigType(String),
    #[error("config type not match {left:?} != {right:?}")]
    TypeNotMatch {
        left: &'static str,
        right: &'static str,
    },

    #[error("config [{id:?}] not define")]
    NotDefine { id: String },
}

impl From<UserConfigError> for mlua::Error {
    fn from(value: UserConfigError) -> Self {
        Self::external(value)
    }
}

impl UserConfigError {
    pub fn new_out_range<T, I, S, R>(select: String, range: I) -> Result<T, R>
    where
        I: Iterator<Item = S>,
        S: ToString,
        R: From<Self>,
    {
        Err(Self::SelectOutOfRange {
            range: range.map(|s| s.to_string()).collect(),
            select,
        }
        .into())
    }

    pub fn new_unknown<T, R: From<Self>>(unknown: &str) -> Result<T, R> {
        Err(Self::UnknownConfigType(unknown.to_string()).into())
    }

    pub fn new_not_match(left: &'static str, right: &'static str) -> Self {
        Self::TypeNotMatch { left, right }
    }
}
