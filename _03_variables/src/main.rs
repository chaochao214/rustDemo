use std::{thread, time};

fn main() {
 // scalarDemo()

     // dataType()

    forLoop()


}
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn printNum(){
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    another_function(5)
}

fn scalarDemo(){
    // 未指定类型则报错   ^^^^^ consider giving `guess` a type
    //  let guess = "42".parse().expect("Not a number");

    // u32 指定常量的大小   Constants for the 32-bit unsigned integer type.
     let guess:u32 = "42".parse().expect("Not a number");
}

// 数据类型
fn dataType(){
    let emoji = '😀';

    println!("{}", emoji);

}
//元组类型
fn  tup(){
    let a:(i32,f64,u8) = (500, 6.4,1);
    let (x, y, z): (i32, f64, u8) = a;
    println!("The value of y is: {}", y);

}

// for
 fn forLoop(){
   for  number in (1..100).rev(){
       println!("延迟输出计数");
      thread::sleep(time::Duration::from_secs(3));
       println!("{}", number);
   }

}




