use actix_web::http::header::CONTENT_TYPE;
use actix_web::web;
use actix_web::Responder;
use serde::{Deserialize, Serialize};

mod delivery;
mod order;
mod user;

pub const APPLICATION_JSON: &str = "application/json";

pub fn route(config: &mut web::ServiceConfig) {
    // config.service(user::all);
    config.service(user::login);
    config.service(user::logout);
    config.service(user::get_user_info);
    config.service(order::get_order_info);
    config.service(order::add_order);
    config.service(order::delete_order);
    config.service(delivery::get_delivery_info);
    config.service(delivery::add_delivery);
    config.service(delivery::delete_delivery);
}

pub async fn default_route() -> impl Responder {
    r#"{"code":"0","message":"404 Not Found"}"#
        .customize()
        .insert_header((CONTENT_TYPE, APPLICATION_JSON))
}

#[derive(Serialize, Deserialize)]
pub struct HttpResult<T> {
    code: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "String::is_empty")]
    message: String,
}

mod result {
    use std::fmt::Display;

    use actix_web::web::Json;
    use log::error;

    use crate::handler::HttpResult;

    fn err<T, M: Display>(message: M) -> Json<HttpResult<T>> {
        let message = message.to_string();
        error!(target: "main", "{}", message);
        Json(HttpResult {
            code: 1,
            data: None,
            message,
        })
    }

    /// 响应数据
    pub fn some<T, E: Display>(v: crate::Result<T, E>) -> Json<HttpResult<T>> {
        match v {
            Ok(o) => Json(HttpResult {
                code: 200,
                data: Some(o),
                message: "".into(),
            }),
            Err(e) => err(e),
        }
    }

    /// 不响应数据
    pub fn none<T, E: Display>(v: crate::Result<T, E>) -> Json<HttpResult<()>> {
        match v {
            Ok(_) => Json(HttpResult {
                code: 200,
                data: None,
                message: "".into(),
            }),
            Err(e) => err(e),
        }
    }
}
