fn main() {
    
    let four = IPAddrkind::V4;
    let six = IPAddrkind::V6;

    route(four);
    route(six);
}

enum IPAddrkind{
    V4,
    V6
}

fn route(x: IPAddrkind){

}