use crate::error::Result;
use crate::uri::Uri;

// mod addr;
mod authority;
mod error;
mod range;
mod uri;
mod utils;

// fn get_range(s: &str, ch: char) {
//     println!("{:?}", s.contains(ch));
//     println!("{:?}", s.find(ch));
// }

// fn get_path_start(s: &str) -> Option<usize> {
//    s.find('/')
// }

// fn check_host(s: &str) -> Result<()> {
//     Ok(())
// }

fn main() -> Result<()> {
    let s = "aSd://webmaster:13z@www.example.org:234/sdfsdf/asd?asdew=12;asdwd=234&qwrc=1#skdjfnkjsdhfjskdf";
    let uri = s.parse::<Uri>()?;
    assert_eq!(uri.scheme(), "asd");
    println!("{:?}", uri);

    let authority = "www.example.org".parse::<authority::Authority>().unwrap();
    println!("{:?}", authority);
    assert_eq!(authority.host(), "www.example.org");

    Ok(())
}