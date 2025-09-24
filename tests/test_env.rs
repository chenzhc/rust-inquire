#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{env, sync::{Arc}};
use log::info;
use rust_inquire::{init, utils};
use sqlx::{database, mysql::MySqlPool, Executor};
use tokio::sync::Mutex;

#[test]
fn it_env_test() {
    init();

    let my_host = env::var("MYSQL_HOST");
    info!("{}", my_host.unwrap());
    
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct SysUser {
    user_id: i32,
    user_name: String,
    email: String,
}

#[tokio::test]
async fn it_conn_db_test() -> anyhow::Result<()> {
    init();
    // 获取数据库连接 url
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 初始化数据库连接池
    let pool = MySqlPool::connect(&database_url).await?;

    let users = sqlx::query_as_unchecked!(
        SysUser, 
        "select user_id, user_name,  email from sys_user "
    )
    .fetch_all(&pool)
    .await?;

    info!("{:?}", users);

    Ok(())
}



#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Order {
    id: i32,
    user_id: i32,
    product_id: i32,
    quantity: i32,
    status: String,
}   

#[tokio::test]
async fn it_conn_db_test02() -> anyhow::Result<()> {
    init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPool::connect(&database_url).await?;

    let order = Order {
        id: 1,
        user_id: 1,
        product_id: 1,
        quantity: 2,
        status: "pending".to_string(),
    };

    let _ = sqlx::query!(
        "insert into orders (  user_id, product_id, quantity, status) values ( ?, ?,?,?) ",
        order.user_id,
        order.product_id,
        order.quantity,
        order.status
    )
    .execute(&pool)
    .await?;

    let orders = sqlx::query_as_unchecked!(
        Order,
        "select id, user_id, product_id, quantity, status from orders "
    )
    .fetch_all(&pool)
    .await?;

    info!("Orders: {:?}", orders);

    Ok(())
}


#[tokio::test]
async fn it_conn_db_test03() -> anyhow::Result<()> {
    init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool_old = MySqlPool::connect(&database_url).await?;
    let pool = Arc::new(Mutex::new(pool_old.clone()));

    
    // 模拟微服务之间的数据一致性
    let pool1 = pool.clone();
    let handle1 = tokio::spawn(async move {
        let mut conn = pool1.lock().await;
        conn.execute(sqlx::query("update orders set status = 'test' where id = 1"))
            .await.unwrap();
    });

    let pool2 = pool.clone();
    let handle2 = tokio::spawn(async move {
        let mut conn = pool2.lock().await;
        conn.execute(sqlx::query("update orders set status = 'shipped' where id = 1"))
            .await
            .unwrap();
    });

    let _ = tokio::join!(handle1, handle2);

    let orders = sqlx::query_as_unchecked!(
        Order,
        "SELECT id, user_id, product_id, quantity, status FROM orders"
    )
    .fetch_all(&pool_old)
    .await?;

    info!("Orders: {:?}", orders);

    Ok(())
}



#[tokio::test]
async fn it_web_test01() -> anyhow::Result<()> {
    init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;

    let user = rust_inquire::models::auth::UserModel {
        id: 1i64,
        firstname: "test".to_string(),
        lastname: "lastname".to_string(),
        password: "pwd".to_string(),
        email: "test@email.com".to_string(),
    };

    let result = utils::db::users::insert(user, &pool).await;
    info!("{:?}", result);

    Ok(())
}


#[tokio::test]
async fn it_get_by_id_test01() -> anyhow::Result<()> {
    init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;

    let id = 1u64;
    let result = utils::db::users::get( id,&pool).await;
    info!("{:?}", result);

    Ok(())
}

#[tokio::test]
async fn it_delete_by_id_test01() -> anyhow::Result<()> {
    init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;

    let id = 1i64;
    let result = utils::db::users::delete(id, &pool).await?;
    info!("{:?}", result);

    Ok(())
}

#[tokio::test]
async fn it_get_all_test01() -> anyhow::Result<()> {
    init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url).await?;

    let result = utils::db::users::get_all(&pool).await?;
    info!("{:?}", result);
    
    Ok(())
}