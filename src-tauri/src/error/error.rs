#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("failed to parse as string: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

#[derive(serde::Serialize)]
#[serde(tag = "kind", content = "msg")]
#[serde(rename_all = "camelCase")]
enum ErrorKind {
    Io { code: u16, msg: String },
    Utf8 { code: u16, msg: String },
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            Self::Io(_) => ErrorKind::Io {
                code: 101,
                msg: error_message,
            },
            Self::Utf8(_) => ErrorKind::Utf8 {
                code: 102,
                msg: error_message,
            },
        };
        error_kind.serialize(serializer)
    }
}
