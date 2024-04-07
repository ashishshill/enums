enum IpAddrKind {
    v4(String),
    v6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let four = IpAddrKind::v4;
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("Hello, world! {}", sum);
}

// Module system
// main consept
// start with packages
// module allow to control securty
//
