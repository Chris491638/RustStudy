extern crate collection_practice;
use collection_practice::num_vec_op;

fn main() {
    // 1.给定一系列数字，使用 vector 并返回这个列表的平均数、中位数和众数（出现次数最多的值）。
    let mut v: Vec<i32> = vec![-2,-3,-3,2];
    // 从小到大排序　
    // pub fn sort_unstable(&mut self) 
    v.sort_unstable();
    num_vec_op::info(&v)
    // 2.将字符串转换为 Pig Latin，辅音字母开头的单词移至单词尾加'ay'("first"=>"irst-fay");
    // 元音字母开头的单词在结尾增加'hay'("apple"=>"apple-hay")

    // 3.使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。
    // 例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。
    // 接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字母顺排序的列表。
}
