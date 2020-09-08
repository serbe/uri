use uri::Authority;

#[test]
fn authority_user() {
    let authority = "user:password@host:80".parse::<Authority>().unwrap();
    assert_eq!(authority.username(), Some("user"));
}

#[test]
fn authority_password() {
    let authority = "user:password@host:80".parse::<Authority>().unwrap();
    assert_eq!(authority.password(), Some("password"));
}

#[test]
fn authority_host() {
    let authority = "user:password@host:80".parse::<Authority>().unwrap();
    assert_eq!(authority.host(), "host");
}

#[test]
fn authority_port() {
    let authority = "user:password@host:80".parse::<Authority>().unwrap();
    assert_eq!(authority.port(), Some(80));
}

#[test]
fn authority_userinfo() {
    let authority = "user:password@host:80".parse::<Authority>().unwrap();
    assert_eq!(authority.user_info(), Some("user:password"));
}

#[test]
fn authority_display() {
    let authority = "user:password@host:80".parse::<Authority>().unwrap();
    assert_eq!(&authority.to_string(), "user:********@host:80");
}
