## 什么是所有权
所有权 是 Rust 最独特的功能，其令 Rust 无需垃圾回收即可保障内存安全。
### 所有权规则
- Rust 中的每一个值都有一个被称为其 所有者 的变量
- 值有且只有一个所有者
- 当所有者（变量）离开作用域，这个值将被丢弃

### 变量与数据交互的方式（一）：移动
```rust
let x = 5;
let y = x;
```
将 5 绑定到 x，然后生成 x 的值拷贝并绑定到 y，由于整数是具有已知大小的简单值，所以这两个5被放入栈中。

> Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上。如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用。Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误。

> 那么什么类型是 Copy 的呢？可以查看给定类型的文档来确认，不过作为一个通用的规则，任何简单标量值的组合可以是 Copy 的，不需要分配内存或某种形式资源的类型是 Copy 的。如下是一些 Copy 的类型：
所有整数类型，比如 u32。
布尔类型，bool，它的值是 true 和 false。
所有浮点数类型，比如 f64。
字符类型，char。
元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。

```rust
let s1 = String::from("hello");
let s2 = s1;
```
下图为内存存储结构图，s2从栈上拷贝了s1的指针、长度和容量,但并没有复制s1指针指向的堆上数据。
![](views/1.svg)

由于变量离开作用域后，Rust 自动调用 drop 函数并清理变量的堆内存，但 s1 和 s2 数据指针指向了同一位置,那么当 s1 和 s2 离开作用域时，他们都会尝试释放相同的内存，这是一个叫做 二次释放 的错误，也是之前提到过的内存安全性 bug 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。

为了确保内存安全， Rust 会失效 s1，因此 Rust 不需要在 s1 离开作用域后清理任何东西。s1 无效后的内存表现如下
![](views/2.svg)

### 变量与数据交互的方式（二）：克隆
```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```
![](views/3.svg)

### 所有权和函数
```rust
fn main() {
    let s = String::from("hello");// s 进入作用域
    takes_ownership(s);           // s 的值移动到函数里
                                  // s 到这里不再有效
    let x = 5;                    // x 进入作用域
    makes_copy(x);                // x 移动到函数里，但 i32 是 Copy 的，所以在后面可继续使用 x
} // x 先移出了作用域，然后是 s，但因为 s 的值已被移走，所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // some_string 移出作用域并调用 `drop` 方法,占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // some_integer 移出作用域,不会有特殊操作
```
### 返回值和作用域
```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值所有权移给 s1
    let s2 = String::from("hello");     // s2 进入作用域
    let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中,将返回值所有权移给 s3，s2失效
} // s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，不做特殊操作。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string                              // 返回 some_string 并移出给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}
```
> 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

下面是一个通过元组返回参数所有权的例子
```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度
    (s, length)
}
```
## 引用和借用
& 符号代表 引用，允许你使用值但不获取其所有权
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}// s离开作用域，由于不拥有引用值的所有权，不做特殊操作
```
存储结构图如下
![](views/4.svg)

我们将获取引用作为函数参数称为 借用。rust 默认不允许修改引用的值，如果要修改引用的值，如下所示，修改 s 为 mut，创建可变引用 &mut s 和接受可变引用 some_string: &mut String
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

可变引用有两个限制
- 在特定作用域中的特定数据有且只有一个可变引用。这个限制的好处在于 Rust 可以在编译时就避免数据竞争。（可以使用大括号创建新作用的方式来允许多个可变引用）

```rust
let mut s = String::from("hello");

let r1 = &mut s;
// let r2 = &mut s; 报错
// {let r2 = &mut s;} 正确
```
- 不能在拥有不可变引用的同时拥有可变引用
```rust
let mut s = String::from("hello");
let r1 = &s; 
let r2 = &s; 
// let r3 = &mut s; 报错
```

## Slice
slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合，是一种没有所有权的数据结构。
```rust
// “字符串 slice” 的类型声明写作 &str
fn first_word(s: &String) -> &str {
    // as_bytes 方法将 String 转化为字节数组
    let bytes = s.as_bytes();
    // iter 方法在字节数组上创建一个迭代器,返回集合中的每一个元素
    // enumerate 包装 iter 的结果并返回一个元组
    //元组的第一个元素是索引，第二个元素是集合中元素的引用
    for (i, &item) in bytes.iter().enumerate() {
        // 判断是否是字符空格
        if item == b' ' {
            return &s[0..i];
        }
    }
    // 获取整个字符串的 slice
    &s[..]
}
```
下面例子中， s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的,因为 &str 是一个不可变引用
```rust
// 字符串字面值就是 slice
let s = "Hello, world!";
```
下面例子展示其他类型的 slice
```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

