use std::{time::Duration, vec};

use crate::core::{
    dns::{self, DnsResult, DoHServerConfig, TestDnsMethod},
    hosts,
};
use tokio::try_join;

#[tauri::command]
pub fn add_doh_servers(conf: Vec<&str>) {
    for record in conf.iter() {
        let server = DoHServerConfig::new(record.to_string());
        dns::DOH_SERVERS.insert(server.domain.clone(), server);
    }
    dns::init_doh_resolvers();
    println!("以下是可用的doh：");
    for entry in dns::DOH_SERVERS.iter() {
        println!("{}", entry.key());
    }
}

#[tauri::command]
pub fn add_hosts(conf: Vec<&str>) {
    for record in conf.iter() {
        let v: Vec<&str> = record.split(" ").collect();
        hosts::put(v[v.len() - 1].to_string(), v[0].to_string());
    }
    println!("以下是host：");
    for entry in hosts::HOSTS.iter() {
        println!("{} -> {}", entry.key(), entry.value());
    }
}

#[tauri::command]
pub fn get_all_hosts() -> Vec<String> {
    let mut result = vec![];
    for entry in hosts::HOSTS.iter() {
        result.push(format!("{} {}", entry.value(), entry.key()));
    }
    result
}

#[tauri::command]
pub fn get_all_doh_servers() -> Vec<String> {
    let mut result = vec![];
    for entry in dns::DOH_SERVERS.iter() {
        result.push(format!("{}", entry.key()));
    }
    result
}

#[tauri::command]
pub async fn pull_doh_servers_and_set_resolvers() {
    // set doh server
    dns::collect_valid_doh_servers(Duration::from_secs(2))
        .await
        .unwrap();
    // init resolvers
    dns::init_doh_resolvers();
    println!("以下是可用的doh：");
    for entry in dns::DOH_SERVERS.iter() {
        println!("{}", entry.key());
    }
}

#[tauri::command]
pub async fn query_dns_of_single_domain(
    domain: String,
    timeout: u64,
    test_type: &str,
) -> Result<Vec<DnsResult>, String> {
    let test_method = match test_type{
        "tcp" => TestDnsMethod::TCP,
        "tls" => TestDnsMethod::HTTPS,
        _ => TestDnsMethod::TCP,
    };
    let clone = domain.clone();
    let ips = dns::query_domain_for_valid_ips(
        &clone,
        test_method,
        false,
        Duration::from_secs(timeout),
    )
    .await;
    ips.map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn query_dns_and_set_host(domains: Vec<String>, timeout: u64) {
    // query for domain
    let mut handles = vec![];
    for domain in domains {
        let handle = tokio::spawn(async move {
            // println!("查询{} 开始...", domain);
            let clone = domain.clone();
            let ips = dns::query_domain_for_valid_ips(
                &clone,
                TestDnsMethod::HTTPS,
                false,
                Duration::from_secs(timeout),
            )
            .await
            .unwrap();
            if ips.len() > 0 {
                let mut best_result = &ips[0];
                let mut second_best_result = &ips[0];
                for result in ips.iter() {
                    if result.cost <= best_result.cost {
                        second_best_result = best_result;
                        best_result = result;
                    }
                }
                if hosts::get_addr(domain.clone()) == best_result.ip {
                    hosts::put(clone, second_best_result.ip.clone());
                } else {
                    hosts::put(clone, best_result.ip.clone());
                }
            }
            // println!("查询{} 结束...", domain);
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = try_join!(handle);
    }
    println!("以下是host：");
    for entry in hosts::HOSTS.iter() {
        println!("{} -> {}", entry.key(), entry.value());
    }
}
