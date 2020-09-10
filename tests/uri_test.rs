use uri::Uri;

#[test]
fn parse_ipv4() {
    let socks = "socks5://127.0.0.1:5959";
    let http = "http://api.ipify.org";
    assert!(socks.parse::<Uri>().is_ok());
    assert!(http.parse::<Uri>().is_ok());
}

#[test]
fn domain_addr() {
    let http = "http://api.ipify.org".parse::<Uri>().unwrap();
    assert_eq!(http.host_port(), Some("api.ipify.org:80".to_string()));
}

#[test]
fn ipv4_addr() {
    let socks = "socks5://127.0.0.1:5959".parse::<Uri>().unwrap();
    assert_eq!(socks.host_port(), Some("127.0.0.1:5959".to_string()));
}
