fn main() {
    let ip_address1 = IPAddress{
        Kind: IPAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    
    println!("{:#?}",ip_address1);

    let ip_address2 = IPAddress{
        kind: IPAddrKind::V6,
        address: String::from("::1")
    };

    println!("{:#?}",ip_address2);
}



#[derive(Debug)]
enum IPAddrKind{
    V4,
    V6
}

#[derive(Debug)]
struct IPAddress{
    kind: IPAddrKind,
    address: String
}
