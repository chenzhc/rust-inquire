#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{env, sync::{Arc}};
use actix_web::rt::task;
use log::info;
use rust_inquire::{init, utils};
use sqlx::{database, mysql::MySqlPool, Executor};
use tokio::{fs::{self, File}, io::{AsyncReadExt, BufReader}, stream, sync::{mpsc, Mutex}, task::JoinSet};
use tokio_stream::{self, StreamExt};

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

#[tokio::test]
async fn it_channel_test05()  {
    init();
    let bufer_size = 10000;
    // 有界通道防爆内存
    let (tx, mut rx) = mpsc::channel::<u32>(bufer_size);

    // 生产者
    let producer = tokio::spawn(async move {
        for i in 0..bufer_size as u32 {
            tx.send(i).await.unwrap();
        }
    });

    // 消费者：buffer_unordered 控制并发度
    // let mut tasks = tokio_stream::iter(0..bufer_size).map(|_| rx.recv());
    // let mut buffered = tasks.buffer_unordered(100);
    // while let Some(Some(num)) = buffered.next().await {
    //     info!("Processed: {}", num);
    // }

    producer.await.unwrap();

}

#[tokio::test]
async fn it_async_read_file_test01() -> anyhow::Result<()> {
    init();
    let file_path = "http-test.http";
    let file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    // 异步读取 
    reader.read_to_string(&mut contents).await?;

    // 重 IO 处理; 用 spawn_blocking 
    let processed = task::spawn_blocking(move || {
        // 模拟 CPU/IO 混合, 如解析
        contents.lines().count()
    }).await?;

    info!("lines: {}", processed);

    Ok(())
}

// 复制多个大文件，IO 密集。
#[tokio::test]
async fn it_copy_test01() -> anyhow::Result<()> {
    init();
    let mut set = JoinSet::new();

    for i in 0..10 {
        set.spawn(async move {
            let src = format!("src{}.bin", i);
            let dst = format!("dst{}.bin", i);
            fs::copy(&src, &dst).await
        });
    }

    while let Some(res) = set.join_next().await {
        res??;
    }

    Ok(())
}

