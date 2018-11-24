## mod与文件系统
### 模块定义
```rust
// 定义一个 network 模块，包含一个 connect 的函数
// 在 network 模块外，使用 network::connect() 调用
mod network {
    fn connect() {
    }
}
// 可以定义多个模块
mod client {
    fn connect() {
    }
}
```
也可以将模块放入其他模块中,使用 network::client::connect() 调用
```rust
mod network {
    fn connect() {
    }

    mod client {
        fn connect() {
        }
    }
}
```
### 将模块移动到其他文件
```rust
// src/lib.rs
mod client {
    fn connect() {
    }
}

mod network {
    fn connect() {
    }

    mod server {
        fn connect() {
        }
    }
}
```
将上述代码简化成
```rust
mod client;
mod network; 
```
首先新建 src/client.rs
```rust
// src/client.rs
fn connect(){
}
```
然后新建 src/network/mod.rs
```rust
fn connect(){
}
mod server;
```
新建 src/network/server.rs
```rust
fn connect(){ 
}
```
### 模块文件系统的生成规则
- 如果一个叫做 foo 的模块没有子模块，应该将 foo 的声明放入叫做 foo.rs的文件中。
- 如果一个叫做 foo 的模块有子模块，应该将 foo 的声明放入叫做 foo/mod.rs 的文件中。
## 使用pub控制可见性
Rust 所有代码的默认状态是私有的.在声明的开头增加 pub 关键字，可以将函数标记成公有。
### 私有性规则
- 如果一个项是公有的，它能被任何父模块访问
- 如果一个项是私有的，它能被其直接父模块及父模块的任何子模块访问
```rust
mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}
// try_me 函数和 outermost 模块同处于跟模块中，outermost 模块私有，根据第二条规则，try_me 函数可以访问 outermost 模块
fn try_me() {
    // middle_function 函数公有，根据第一条原则，可以访问
    outermost::middle_function();
    // middle_secret_function 函数是私有的，根据第二条定义，它只能被 outermost及其子模块访问，try_me所属的根模块不符合，发生编译错误
    // outermost::middle_secret_function();
    // inside私有且没有子模块，所以只能被outermost访问，错误
    // outermost::inside::inner_function();
    // outermost::inside::secret_function();
}
```
### 使用use关键字将名称导入作用域
```rust
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of;

fn main() {
    // use 关键字只将指定的模块引入作用域,它并不会将其子模块也引入。
    // 这就是为什么想要调用 nested_modules 函数时仍然必须写成 of::nested_modules
    of::nested_modules();
}
```
### 使用 super 访问父模块
```rust
#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
    }
}
```