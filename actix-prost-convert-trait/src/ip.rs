use crate::{impl_try_convert_from_string, impl_try_convert_to_string, TryConvert};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

impl_try_convert_from_string!(IpAddr);
impl_try_convert_to_string!(IpAddr);

impl_try_convert_from_string!(Ipv4Addr);
impl_try_convert_to_string!(Ipv4Addr);

impl_try_convert_from_string!(Ipv6Addr);
impl_try_convert_to_string!(Ipv6Addr);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_ip() {
        let ip = IpAddr::try_convert("127.0.0.1".to_string()).unwrap();
        assert_eq!(ip, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        let ip = IpAddr::try_convert("::1".to_string()).unwrap();
        assert_eq!(ip, IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));

        let ip = Ipv4Addr::try_convert("1.1.1.1".to_string()).unwrap();
        assert_eq!(ip, Ipv4Addr::new(1, 1, 1, 1));

        let ip = Ipv6Addr::try_convert("1::1".to_string()).unwrap();
        assert_eq!(ip, Ipv6Addr::new(1, 0, 0, 0, 0, 0, 0, 1));
    }
}
