pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// 餐厅前厅，用于吃饭
mod front_of_house;
pub use front_of_house::hosting;
// pub fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();

//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order() {}

// // 厨房模块
// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}