use crate::operate_receipt::OperateReceipt;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GlobalResponse<T> {
    pub operate_receipt: OperateReceipt,
    pub data: T,
}

impl Responder for GlobalResponse<()> {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().body(serde_json::to_string(&self).unwrap())
    }
}
