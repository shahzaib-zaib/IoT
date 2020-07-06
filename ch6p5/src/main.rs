fn main() {
    let msg = Message::Write(String::from("Hi how are you?"));

    let msg2 = Message::ChangeColor(10,20,30);

    msg.call();

    msg2.call();
}

#[derive(Debug)]
enum Message{
    Quit,
    Write(String),
    Move{x:i32, y: i32},
    ChangeColor(u32,u32,u32)
}

impl Message{
    fn call(&self){
        println!("{:?}",self);
    }
}