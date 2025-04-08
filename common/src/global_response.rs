use crate::operate_receipt::OperateReceipt;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GlobalResponse<T:Serialize> {
    pub operate_receipt: OperateReceipt,
    pub data: Option<T>,
}

impl<T: Serialize> Responder for GlobalResponse<T> {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().body(serde_json::to_string(&self).unwrap())
    }
}
