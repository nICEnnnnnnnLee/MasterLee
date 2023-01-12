// use std::collections::HashMap;

use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;

use crate::core::dns_stamp::Addr::Port;
use crate::core::dns_stamp::Addr::SocketAddr;
use dashmap::DashMap;
use dashmap::DashSet;
use serde::Serializer;
use serde::ser::SerializeMap;
use tokio::io::AsyncWriteExt;
use tokio::time::timeout;
use trust_dns_resolver::config::*;
use trust_dns_resolver::lookup_ip::LookupIp;
use trust_dns_resolver::TokioAsyncResolver;

use crate::core::dns_stamp::DnsStamp;

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
#[derive(Debug)]
pub struct DnsResult {
    pub ip: String,
    pub cost: u128,
}

impl DnsResult {
    pub fn new(ip: String, cost: u128) -> Self {
        DnsResult { ip, cost }
    }
}

impl serde::Serialize for DnsResult{
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_map(Some(2))?;
        s.serialize_entry("ip", &self.ip)?;
        s.serialize_entry("cost", &(*&self.cost as u32))?;
        s.end()
    }
}
impl std::fmt::Display for DnsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} - {}ms", self.ip, self.cost))
    }
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct DoHServerConfig {
    pub domain: String,
    pub ip: String,
    pub sni: Option<String>,
}

impl DoHServerConfig {
    pub fn new(domain: String) -> Self {
        let ip = domain.clone();
        let sni = Some(domain.clone());
        DoHServerConfig { domain, ip, sni }
    }
}
#[allow(dead_code)]
pub enum TestDnsMethod {
    None = 0,
    TCP = 1,
    HTTPS = 2,
}

lazy_static::lazy_static! {
    static ref REG_SDNS :regex::Regex  = regex::Regex::new(r"sdns://[0-9a-zA-Z_\-=]+").unwrap();
    pub static ref DOH_SERVERS: DashMap<String, DoHServerConfig> = DashMap::new();
    static ref DOH_SERVER_RESOLVERS: DashMap<String, TokioAsyncResolver> = DashMap::new();
    pub static ref DEFAULT_RESOLVER: TokioAsyncResolver = {
        let mut resolver_conf = ResolverConfig::new();
        let conf = NameServerConfig {
            socket_addr: ("114.114.114.114".clone(), 53u16)
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap(),
            protocol: Protocol::Udp,
            tls_dns_name: None,
            trust_nx_responses: true,
            tls_config: None,
            bind_addr: None,
        };
        resolver_conf.add_name_server(conf);
        let mut resolver_opts = ResolverOpts::default();
        resolver_opts.use_hosts_file = false;
        resolver_opts.timeout = tokio::time::Duration::from_secs(10);

        let resolver = TokioAsyncResolver::tokio(resolver_conf, resolver_opts).unwrap();
        resolver
    };
}

impl std::fmt::Debug for DOH_SERVERS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DOH_SERVERS")
            .field("__private_field", &self.__private_field)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn query_valid_result() {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async {
            // set doh server
            // DOH_SERVERS.insert(
            //     "doh.pub".to_owned(),
            //     DoHServerConfig::new("doh.pub".to_owned()),
            // );
            // DOH_SERVERS.insert(
            //     "1.1.1.1".to_owned(),
            //     DoHServerConfig {
            //         domain: "1.1.1.1".to_owned(),
            //         ip: "1.1.1.1".to_owned(),
            //         sni: None,
            //     },
            // );
            collect_valid_doh_servers(Duration::from_secs(2))
                .await
                .unwrap();
            // init resolvers
            init_doh_resolvers();
            // query for domain
            let ips = query_domain_for_valid_ips(
                &"github.com",
                TestDnsMethod::HTTPS,
                false,
                Duration::from_secs(2),
            );
            let ips = ips.await.unwrap();
            println!("{:#?}", ips);
            // assert_eq!(4, add_two(2));
        });
    }
}
// fn init_default_resolver() {
//     let mut resolver_conf = ResolverConfig::new();
//     let conf = NameServerConfig {
//         socket_addr: ("114.114.114.114".clone(), 53u16)
//             .to_socket_addrs()
//             .unwrap()
//             .next()
//             .unwrap(),
//         protocol: Protocol::Udp,
//         tls_dns_name: None,
//         trust_nx_responses: true,
//         tls_config: None,
//         bind_addr: None,
//     };
//     resolver_conf.add_name_server(conf);
//     let mut resolver_opts = ResolverOpts::default();
//     resolver_opts.use_hosts_file = false;
//     resolver_opts.timeout = tokio::time::Duration::from_secs(10);

