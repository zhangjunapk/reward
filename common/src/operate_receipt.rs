use serde::Serialize;

#[derive(Debug, Clone,Serialize)]
pub enum OperateReceipt {
    Success,
    Error(String),
    Exists(String),
}
