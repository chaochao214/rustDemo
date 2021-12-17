#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



pub mod front_of_house {
     pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant(){
     //绝对路径
     crate::front_of_house::hosting::add_to_waitlist();

    //相对路径
     front_of_house::hosting::add_to_waitlist();

}



//示例 7.9  一个拥有部分公共字段 , 部分私有字段的结构体



// 7-12 use 引入作用域
 use self::front_of_house::hosting;
pub fn eat_pub(){
     hosting::add_to_waitlist();
     hosting::add_to_waitlist();

}

