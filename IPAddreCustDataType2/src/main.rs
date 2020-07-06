fn main() {
    
    let ip_address1 = IPAddress::V4(String::from("127.0.0.1"));
    let ip_address2 = IPAddress::V6(127,0,0,1);

    println!("{:?}", ip_address1);
    println!("{:?}", ip_address2);

}


#[derive(Debug)]
enum IPAddress{
    V4(String),
    V6(u32,u32,u32,u32),
}