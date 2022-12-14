use actix_web::web::{Data, Json};
use actix_web::{get, post, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, MySqlPool};

use crate::handler::result::{none, some};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Order {
    order_id: Option<String>,
    customer_name: Option<String>,
    customer_id: Option<String>,
    product_number: Option<String>,
    create_time: Option<String>,
    finish_time: Option<String>,
    is_profit: Option<String>,
    order_status: Option<String>,
    location: Option<String>,
    order_fee_status: Option<String>,
    comments: Option<String>,
}

#[get("/api/orders")]
pub async fn get_order_info(data: Data<MySqlPool>) -> impl Responder {
    some(
        query_as!(
            Order,
            r#"SELECT 
            CAST(order_id AS CHAR) AS order_id,
            CAST(customer_name AS CHAR) AS customer_name,
            CAST(customer_id AS CHAR) AS customer_id,
            CAST(product_number AS CHAR) AS product_number,
            DATE_FORMAT(create_time, '%Y-%m-%d %H:%i:%s') AS create_time,
            DATE_FORMAT(finish_time, '%Y-%m-%d %H:%i:%s') AS finish_time,
            CAST(is_profit AS CHAR) AS is_profit,
            CAST(order_status AS CHAR) AS order_status,
            location,
            CAST(order_fee_status AS CHAR) AS order_fee_status,
            comments
        FROM
            orders
        ORDER BY create_time DESC
        LIMIT 20"#,
        )
        .fetch_all(data.get_ref())
        .await,
    )
}

#[post("/api/order/add")]
pub async fn add_order(data: Data<MySqlPool>, Json(order): Json<Order>) -> impl Responder {
    none(match order.order_id {
        None => query!(
            r#"insert into orders (customer_name, customer_id, product_number, create_time, is_profit, order_status, location, comments) values (?, ?, ?, ?, ?, ?, ?, ?);"#,
            order.customer_name,
            order.customer_id,
            order.product_number,
            order.create_time,
            order.is_profit,
            order.order_status,
            order.location,
            order.comments
        )
        .execute(data.get_ref())
        .await,
        Some(id) => {
            query!(
                r#"update orders set customer_name = ? ,customer_id=?, product_number=?, create_time=?, finish_time=?, is_profit=?, order_status=?, location=?, order_fee_status=?, comments=? where order_id = ?"#,
                order.customer_name,
                order.customer_id,
                order.product_number,
                order.create_time,
                order.finish_time,
                order.is_profit,
                order.order_status,
                order.location,
                order.order_fee_status,
                order.comments,
                id
            )
            .execute(data.get_ref())
            .await
        }
    })
}

#[post("/api/order/delete")]
pub async fn delete_order(data: Data<MySqlPool>, Json(order): Json<Order>) -> impl Responder {
    none(match order.order_id {
        Some(id) => {
            query!(r#"delete from orders where order_id = ?"#, id)
                .execute(data.get_ref())
                .await
        }
        None => todo!(),
    })
}
