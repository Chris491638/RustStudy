## 枚举
```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```
我们可以使用一种更简洁的方式来表达相同的概念，仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分
```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));

```
如上，我们可以将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。用枚举替代结构体的另一个优势在于，每个成员可以处理不同类型和数量的数据。
```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```
我们同样可以使用 impl 来为枚举定义方法
```rust
impl IpAddr {
    fn call(&self) {
        println!("{:#?}",self);
    }
}
home.call();

loopback.call();
```
### Option 枚举
```rust
enum Option<T> {
    Some(T),
    None,
}
// Option<T> 枚举被包含在了 prelude 之中，因此不需要显式引入作用域
let some_number = Some(5);
let some_string = Some("a string");
// 如果使用 None 而不是 Some，需要告诉 Rust Option<T> 是什么类型的，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型
let absent_number: Option<i32> = None;
```
## match
```rust
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        _ => ()
    }
}
```
如果调用 value_in_cents(Coin::Quarter(UsState::Alaska))，coin 将是 Coin::Quarter(UsState::Alaska)。将值与每个分支比较时，匹配 Coin::Quarter(state)，此时，state 绑定的值是 UsState::Alaska。`_` 是通配符。`()` 就是 unit 值，所以 `_` 的情况什么也不会发生。
## if let
当只关心某一种情况时，match 显得很冗余，因此我们使用 if let
```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}
// 等价于
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```
