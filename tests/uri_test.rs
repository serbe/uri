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
    assert_eq!(http.host(), Some("api.ipify.org"));
    assert!(http.addr().is_ok());
}

#[test]
fn ipv4_addr() {
    let socks = "socks5://127.0.0.1:5959".parse::<Uri>().unwrap();
    assert_eq!(socks.host(), Some("127.0.0.1"));
    assert!(socks.addr().is_ok());
}

#[test]
fn get_host_str() {
    let http = "http://api.ipify.org".parse::<Uri>().unwrap();
    assert_eq!(http.host_str(), "api.ipify.org");
}

#[test]
fn get_host_with_port() {
    let http = "http://api.ipify.org".parse::<Uri>().unwrap();
    assert_eq!(http.host_with_port(), Some("api.ipify.org:80".to_string()));

    let http = "http://api.ipify.org:1245".parse::<Uri>().unwrap();
    assert_eq!(http.host_with_port(), Some("api.ipify.org:1245".to_string()));
}