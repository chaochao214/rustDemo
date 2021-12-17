use std::thread::sleep;

fn main() {

    // calcuteAnkle();
    // printRectangle();
    // 方法
    printWithMethod()
}

//方法
impl Rectangle {
    fn area(&self) -> u32{
       self.width * self.height
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width > other.width  && self.height > other.height
    }
}

fn printWithMethod(){
    let rec1= Rectangle{
         width:30,
        height:20
    };
    println!("innerMethod _>>>>>  {}", rec1.area());
}



fn printRectangle(){
    let rec1=  Rectangle{
        width:30,
        height:20
    };
    println!("pr");
    println!("{:?}", rec1);
}
// Rust确实包含了打印调试信息的功能，但我们必须为自己的结构体显式地选择这一功能。为了完成该声明，我们在结构体定义前添加了#[derive(Debug)]注解
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


fn build_user(param1: String, param2: String) ->User{
    User{
        username: param2,
        email: param1,
        sign_in_count: 0,
        active: false
    }
}

// 计算长方形的面积
fn calcuteAnkle(){
    println!("twcccc");
    println!("twccc-> {}", area(30, 50));

}
fn area(width:u32,height:u32)->u32{
    println!("area");
        width * height
}





fn structDemo(){
     let mut user1 = User{
         username: "twc".to_string(),
         email: "1@q.com".to_string(),
         sign_in_count: 0,
         active: false
     };

     user1.email =  String::from("twc@qq");
    
}
 


//结构体定义
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


