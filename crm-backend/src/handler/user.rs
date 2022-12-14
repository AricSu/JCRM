use crate::handler::result::some;
use actix_web::web::Data;
use actix_web::{get, post, web, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, MySqlPool};
use web::Json;

#[derive(Deserialize, Serialize)]
pub struct LoginInfo {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Debug)]
struct UserInfo {
    name: Option<String>,
    introduction: Option<String>,
    avatar: Option<String>,
    roles: Option<String>,
    token: String,
}

#[derive(Deserialize, Debug)]
pub struct TokenCheck {
    token: String,
}

#[get("/api/user/info")]
pub async fn get_user_info(data: Data<MySqlPool>, token: web::Query<TokenCheck>) -> impl Responder {
    if token.0.token == "admin" {
        some( query_as!(
                    UserInfo,
                    r#"select name,introduction,avatar,roles,"admin" as token from users where name = 'admin'"#
                )
                .fetch_one(data.get_ref())
                .await)
    } else {
        todo!()
    }
}

#[post("/api/user/login")]
pub async fn login(data: Data<MySqlPool>, Json(login_info): Json<LoginInfo>) -> impl Responder {
    let user = query_as!(
        UserInfo,
        r#"select name,introduction,avatar,roles,"admin" as token from users where name = ? and password = ?"#,
        login_info.username,
        login_info.password
    )
    .fetch_one(data.get_ref())
    .await;
    some(user)
}

#[post("/api/user/logout")]
pub async fn logout(_data: Data<MySqlPool>) -> impl Responder {
    #[derive(Serialize)]
    struct Resp {
        code: i32,
        data: Option<String>,
    }
    Json(Resp {
        code: 200,
        data: None,
    })
}

// #[post("user")]
// pub async fn post(data: Data<MySqlPool>, Json(v): Json<SaveUserQuery>) -> impl Responder {
//     none(
//         match v.id {
//             None => query!(
//                 r#"insert into "user" (name, created_at) values ($1, now())"#,
//                 v.name
//             ).execute(data.get_ref()).await,
//             Some(id) => query!(
//                 r#"update "user" set name = $1, updated_at = now() where deleted_at is null and id = $2"#,
//                 v.name,
//                 id
//             ).execute(data.get_ref()).await,
//         }
//     )
// }

// #[delete("user/{id}")]
// pub async fn delete(data: Data<MySqlPool>, path: Path<i64>) -> impl Responder {
//     let id = path.into_inner();
//     none(
//         query!(r#"update "user" set deleted_at = now() where id = $1"#, id)
//             .execute(data.get_ref())
//             .await,
//     )
// }
