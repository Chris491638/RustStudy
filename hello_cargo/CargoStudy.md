# Hello,Cargo
- cargo new hello_cargo --bin

  新建 hello_cargo 项目，bin 参数会生成一个可执行程序，而不是一个库。cargo 自动生成 src 目录以及 Cargo.toml 配置文件。
- cargo build

  编译项目，生成可执行文件（调试模式在 target/debug 目录下）
- cargo run 

  在 build 的基础上自动运行，如果编译之后，源文件未变更，直接运行已编译好的可执行文件，不再编译
- cargo check

  只编译，不生成可执行文件，速度较快 
- cargo build --release

  发布构建，在 target/release 目录下生成可执行文件，相比较于 debug，运行更快，但优化编译导致编译速度更慢

- cargo update
  
  用于升级 crate，update 会忽略 Cargo.lock 文件，并计算出所有符合 Cargo.toml 声明的最新版本。如果成功了，Cargo 会把这些版本写入 Cargo.lock 文件