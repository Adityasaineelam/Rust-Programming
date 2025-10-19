//Enum
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routing IPv4"),
        IpAddrKind::V6 => println!("Routing IPv6"),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("Loopback address: {}", loopback.address);
}