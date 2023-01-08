
use std::time::Duration;

use crate::{
    core::{dns::{self, TestDnsMethod}, hosts},
};
use tokio::try_join;

#[tauri::command]
pub async fn pull_doh_servers_and_set_resolvers() {
    // set doh server
    dns::collect_valid_doh_servers(Duration::from_secs(2))
                .await
                .unwrap();
    // init resolvers
    dns::init_doh_resolvers();
    println!("以下是可用的doh：");
    for entry in dns::DOH_SERVERS.iter(){
        println!("{}", entry.key());
    }
}

#[tauri::command]
pub async fn query_dns_and_set_host(domains: Vec<String>) {
    // query for domain
    let mut handles = vec![];
    for domain in domains{
        let handle = tokio::spawn(async move{
            // println!("查询{} 开始...", domain);
            let clone = domain.clone();
            let ips = dns::query_domain_for_valid_ips(
                &clone,
                TestDnsMethod::HTTPS,
                false,
                Duration::from_secs(2),
            ).await.unwrap();
            if ips.len() > 0{
                let mut best_result = &ips[0];
                for result in ips.iter(){
                    if result.cost < best_result.cost{
                        best_result = result;
                    }
                }
                hosts::put(clone, best_result.ip.clone());
            }
            // println!("查询{} 结束...", domain);
        });
        handles.push(handle);
    }
    for handle in handles{
        let _ = try_join!(handle);
    }
    println!("以下是host：");
    for entry in hosts::HOSTS.iter(){
        println!("{} -> {}", entry.key(), entry.value());
    }
}

