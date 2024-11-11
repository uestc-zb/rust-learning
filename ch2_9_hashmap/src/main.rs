use std::collections::HashMap;

fn main() {

// 创建一个HashMap，用于存储宝石种类和对应的数量
    let mut my_gems = HashMap::new();

    // 将宝石类型和对应的数量写入表中
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    println!("Hello, world!");
}
