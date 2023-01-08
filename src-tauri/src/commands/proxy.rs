use std::net::ToSocketAddrs;

use crate::{
    core::{http_proxy},
    state,
};
use tauri::Manager;
use tokio::{io, net::TcpListener};

async fn run(addr: &str, port: u16) -> io::Result<()> {
    let addr = (addr, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    // 监听TCP连接
    let listener = TcpListener::bind(&addr).await?;
    http_proxy::set_bind_port(port);
    loop {
        if let Ok((stream, _peer_addr)) = listener.accept().await {
            tokio::spawn(async move {
                if let Err(_err) = http_proxy::handle(stream).await {
                    // eprintln!("{:?}", _err);
                }
            });
        }
    }
}
#[tauri::command]
pub fn start_proxy2(_port: u32) {
    // tauri::async_runtime::spawn(async move {
    //     let _ = run().await;
    // });
}

#[tauri::command]
pub async fn stop_proxy(window: tauri::Window) {
    // window.emit_all("stop-proxy", "-").unwrap();
    window.trigger_global("stop-proxy", None);
    println!("产生信号 stop-proxy");
}

#[tauri::command]
pub async fn start_proxy(addr: String, port: u16, window: tauri::Window) {
    if !state::proxy_enabled() {
        // hosts::put("nicelee.top".into(), "127.0.0.1".into());
        // hosts::put("www.baidu.com".into(), "127.0.0.1".into());
        // let ip = hosts::get_addr("www.baidu.com".into());
        // println!("key: {}, value: {}", "www.baidu.com", ip);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        let handle = runtime.handle().clone();
        window.once_global("stop-proxy", |_event| {
            println!("接收到 stop-proxy");
            state::enable_proxy(false);
            runtime.shutdown_background();
        });
        let task_run = handle.spawn(async move {
            run(&addr, port).await.unwrap();
            println!("stopped");
        });

        tauri::async_runtime::spawn(async move {
            let _ = task_run.await;
        });
        state::enable_proxy(true);
        println!("start_proxy end...");
    } else {
        println!("proxy 已开启...");
    }
}
