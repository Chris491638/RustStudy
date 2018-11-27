extern crate collection_practice;
use collection_practice::num_vec_op;
use collection_practice::latin_op;

fn main() {
    // 1.给定一系列数字，使用 vector 并返回这个列表的平均数、中位数和众数（出现次数最多的值）。
    let mut v: Vec<i32> = vec![-2,-3,-3,2];
    // 从小到大排序　
    // pub fn sort_unstable(&mut self) 
    v.sort_unstable();
    num_vec_op::info(&v);
    // 2.将字符串转换为 Pig Latin，辅音字母开头的单词移至单词尾加'ay'("first"=>"irst-fay");
    // 元音字母开头的单词在结尾增加'hay'("apple"=>"apple-hay") a e i o u
    latin_op::info(&String::from("frid app up down earth own"));
}
