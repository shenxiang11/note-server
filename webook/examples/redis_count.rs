use deadpool::Runtime;
use deadpool_redis::Config;
use redis::AsyncCommands;

#[tokio::main]
async fn main() {
    // balance_v1().await;
    balance_v2().await;
}

async fn balance_v1() {
    let redis_cfg = Config::from_url("redis://127.0.0.1:6379");

    let rdb = redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create redis pool");

    let mut con = rdb.get().await.unwrap();

    let _: () = con.set("balance", 100).await.unwrap();

    // 创建线程 A 和 B
    let rdb1 = rdb.clone();
    let handle_a = tokio::spawn(async move {
        // 线程 A
        let mut con = rdb1.get().await.unwrap();
        let mut balance: i32 = con.get("balance").await.unwrap();
        balance -= 50; // 扣除 50
        let _: () = con.set("balance", balance).await.unwrap(); // 设置余额为 50
        println!("Thread A finished, balance: {}", balance);
    });

    let rdb2 = rdb.clone();
    let handle_b = tokio::spawn(async move {
        // 线程 B
        let mut con = rdb2.get().await.unwrap();
        let mut balance: i32 = con.get("balance").await.unwrap();
        balance -= 30; // 扣除 30
        let _: () = con.set("balance", balance).await.unwrap(); // 设置余额为 70
        println!("Thread B finished, balance: {}", balance);
    });

    // 等待线程 A 和 B 完成
    let _ = tokio::join!(handle_a, handle_b);

    // 最终查看余额
    let balance: i32 = con.get("balance").await.unwrap();
    println!("Final balance: {}", balance);
}

async fn balance_v2() {
    let redis_cfg = Config::from_url("redis://127.0.0.1:6379");

    let rdb = redis_cfg
        .create_pool(Some(Runtime::Tokio1))
        .expect("Failed to create redis pool");

    let mut con = rdb.get().await.unwrap();

    let _: () = con.set("balance", 100).await.unwrap();

    // 创建线程 A 和 B
    let rdb1 = rdb.clone();
    let handle_a = tokio::spawn(async move {
        // 线程 A
        let script = r#"
            local balance = redis.call("GET", "balance")
            if balance then
                balance = tonumber(balance)
                balance = balance - 50
                redis.call("SET", "balance", balance)
            end
            return balance
        "#;
        let script = redis::Script::new(script);
        let result: i32 = script
            .invoke_async(&mut rdb1.get().await.unwrap())
            .await
            .unwrap();
        println!("Thread A finished, balance: {}", result);
    });

    let rdb2 = rdb.clone();
    let handle_b = tokio::spawn(async move {
        // 线程 B
        let script = r#"
            local balance = redis.call("GET", "balance")
            if balance then
                balance = tonumber(balance)
                balance = balance - 30
                redis.call("SET", "balance", balance)
            end
            return balance
        "#;
        let script = redis::Script::new(script);
        let result: i32 = script
            .invoke_async(&mut rdb2.get().await.unwrap())
            .await
            .unwrap();
        println!("Thread B finished, balance: {}", result);
    });

    // 等待线程 A 和 B 完成
    let _ = tokio::join!(handle_a, handle_b);

    // 最终查看余额
    let balance: i32 = con.get("balance").await.unwrap();
    println!("Final balance: {}", balance);
}
