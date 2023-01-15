use crate::core::hosts;
use regex::Regex;
use tokio::io::{self, copy, split, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use super::dns;

lazy_static::lazy_static! {
    static ref REG_HEAD :Regex  = Regex::new(r"(CONNECT|Host:) ([^ :\r\n]+)(?::(\d+))?").unwrap();
    static ref BIND_PORT :u16  = 443;
}

pub fn set_bind_port(port: u16) {
    let p = std::ptr::addr_of!(*BIND_PORT) as *mut u16;
    unsafe {
        std::ptr::write(p, port);
    }
}

async fn _parse_head(head: &[u8]) -> io::Result<(String, String, bool)> {
    if let Some(sni) = crate::core::tls::get_sni(head) {
        let host = match hosts::HOSTS.get(&sni) {
            Some(binding) => Some(binding.to_string()),
            None => {
                // 因为是SNI Proxy, 那么本地使用的话肯定解析为 127.0.0.1, 会造成死循环
                let ips = dns::DEFAULT_RESOLVER.lookup_ip(sni).await?;
                ips.iter().next().map(|lookup| lookup.to_string())
            }
        };
        if let Some(host) = host {
            return Ok((host, BIND_PORT.to_string(), true));
        }
    }
    let head_str =
        std::str::from_utf8(head).map_err(|x| io::Error::new(io::ErrorKind::Interrupted, x))?;
    if let Some(caps) = REG_HEAD.captures(head_str) {
        let host = &caps[2];
        let host = hosts::get_addr(host.into());
        let port = caps.get(3).map_or("80", |m| m.as_str());
        if head_str.starts_with("CONNECT") {
            return Ok((host, port.to_string(), false));
        } else {
            return Ok((host, port.to_string(), true));
        }
    }
    Err(io::Error::new(io::ErrorKind::Interrupted, "invalid head"))
}

pub async fn handle(stream: TcpStream) -> io::Result<()> {
    let (mut local_reader, mut local_writer) = split(stream);
    // 读取头部
    let mut head = [0u8; 2048];
    let n = local_reader.read(&mut head[..]).await?;

    let result = _parse_head(&head[..n]).await;
    if let Err(err) = result {
        let _ = local_writer.shutdown().await;
        return Err(err);
    }
    let (host, port, send_head_to_server) = result.unwrap();
    println!("{} {}", host, port);
    let dst_addr = format!("{}:{}", host, port);
    // let dst1 = dst_addr.clone();
    // let dst2 = dst_addr.clone();
    // let remote_stream = TcpStream::connect(dst_addr).await?;
    match TcpStream::connect(dst_addr).await {
        Ok(remote_stream) => {
            let (mut remote_reader, mut remote_writer) = split(remote_stream);

            if !send_head_to_server {
                if let Err(err) = local_writer
                    .write_all("HTTP/1.1 200 Connection Established\r\n\r\n".as_bytes())
                    .await
                {
                    let _ = local_writer.shutdown().await;
                    let _ = remote_writer.shutdown().await;
                    return Err(err);
                }
            } else {
                if let Err(err) = remote_writer.write_all(&head[..n]).await {
                    let _ = local_writer.shutdown().await;
                    let _ = remote_writer.shutdown().await;
                    return Err(err);
                };
            }

            let client_to_server = async {
                let _ = copy(&mut local_reader, &mut remote_writer).await;
                // println!("{} server连接断开", dst1);
                let _ = remote_writer.shutdown().await;
                Ok(()) as io::Result<()>
            };

            let server_to_client = async {
                let _ = copy(&mut remote_reader, &mut local_writer).await;
                // println!("{} client连接断开", dst2);
                let _ = local_writer.shutdown().await;
                Ok(())
            };

            tokio::try_join!(client_to_server, server_to_client)?;
            Ok(())
        }
        Err(err) => {
            let _ = local_writer.shutdown().await;
            Err(err)
        }
    }
}
