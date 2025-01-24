use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommonError {
    #[error("es 账号不能为空")]
    EsUserNotEmpty,
    #[error("es 密码不能为空")]
    EsPwdNotEmpty,
    #[error("es urls配置错误{0}")]
    EsUrlConfigError(String),
    #[error("transport err:{0}")]
    EstransportError(String),
    #[error("解析json字段{0}错误")]
    ParseJsonError(String),
    #[error("配置yml中的total_name不能为空")]
    TotalNameError,
}
