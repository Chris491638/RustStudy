## 泛型
### 函数定义中的泛型
```rust
fn largest<T:PartialOrd+Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```
### 结构体中的泛型
```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```
### 枚举中的泛型
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
### 方法定义
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```
T 为 f32 类型的 Point 实例拥有一个方法 distance_from_origin，而其他 T 不是 f32 类型的 Point 实例则没有定义此方法
```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```
## trait
类似于其他语言中的接口
### 定义 trait
```rust
pub trait Summarizable {
    fn summary(&self) -> String;
}
```
### 为类实现trait
```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```
### 默认实现
```rust
pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Summarizable for NewsArticle {}
```
默认实现允许调用相同 trait 中的其他方法，哪怕这些方法没有默认实现。
```rust
pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}
```
### trait bounds
只有实现了某些 trait 才能调用特定方法
```rust
pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}
```
可以通过`+`来为泛型指定多个 trait bounds。
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    ...
}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    ...
}
```
### 使用 trait bound 有条件的实现方法
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
## 生命周期
`'a`代指的生命周期是`x`和`y`共有的那一部分，即两者中较小生命周期。
```rust
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
### 生命周期省略规则
- 每一个是引用的参数都有它自己的生命周期参数
- 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
- 如果方法有多个输入生命周期参数，不过其中之一因为方法的缘故为 &self 或 &mut self，那么 self 的生命周期被赋给所有输出生命周期参数

编译器会按照如上三条规则计算函数中参数的生命周期，如果不能计算出则报错。
### 静态生命周期
`'static`生命周期存活于整个程序期间。所有的字符串字面值都拥有 `'static`生命周期
```rust
fn main() {
    let string1 = "long string is long";
    let result;
    {
        let string2 = "xyz";
        result = longest(string1, string2);
    }
    println!("The longest string is {}", result);
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
