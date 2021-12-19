fn vecDemo() {
    let mut v2: Vec<i32> = Vec::new();
    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);


    let mut v = vec![1, 2, 3];
    v.push(1);

    let t = &v2[2];


}

//数组中获取元素
fn getV(){
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //索引获取
    let third:&i32 = &v[2];

    //get获取
    match v.get(2) {
        Some(third)=>println!( "the third element is {}",third),
        None =>println!("there is no third element"),
    }

}
// 遍历数组
fn ite(){
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

}

