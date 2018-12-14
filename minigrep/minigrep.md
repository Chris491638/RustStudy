## 接收命令行参数
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    // cargo run test test.txt
    // ["target/debug/minigrep", "test", "test.txt"]
    // 第一个参数是二进制文件
}
```
> std::env::args 在其任何参数包含无效 Unicode 字符时会 panic。如果你需要接受包含无效 Unicode 字符的参数，使用 std::env::args_os 代替。这个函数返回 OsString 值而不是 String 值。

## 读取文件
```rust
use std::env;
use std::fs;

fn main() {
    // --snip--
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```

## 重构以改进模块化和错误处理
### 组合配置值
```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    // clone 创建数据的完整拷贝
    // 比储存字符串数据的引用消耗更多的时间和内存
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```
### 创建 Config 的构造函数
```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    // --snip--
}
// --snip--
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
```
### 改善错误信息
```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```
```rust
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // unwrap_or_else 可以自定义非 panic 的错误处理
    // Result 是 Ok 时，类似于 unwrap ,返回 Ok 内部封装的值。
    // Err 时，调用一个 闭包,将 Err 内部值传递给匿名函数
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // process::exit 会立即停止程序并将传递给它的数字作为退出状态码
        process::exit(1);
    });

    // --snip--
}
```
### 从 main 提取逻辑
```rust
fn main() {
    // --snip--
    run(config);
}
fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
}
```
### run 函数中返回错误
```rust
use std::error::Error;
// --snip--
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}
```
### 处理 main 中 run 返回的错误
```rust
fn main() {
    // --snip--
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```
### 将代码拆分到库
```rust
// src/lib.rs
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}
```
```rust
extern crate minigrep;
use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}
```
### 搜索函数
```rust
// 添加显示的生命周期，告诉rust，返回的数据跟contents参数存在周期一致
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
```
### 大小写不敏感的搜索函数
```rust
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```
### 通过环境变量判断是否大小写敏感
```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```
```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
```
```rust
use std::env;
// --snip--
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
        // env::var("CASE_INSENSITIVE") 检查系统中是否有名为 CASE_INSENSITIVE 的环境变量
        // is_err 只判断是否是 Err ，返回 true 或 false 
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}
```
### 输出错误信息到标准错误而不是标准输出
```rust
// 使用 eprintln! 将错误信息写入标准错误而不是标准输出
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
```
> cargo run > output.txt
Problem parsing arguments: not enough arguments

> cargo run to poem.txt > output.txt