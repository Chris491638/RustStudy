## 定义并实例化结构体
```rust
// 定义结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// 实例化结构体
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
// 更改结构体值
let mut user2 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};
user2.email = String::from("anotheremail@example.com");
```
### 变量与字段同名时的字段初始化简写语法
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
### 使用结构体更新语法从其他实例创建实例
```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```
`..` 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。
```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```
### 元组结构体
```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// black 和 origin 值的类型不同，因为它们是不同的元组结构体的实例
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
### 通过派生 trait 增加结构体实用功能
```rust
// Rust 包含了打印出调试信息的功能，不过我们必须为结构体显式选择这个功能.
// 在结构体定义之前加上 #[derive(Debug)] 注解
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);
    // rect1 is Rectangle { width: 30, height: 50 }
    // 使用 {:#?} 可以格式化输出如下
    // rect1 is Rectangle {
    //     width: 30,
    //     height: 50
    // }
}
```
## 方法语法
### 定义方法
> 在 C/C++ 语言中，有两个不同的运算符来调用方法：`.` 直接在对象上调用方法，而 `->` 在一个对象的指针上调用方法，这时需要先解引用指针。换句话说，如果 object 是一个指针，那么 object->something() 就像 (*object).something() 一样。

> Rust 并没有一个与 `->` 等效的运算符；相反，Rust 有一个叫 自动引用和解引用的功能。当使用 object.something() 调用方法时，Rust 会自动为 object 添加 `&`、`&mut` 或 `*` 以便使 object 与方法签名匹配。也就是说，这些代码是等价的：
p1.distance(&p2);
(&p1).distance(&p2);

> 这种自动引用的行为之所以有效，是因为方法有一个明确的接收者 self 的类型。在给出接收者和方法名的前提下，Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块(implementation)
impl Rectangle {
    // 因为该方法位于 impl Rectangle 上下文中,所以 Rust 知道 self 的类型是 Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
### 带有更多参数的方法
```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

let rect1 = Rectangle { width: 30, height: 50 };
let rect2 = Rectangle { width: 10, height: 40 };

rect1.can_hold(&rect2)；
```
### 关联函数
impl 块的另一个有用的功能是：允许在 impl 块中定义不以 self 作为参数的函数，即 关联函数。因为它们与结构体相关联。它们仍是函数而不是方法，因为它们并不作用于一个结构体的实例。
```rust
// 关联函数经常被用作返回一个结构体新实例的构造函数
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

let sq = Rectangle::square(3);
```
### 多个 impl 块
```rust
// 不同 impl 块中方法名不可重复
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```