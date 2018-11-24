use std::collections::HashMap;
// 求 vec 平均值
pub fn avg(vec: &Vec<i32>)-> i32{
    let mut sum = 0;
    // len()　默认返回 usize 类型，使用　as 转换类型
    let len = vec.len() as i32;
    for i in vec {
        sum += *i;
    }
    sum / len
}
// 求 vec 中位数
pub fn median(vec: &Vec<i32>) -> i32{
    let len = vec.len();
    let index = len/2; 
    vec[index]
}
// 出现次数最多的数字
pub fn most(vec: &Vec<i32>) -> i32{
    let mut map = HashMap::new();
    let mut current_val: i32 = 0;
    let mut current_key: i32 = 0;
    for i in vec {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    
    for (key, val) in &map {
        if *val > current_val {
            current_key = **key;
            current_val = *val
        }
    }
    current_key
}

pub fn info(vec: &Vec<i32>) {
    println!("Avg : {}",avg(vec));
    println!("Median : {}",median(vec));
    println!("Most : {}",most(vec));
}