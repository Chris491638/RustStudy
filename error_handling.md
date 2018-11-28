## Result与可恢复的错误
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```
### 匹配不同的错误
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```
条件`if error.kind() == ErrorKind::NotFound`被称作`match guard`：它是一个进一步完善 match 分支模式的额外的条件。这个条件必须为真才能使分支的代码被执行；否则，模式匹配会继续并考虑 match 中的下一个分支。模式中的`ref`是必须的，这样 error 就不会被移动到 guard 条件中而仅仅只是引用它。为什么在模式中使用`ref`而不是`&`来获取一个引用？简而言之，在模式的上下文中，`&`匹配一个引用并返回它的值，而`ref`匹配一个值并返回。
### unwrap 和 expect
如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
```rust
let f = File::open("hello.txt").unwrap();
```
expect 用来调用 panic! 的错误信息将会作为参数传递给 expect ，而不像unwrap 那样使用默认的 panic! 信息。
```rust
let f = File::open("hello.txt").expect("Failed to open hello.txt");
```
### 传播错误
```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```
### 传播错误的简写
```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
Result 值之后的`?`表示如果 Result 的值是 Ok，返回 Ok 中的值而程序将继续执行；如果值是 Err，将 Err 中的值作为整个函数的返回值。`？`只能用于返回 Result 的函数。
我们可以使用`？`之后的链式调用进一步简写
```rust
File::open("hello.txt")?.read_to_string(&mut s)?;
```