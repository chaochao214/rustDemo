fn main() {




    partition()

}

// 所有权
fn StringDemo() {
    // let s= "hello"

    let mut s = String::from("hello");
    s.push_str(",world");
    println!("{}", s);

}

// 切片
fn partition(){
    let mut str:String= "acb".to_string();
    println!("{}", first_word(&str));

}
fn first_word(s: &String) -> usize {
     let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{}", item);
         if item == b' ' {
            return i;
        }
    }

    s.len();
}
