pub mod operate_receipt;
pub mod global_response;
pub mod request;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


#[macro_export]
macro_rules! response {
    ($arg:expr) => {{
        match $arg {
            Result::Ok(value) => $crate::global_response::GlobalResponse {
                operate_receipt: $crate::operate_receipt::OperateReceipt::Success,
                data: Some(value),
            },
            Result::Err(error) => $crate::global_response::GlobalResponse {
                operate_receipt: error,
                data: None,
            }
        }
    }};
}
