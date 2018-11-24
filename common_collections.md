## Vector
vector 允许我们在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。vector 只能储存相同类型的值.
### 新建vector
```rust
// 创建一个新的空 vector
let v: Vec<i32> = Vec::new();
// 新建一个拥有值 1、2 和 3 的 Vec<i32>
let v = vec![1, 2, 3];
```
### 更新vector
```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```
###　读取 vector 的元素
```rust
let v = vec![1, 2, 3, 4, 5];
// 方式一，通过索引
let third: &i32 = &v[2];
// 方式二，通过get
let third: Option<&i32> = v.get(2);
// 两者的区别在于，访问不存在的索引时，使用[]会报错，使用get会返回None
```
#### 无效引用
```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];
// 在相同作用域中不能同时出现可变和不可变引用
// 在 vector 的结尾增加新元素时，在没有足够空间将所有所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存,借用规则阻止程序陷入这种状况.
// v.push(6);
```
### 遍历 vector 中的元素
```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```
### 使用枚举来储存多种类型
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
## hash map
```rust
use std::collections::HashMap;
// 新建空的hash
let mut scores = HashMap::new();
// 插入键值对
scores.insert(String::from("Blue"), 10);
// 读取键值
let team_name = String::from("Blue");
let score = scores.get(&team_name);
// 遍历hash
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
//更新hash
// 1.覆盖一个值
scores.insert(String::from("Blue"), 25);
println!("{:?}", scores);  // {"Blue": 25}
// 2.只在键没有对应值时插入
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);
println!("{:?}", scores);  //{"Yellow": 50, "Blue": 10}
```
下面例子，通过哈希 map 储存单词和计数来统计出现次数。or_insert 方法会返回这个键的值的一个可变引用（&mut V），我们将可变引用储存在 count 变量中，然后使用星号（*）解引用 count　获取变量的值。
```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
println!("{:?}", map);  
// {"world": 2, "hello": 1, "wonderful": 1}
```