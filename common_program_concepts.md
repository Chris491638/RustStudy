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
