# Hello,Cargo
- cargo new hello_cargo --bin
  新建hello_cargo项目，bin参数会生成一个可执行程序，而不是一个库。cargo自动生成src目录以及Cargo.toml配置文件。
- cargo build
  编译项目，生成可执行文件（调试模式在target/debug目录下）
- cargo run 
  在build的基础上自动运行，如果编译之后，源文件未变更，直接运行已编译好的可执行文件，不再编译
- cargo check
  只编译，不生成可执行文件，速度较快 
- cargo build --release
  发布构建，在target/release目录下生成可执行文件，相比较于debug，运行更快，但优化编译导致编译速度更慢