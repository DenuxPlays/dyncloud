use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::RwLock;
use std::time::Instant;

pub(crate) struct IpCache {
    ttl: u64,
    inner: RwLock<IpCacheInner>,
}

impl IpCache {
    pub(crate) fn new(ttl: u64) -> Self {
        Self {
            ttl,
            inner: Default::default(),
        }
    }

    pub(crate) fn set_ipv4addr(&self, ipv4addr: Ipv4Addr) {
        #[allow(clippy::expect_used)]
        let mut inner = self.inner.write().expect("IP cache poisoned");
        inner.ipv4addr = Some(ipv4addr);
        inner.ipv4_last_update = Some(Instant::now());
    }

    pub(crate) fn set_ipv6addr(&self, ipv6addr: Ipv6Addr) {
        #[allow(clippy::expect_used)]
        let mut inner = self.inner.write().expect("IP cache poisoned");
        inner.ipv6addr = Some(ipv6addr);
        inner.ipv6_last_update = Some(Instant::now());
    }

    pub(crate) fn get_ipv4addr(&self) -> Option<Ipv4Addr> {
        #[allow(clippy::expect_used)]
        let inner = self.inner.read().expect("IP cache poisoned");
        match inner.ipv4_last_update {
            None => None,
            Some(time) => {
                if time.elapsed().as_secs() > self.ttl {
                    return None;
                }

                inner.ipv4addr
            }
        }
    }

    pub(crate) fn get_ipv6addr(&self) -> Option<Ipv6Addr> {
        #[allow(clippy::expect_used)]
        let inner = self.inner.read().expect("IP cache poisoned");
        match inner.ipv6_last_update {
            None => None,
            Some(time) => {
                if time.elapsed().as_secs() > self.ttl {
                    return None;
                }

                inner.ipv6addr
            }
        }
    }
}

#[derive(Default)]
struct IpCacheInner {
    ipv4_last_update: Option<Instant>,
    ipv4addr: Option<Ipv4Addr>,
    ipv6_last_update: Option<Instant>,
    ipv6addr: Option<Ipv6Addr>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_set_and_get_ipv4_address() {
        let cache = IpCache::new(60); // 60 seconds TTL
        let ipv4 = "192.168.1.1".parse::<Ipv4Addr>().unwrap();

        cache.set_ipv4addr(ipv4);
        assert_eq!(cache.get_ipv4addr(), Some(ipv4));
    }

    #[test]
    fn test_set_and_get_ipv6_address() {
        let cache = IpCache::new(60); // 60 seconds TTL
        let ipv6 = "2001:db8::1".parse::<Ipv6Addr>().unwrap();

        cache.set_ipv6addr(ipv6);
        assert_eq!(cache.get_ipv6addr(), Some(ipv6));
    }

    #[test]
    fn test_ipv4_returns_none_when_no_address_set() {
        let cache = IpCache::new(60);
        assert_eq!(cache.get_ipv4addr(), None);
    }

    #[test]
    fn test_ipv6_returns_none_when_no_address_set() {
        let cache = IpCache::new(60);
        assert_eq!(cache.get_ipv6addr(), None);
    }

    #[test]
    fn test_ipv4_expires_after_ttl() {
        let cache = IpCache::new(1); // 1 second TTL
        let ipv4 = "192.168.1.1".parse::<Ipv4Addr>().unwrap();

        cache.set_ipv4addr(ipv4);
        assert_eq!(cache.get_ipv4addr(), Some(ipv4));

        // Wait for TTL to expire
        thread::sleep(Duration::from_secs(2));
        assert_eq!(cache.get_ipv4addr(), None);
    }

    #[test]
    fn test_ipv6_expires_after_ttl() {
        let cache = IpCache::new(1); // 1 second TTL
        let ipv6 = "2001:db8::1".parse::<Ipv6Addr>().unwrap();

        cache.set_ipv6addr(ipv6);
        assert_eq!(cache.get_ipv6addr(), Some(ipv6));

        // Wait for TTL to expire
        thread::sleep(Duration::from_secs(2));
        assert_eq!(cache.get_ipv6addr(), None);
    }

    #[test]
    fn test_cache_returns_valid_address_within_ttl() {
        let cache = IpCache::new(5); // 5 seconds TTL
        let ipv4 = "10.0.0.1".parse::<Ipv4Addr>().unwrap();
        let ipv6 = "fe80::1".parse::<Ipv6Addr>().unwrap();

        cache.set_ipv4addr(ipv4);
        cache.set_ipv6addr(ipv6);

        // Check immediately
        assert_eq!(cache.get_ipv4addr(), Some(ipv4));
        assert_eq!(cache.get_ipv6addr(), Some(ipv6));

        // Wait less than TTL
        thread::sleep(Duration::from_millis(500));
        assert_eq!(cache.get_ipv4addr(), Some(ipv4));
        assert_eq!(cache.get_ipv6addr(), Some(ipv6));
    }

    #[test]
    fn test_updating_address_resets_ttl() {
        let cache = IpCache::new(2);
        let ipv4_old = "192.168.1.1".parse::<Ipv4Addr>().unwrap();
        let ipv4_new = "192.168.1.2".parse::<Ipv4Addr>().unwrap();

        cache.set_ipv4addr(ipv4_old);
        thread::sleep(Duration::from_secs(1));

        // Update with new address
        cache.set_ipv4addr(ipv4_new);

        // Wait another second (total 2 seconds from first set, but only 1 from update)
        thread::sleep(Duration::from_secs(1));

        // Should still be valid because TTL was reset on update
        assert_eq!(cache.get_ipv4addr(), Some(ipv4_new));
    }
}
