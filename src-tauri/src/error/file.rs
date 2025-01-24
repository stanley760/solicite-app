use thiserror::Error;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("路径不存在")]
    FileNotFound,
    #[error("配置文件不存在")]
    ConfigFileNotFound,
    #[error("该路径不存在文件")]
    NotAFile,
    #[error("文件名不是有效的 UTF-8 字符串")]
    FileNameUTF8Error,
    #[error("读取文件失败")]
    ReadError,
    #[error("保存文件内容不能为空")]
    SaveContentNotNull,
    #[error("写入文件失败")]
    WriteError,
    #[error("yml文件反序列化失败{0}")]
    YamlDeserializeError(String),
    #[error("json文件反序列化失败{0}")]
    JsonDeserializeError(String),
}
