use dashmap::{self, DashMap};
// #[derive(Debug)]
pub struct Addr(String);

impl Clone for Addr {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl std::fmt::Display for Addr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

lazy_static::lazy_static! {
    pub static ref HOSTS: DashMap<String, Addr> = DashMap::new();
}

impl std::fmt::Debug for HOSTS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HOSTS").field("__private_field", &self.__private_field).finish()
    }
}
// pub fn clear() {
//     HOSTS.clear();
// }

// pub fn remove(domain: String) {
//     HOSTS.remove(&domain);
// }

// pub fn get(domain: String) -> Option<Addr> {
//     HOSTS
//     .get(&domain)
//     .and_then(|binding| Some(binding.value().clone()))
// }

pub fn put(domain: String, ip: String) {
    HOSTS.insert(domain, Addr(ip));
}
// 如果能查询到ip, 返回 ip; 如果不能, 则返回原域名
pub fn get_addr(domain: String) -> String {
    match HOSTS.get(&domain) {
        Some(binding) => binding.value().0.clone(),
        None => domain,
    }
}
