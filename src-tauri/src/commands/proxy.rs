use std::net::ToSocketAddrs;

use crate::{core::http_proxy, state};
use tauri::Manager;
use tokio::{io, net::TcpListener};

async fn bind_server(addr: &str, port: u16) -> io::Result<TcpListener> {
    let addr = (addr, port)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

    // 监听TCP连接
    let listener = TcpListener::bind(&addr).await?;
    http_proxy::set_bind_port(port);
    Ok(listener)
}

async fn run(listener: TcpListener) -> io::Result<()> {
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
// async fn run(addr: &str, port: u16) -> io::Result<()> {
//     let addr = (addr, port)
//         .to_socket_addrs()?
//         .next()
//         .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;

//     // 监听TCP连接
//     let listener = TcpListener::bind(&addr).await?;
//     http_proxy::set_bind_port(port);
//     loop {
//         if let Ok((stream, _peer_addr)) = listener.accept().await {
//             tokio::spawn(async move {
//                 if let Err(_err) = http_proxy::handle(stream).await {
//                     // eprintln!("{:?}", _err);
//                 }
//             });
//         }
//     }
// }
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
        let window_clone = window.clone();
        window.once_global("stop-proxy", |_event| {
            println!("接收到 stop-proxy");
            runtime.shutdown_background();
            state::enable_proxy(false);
        });
        let task_run = handle.spawn(async move {
            if let Ok(listener) = bind_server(&addr, port).await {
                window_clone.emit("proxy-start", true).unwrap();
                state::enable_proxy(true);
                run(listener).await.unwrap();
                println!("stopped");
            } else {
                state::enable_proxy(false);
                window_clone.emit("proxy-start", false).unwrap();
            }
        });

        tauri::async_runtime::spawn(async move {
            let _ = task_run.await;
        });
        
        println!("start_proxy end...");
    } else {
        window.emit("proxy-start", true).unwrap();
        println!("proxy 已开启...");
    }
}
