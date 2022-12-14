use actix_web::web::{Data, Json};
use actix_web::{get, post, Responder};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, MySqlPool};

use crate::handler::result::{none, some};

#[derive(Deserialize, Serialize, Default, Debug)]
pub struct Delivery {
    delivery_id: Option<String>,
    customer_id: Option<String>,
    product_number: Option<String>,
    delivery_provider: Option<String>,
    order_price: Option<String>,
    gas_leak_rate: Option<String>,
    destination: Option<String>,
    create_date: Option<String>,
    finish_date: Option<String>,
    delivery_number: Option<String>,
    delivery_status: Option<String>,
    comments: Option<String>,
}

#[get("/api/deliveries")]
pub async fn get_delivery_info(data: Data<MySqlPool>) -> impl Responder {
    some(
        query_as!(
            Delivery,
            r#"SELECT 
            CAST(delivery_id AS CHAR) AS delivery_id,
            CAST(customer_id AS CHAR) AS customer_id,
            CAST(product_number AS CHAR) AS product_number,
            CAST(delivery_provider AS CHAR) AS delivery_provider,
            CAST(order_price AS CHAR) AS order_price,
            CAST(gas_leak_rate AS CHAR) AS gas_leak_rate,
            destination,
            DATE_FORMAT(create_date, '%Y-%m-%d %H:%i:%s') AS create_date,
            DATE_FORMAT(finish_date, '%Y-%m-%d %H:%i:%s') AS finish_date,
            delivery_number,
            CAST(delivery_status AS CHAR) AS delivery_status,
            comments
        FROM
            delivery
        ORDER BY create_date DESC
        LIMIT 20"#,
        )
        .fetch_all(data.get_ref())
        .await,
    )
}

#[post("/api/delivery/add")]
pub async fn add_delivery(data: Data<MySqlPool>, Json(deliver): Json<Delivery>) -> impl Responder {
    none(match deliver.delivery_id {
        None => query!(
            r#"insert into delivery (customer_id, product_number, delivery_provider, order_price, gas_leak_rate, destination, create_date, finish_date, delivery_number, delivery_status, comments) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);"#,
            deliver.customer_id,
            deliver.product_number,
            deliver.delivery_provider,
            deliver.order_price,
            deliver.gas_leak_rate,
            deliver.destination,
            deliver.create_date,
            deliver.finish_date,
            deliver.delivery_number,
            deliver.delivery_status,
            deliver.comments
        )
        .execute(data.get_ref())
        .await,
        Some(id) => {
            query!(
                r#"update delivery set customer_id = ? ,product_number=?, delivery_provider=?, order_price=?, gas_leak_rate=?, destination=?, create_date=?, finish_date=?, delivery_number=?, delivery_status=?, comments=? where delivery_id = ?"#,
                deliver.customer_id,
            deliver.product_number,
            deliver.delivery_provider,
            deliver.order_price,
            deliver.gas_leak_rate,
            deliver.destination,
            deliver.create_date,
            deliver.finish_date,
            deliver.delivery_number,
            deliver.delivery_status,
            deliver.comments,
                id
            )
            .execute(data.get_ref())
            .await
        }
    })
}

#[post("/api/delivery/delete")]
pub async fn delete_delivery(
    data: Data<MySqlPool>,
    Json(deliver): Json<Delivery>,
) -> impl Responder {
    none(match deliver.delivery_id {
        Some(id) => {
            query!(r#"delete from delivery where delivery_id = ?"#, id)
                .execute(data.get_ref())
                .await
        }
        None => todo!(),
    })
}
