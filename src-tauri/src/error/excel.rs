use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExcelError {
    #[error("路径不存在")]
    FileNotFound,
    #[error("该路径不存在文件")]
    NotAFile,
    #[error("读取该文件失败")]
    ReadError,
    #[error("密码错误")]
    DecryptError,
    #[error("工作簿不能为空")]
    WorkbookNotNull,
    #[error("未找到指定的工作簿")]
    WorkbookNotExisted,
    #[error("无法获取到该工作簿发生了错误{0}")]
    WorkbookError(String),
}