//     let resolver = TokioAsyncResolver::tokio(resolver_conf, resolver_opts).unwrap();
//     DEFAULT_RESOLVER.push(resolver);
// }
async fn get_valid_doh_servers_from_collections(
    servers_to_test: Vec<DoHServerConfig>,
    test_timeout: tokio::time::Duration,
) -> Result<Vec<DoHServerConfig>> {
    let mut result = vec![];
    let mut handles = vec![];
    for server in servers_to_test {
        // let server = server.clone();
        let handle = tokio::spawn(async move {
            match get_doh_resolver(&server) {
                Ok(resolver) => {
                    let test_func = async {
                        let r = resolver.lookup_ip("github.com").await.unwrap();
                        let r = r.iter().next().unwrap();
                        tokio::net::TcpStream::connect((r.to_string(), 443)).await
                        // crate::core::tls::connect(&r.to_string(), 443u16, "github.com", false).await
                    };
                    match timeout(test_timeout, test_func).await {
                        Ok(Ok(mut stream)) => {
                            // Ok(Ok((_r, mut stream))) => {
                            let _ = stream.shutdown().await;
                            // r.insert(server.clone());
                            Some(server)
                        }
                        Ok(Err(_err)) => None,
                        Err(_err) => None,
                    }
                }
                Err(_err) => None,
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        if let Ok(Some(server)) = handle.await {
            result.push(server);
        }
    }
    Ok(result)
}

async fn get_doh_servers_from_dnscrypt() -> Result<Vec<DoHServerConfig>> {
    let url = "https://download.dnscrypt.info/dnscrypt-resolvers/v3/public-resolvers.md";
    let resp = reqwest::get(url).await?.text().await?;
    // println!("{}", resp);
    let mut result = vec![];
    for caps in REG_SDNS.captures_iter(&resp) {
        let stamp = &caps[0];
        let dns_stamp = DnsStamp::decode(stamp)?;
        if let DnsStamp::DnsOverHttps(doh) = dns_stamp {
            // println!("{:#?}", doh);
            if "/dns-query".to_owned() == doh.path {
                let is_valid = match doh.addr {
                    Some(Port(443u16)) => true,
                    None => true,
                    Some(SocketAddr(sock)) => sock.port() == 443u16,
                    _ => false,
                };
                if is_valid {
                    let addr = doh.hostname;
                    if !addr.contains(":") {
                        result.push(DoHServerConfig::new(addr));
                    } else if addr.ends_with(":443") {
                        let addr = addr[0..addr.len() - 4].to_string();
                        result.push(DoHServerConfig::new(addr));
                    }
                }
            }
        }
    }
    // println!("{:#?}", result);
    Ok(result)
}

pub async fn collect_valid_doh_servers(test_timeout: Duration) -> Result<()> {
    let servers = get_doh_servers_from_dnscrypt().await?;
    // println!("all: {:?}\n-----------------\n\n", servers);
    let servers = get_valid_doh_servers_from_collections(servers, test_timeout).await?;
    // println!("valid: {:?}\n-----------------\n\n", servers);
    for server in servers {
        DOH_SERVERS.insert(server.domain.clone(), server);
    }
    Ok(())
}

pub fn init_doh_resolvers() {
    for entry in DOH_SERVERS.iter() {
        let config = entry.value();
        match get_doh_resolver(config) {
            Ok(resolver) => {
                DOH_SERVER_RESOLVERS.insert(config.domain.clone(), resolver);
            }
            Err(_) => {
                println!("doh: {} 无效", config.domain);
            }
        };
    }
}

pub async fn query_domain_for_valid_ips(
    domain: &str,
    method: TestDnsMethod,
    only_find_first: bool,
    conn_timeout: Duration,
) -> Result<Vec<DnsResult>> {
    let all_ips = query_domain_for_all_ips(domain).await?;
    let mut result: Vec<DnsResult> = vec![];
    for ip in all_ips.iter() {
        let begin = tokio::time::Instant::now();
        let ip: String = ip.to_string();
        match method {
            TestDnsMethod::None => {}
            TestDnsMethod::TCP => {
                match timeout(
                    conn_timeout,
                    tokio::net::TcpStream::connect((ip.clone(), 443)),
                )
                .await
                {
                    Ok(Ok(mut stream)) => {
                        let end = tokio::time::Instant::now();
                        let cost = end.duration_since(begin);
                        let cost = cost.as_millis();
                        // println!("{} 耗时： {}ms", ip, cost);
                        let _ = stream.shutdown().await;
                        result.push(DnsResult::new(ip, cost));
                        if only_find_first {
                            break;
                        }
                    }
                    _ => eprintln!("TCP 连接 {} 失败", ip),
                }
            }
            TestDnsMethod::HTTPS => {
                match timeout(
                    conn_timeout,
                    crate::core::tls::connect(&ip.clone(), 443u16, &domain.clone(), false),
                )
                .await
                {
                    Ok(Ok((_r, mut w))) => {
                        let end = tokio::time::Instant::now();
                        let cost = end.duration_since(begin).as_millis();
                        // println!("{} {} 耗时： {}ms", domain, ip, cost);
                        let _ = w.shutdown().await;
                        result.push(DnsResult::new(ip, cost));
                        if only_find_first {
                            break;
                        }
                    }
                    Ok(Err(_err)) => {
                        // eprintln!("HTTPS 连接 {} 失败, 失败原因： {}", ip, err)
                    }
                    Err(_) => {
                        // eprintln!("HTTPS 连接 {} 失败, 失败原因： 超时", ip)
                    }
                }
            }
        };
    }
    Ok(result)
}

pub async fn query_domain_for_all_ips(domain: &str) -> Result<Arc<DashSet<String>>> {
    let result: Arc<DashSet<String>> = Arc::new(DashSet::new());

    let mut handles = vec![];
    for resolver in DOH_SERVER_RESOLVERS.iter() {
        // let doh_identity = resolver.key().clone();
        let resolver_clone = resolver.clone();
        let result_clone = result.clone();
        let domain = domain.to_string();
        let handle = tokio::spawn(async move {
            match timeout(
                tokio::time::Duration::from_secs(4),
                lookup_ip(&domain, resolver_clone, result_clone),
            )
            .await
            {
                // Ok(Ok(_ips)) => {println!("{}: {} -> {:?}", server_host, domain, ips)},
                Ok(Err(_err)) => {
                    // eprintln!("{}: 查询 {} 失败, 失败原因： {}", doh_identity, domain, err)
                }
                Err(_) => {
                    // eprintln!("{}: 查询 {} 失败, 失败原因： 超时", doh_identity, domain)
                }
                _ => {}
            }
        });
        handles.push(handle);
    }

    for handle in handles.iter_mut() {
        let _ = tokio::try_join!(handle);
    }
    Ok(result.into())
}

async fn lookup_ip(
    domain: &str,
    resolver: TokioAsyncResolver,
    result: Arc<DashSet<String>>,
) -> Result<LookupIp> {
    let ips = resolver.lookup_ip(domain).await?;
    for ip in ips.iter() {
        result.insert(ip.to_string());
    }
    Ok(ips)
}

fn get_doh_resolver(
    // server_host: &str,
    config: &DoHServerConfig,
) -> core::result::Result<TokioAsyncResolver, trust_dns_resolver::error::ResolveError> {
    let mut resolver_conf = ResolverConfig::new();
    let conf = NameServerConfig {
        // socket_addr: std::net::SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(1, 1, 1, 1)), 443),
        socket_addr: (config.ip.clone(), 443u16)
            .to_socket_addrs()?
            .next()
            .unwrap(),
        protocol: Protocol::Https,
        tls_dns_name: config.sni.clone(),
        trust_nx_responses: true,
        tls_config: None,
        bind_addr: None,
    };
    resolver_conf.add_name_server(conf);
    let mut resolver_opts = ResolverOpts::default();
    resolver_opts.use_hosts_file = false;
    resolver_opts.timeout = tokio::time::Duration::from_secs(10);
    TokioAsyncResolver::tokio(resolver_conf, resolver_opts)
    // TokioAsyncResolver::tokio(ResolverConfig::cloudflare_https(), ResolverOpts::default())
    //         .unwrap();
}
