____________________________________________________________________________________________________________________________________________________________
Zero-Sized Type (ZST)
#[derive(Debug)]
struct Nothing ;
#[derive(Debug)]
struct User{

    data_type : Nothing ,
}
fn main(){
    let data_1 : Nothing = Nothing ;
    println!("Data 1 is {:?}" , data_1) ;
    let user_1 : User = User{

        data_type : data_1 ,
    } ;
    println!("User 1 is {:?}" ,user_1) ;
    println!("User 1 is {:?}" ,user_1.data_type) ;
}
_____________________________________________________________________________________________________________________________________________________________
#[derive(Debug)]
struct Message(String) ; 
fn main(){
    let msg : Message = Message(String::from("Hello world")) ; 
    println!("Message is {:?}" , msg.0) ;     
}
___________________________________________________________________________________________________________________________________________________________
#[derive(Debug)]
struct Message(String) ; 
impl Message{
    fn print_message(data : &Message){
        println!("Message is {:?}" , data.0) ;
    }
}
fn main(){
    let msg : Message = Message(String::from("Sohee Al Mahdi Dibbo !")) ; 
    Message::print_message(&msg) ;
}
__________________________________________________________________________________________________________________________________________________________
