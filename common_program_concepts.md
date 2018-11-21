## 变量和可变性
```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // 报错，cannot assign twice to immutable variable x
    // x = 6;
    // println!("The value of x is: {}", x);
}
```
使用mut关键字定义可变变量

PS：使用大型数据结构时，适当使用可变变量，可能比复制和返回新分配的实例更快。对于较小的数据结构，总是创建新实例，采用更偏向函数式的编程风格，可能会使代码更易理解，为可读性而牺牲性能或许是值得的
```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```
## 变量和常量的区别
- 不允许对常量使用 mut。声明常量使用 const 关键字而不是 let，并且 必须 注明值的类型。
- 常量可以在任何作用域中声明，包括全局作用域，这在一个值需要被很多部分的代码用到时很有用。
- 常量只能被设置为常量表达式，而不能是函数调用的结果，或任何其他只能在运行时计算出的值。例如：
```rust
// Rust 常量的命名规范是使用下划线分隔的大写字母
const MAX_POINTS: u32 = 100_000;
```
## 隐藏 shadowing
```rust
fn main() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
```
### 隐藏和mut的区别
- 对不可变变量重新赋值时，如果没有使用 let 关键字，会导致编译时错误。
- mut 与 隐藏 的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，但复用这个名字
例如：
```rust
// 第一个spaces是字符串类型
let spaces = "   ";
// 第二个是数字类型
let spaces = spaces.len();
// 如果我们使用 mut，会出现编译时错误，无法改变变量类型
// let mut spaces = "   ";
// spaces = spaces.len();
```
## 数据类型
Rust 是 静态类型语言，即在编译时就必须知道所有变量的类型。根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，必须增加类型注解，如下
```rust
let guess: u32 = "42".parse().expect("Not a number!");
```
### 标量类型
- 整型（默认i32）
isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。

长度 | 有符号 | 无符号
:-:|:-:|:-:
8-bit  | i8   | u8
16-bit | i16  | u16
32-bit | i32  | u32
64-bit | i64  | u64
arch | isize  | usize

数字字面值 | 例子
:-: | :-:
Decimal | 12_345
Hex | 0xff
Octal | 0o77
Binary | 0b1111_0000
Byte (u8 only) | b'A'

- 浮点型（f32 和 f64 , 默认 f64）
- 布尔型（true 和 false）
- 字符类型
Rust 的 char 类型代表了一个 Unicode 标量值，在 Rust 中，拼音字母，中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。

### 复合类型
- tuple 元组
```rust
fn main() {
    let tup = (500, 6.4, 1);
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // 解构
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    println!("The value of x is: {}", tup.0)
}
```
- 数组
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
## 函数
```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    // x + 1 是表达式，有返回值
    // x + 1; 是语句没有返回值
    x + 1
}
```
## 循环
```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```