use std::time::Instant;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use rand::Rng; // 引入随机数生成

const TOTAL_REQUESTS: usize = 100_000;
const CONCURRENCY: usize = 50;
const DATA_COUNT: usize = 10_000; // 准备数据的数量
const KEY: &str = "beijing_pts";

/// 构造 RESP 协议的辅助函数
fn format_cmd(args: &[&str]) -> String {
    let mut cmd = String::new();
    cmd.push_str(&format!("*{}\r\n", args.len()));
    for arg in args {
        cmd.push_str(&format!("${}\r\n{}\r\n", arg.len(), arg));
    }
    cmd
}

async fn prepare_data() {
    println!("🛠Preparing {} items of data...", DATA_COUNT);
    let mut stream = TcpStream::connect("127.0.0.1:6377").await.unwrap();
    let mut rng = rand::thread_rng();
    let mut buffer = [0u8; 1024];

    for i in 0..DATA_COUNT {
        // 生成北京范围内的随机坐标
        let lon = 116.0 + rng.gen::<f64>(); // 116.0 ~ 117.0
        let lat = 39.0 + rng.gen::<f64>();  // 39.0 ~ 40.0
        let member = format!("user_{}", i);
        let lon_str = lon.to_string();
        let lat_str = lat.to_string();

        // 构造 GEOADD 命令: GEOADD key lon lat member
        let cmd = format_cmd(&["GEOADD", KEY, &lon_str, &lat_str, &member]);

        stream.write_all(cmd.as_bytes()).await.unwrap();
        // 读取响应，防止 TCP 缓冲区阻塞，但不打印
        let _ = stream.read(&mut buffer).await.unwrap();

        if (i + 1) % 1000 == 0 {
            print!(".");
        }
    }
    println!("\n✅ Data Loaded!");
}

#[tokio::main]
async fn main() {
    // 1. 先准备数据
    prepare_data().await;

    println!("🚀 Starting Stress Test on Rudis Geo...");
    let start = Instant::now();
    let mut handles = vec![];

    // 2. 开始并发压测
    for _ in 0..CONCURRENCY {
        handles.push(tokio::spawn(async move {
            let mut stream = TcpStream::connect("127.0.0.1:6377").await.unwrap();
            let mut buffer = [0u8; 1024];
            let requests_per_thread = TOTAL_REQUESTS / CONCURRENCY;

            // 预先构造好查询命令 (避免压测循环里字符串分配的开销干扰测试结果)
            // 查询 116.40, 39.90 附近 5km
            let cmd = format_cmd(&["GEORADIUS", KEY, "116.40", "39.90", "5", "km"]);
            let cmd_bytes = cmd.as_bytes();

            for _ in 0..requests_per_thread {
                stream.write_all(cmd_bytes).await.unwrap();
                let _ = stream.read(&mut buffer).await.unwrap();
            }
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    let duration = start.elapsed();
    println!("✅ Done!");
    println!("Time: {:?}", duration);
    println!("QPS: {:.2}", TOTAL_REQUESTS as f64 / duration.as_secs_f64());
}