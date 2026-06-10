____________________________________________________________________________________________________________________________________________________________
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
