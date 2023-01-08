// use dashmap::{self, DashMap};
// use tokio::sync::mpsc::{self, Receiver, Sender};

use tauri::async_runtime::Mutex;
use trust_dns_resolver::TokioAsyncResolver;

// use crate::core::dns;

lazy_static::lazy_static! {
    pub static ref PROXY_ENABLED: bool= false;
    pub static ref DNS_QUERY_DOMAINS: Mutex<Vec<String>> = Mutex::new(vec![]);
    pub static ref DNS_RESOLVERS: Mutex<Vec<TokioAsyncResolver>> = Mutex::new(vec![]);
}

// pub async fn set_dns_resolvers(doh_servers: Vec<String>) {
//     let mut vec = DNS_RESOLVERS.lock().await;
//     vec.clear();
//     for server_host in doh_servers {
//         // let resolver = dns::get_doh_resolver(&server_host).unwrap();
//         // vec.push(resolver);
//     }
// }

// pub async fn set_domains_to_query(mut domains: Vec<String>) {
//     let mut vec = DNS_QUERY_DOMAINS.lock().await;
//     vec.clear();
//     vec.append(&mut domains);
// }

pub fn enable_proxy(enable: bool) {
    let p = std::ptr::addr_of!(*PROXY_ENABLED) as *mut bool;
    unsafe {
        std::ptr::write(p, enable);
    }
}

pub fn proxy_enabled() -> bool {
    *PROXY_ENABLED
}
