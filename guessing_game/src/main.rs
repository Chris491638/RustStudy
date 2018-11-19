/*
 * 使用外部库时，需要先修改 Cargo.toml 引入依赖
 * 通知 Rust 我们要使用外部依赖 
 */
extern crate rand;
/* 
 * 默认情况下，Rust 将 prelude 模块中少量的类型引入到每个程序的作用域中。
 * 如果需要的类型不在 prelude 中，必须使用 use 语句显式地将其引入作用域。
 * std::io 库提供很多有用的功能，包括接收用户输入的功能。
 */   
use std::io;
/*
 * Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal
 */
use std::cmp::Ordering;
/* 
 * Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中
 * 运行 cargo doc --open 命令来构建所有本地依赖提供的文档，并在浏览器中打开
 * 从而了解使用哪个 trait 以及该从 crate 中调用哪个方法 
 */
use rand::Rng;

fn main() {
    println!("Guess the number!");

    /*
     * rand::thread_rng 函数提供实际使用的随机数生成器,
     * 它位于当前执行线程本地，并从操作系统获取 seed;
     * gen_range 方法由刚才引入到作用域的 Rng trait 定义,
     * 获取两个数字作为参数，并生成一个范围在两者之间的随机数,它包含下限但不包含上限
     */
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");
        /*
         * String::new()函数返回一个 String 的新实例。
         * :: 语法表明 new 是 String 类型的一个 关联函数(即静态方法/类方法)
         * String 是标准库提供的字符串类型，它是 UTF-8 编码的可增长文本块
         */
        let mut guess = String::new();

        /*
         * 如果程序的开头没有 use std::io 这一行，可以把函数调用写成 std::io::stdin;
         * stdin函数返回一个 std::io::Stdin 实例;
         * read_line 读取用户输入，以字符串形式存储在可变字符串 guess 中;
         * & 表示引用,相当于指针;
         * read_line 操作完成后会返回一个 io::Result 实例，是枚举类型，成员为 OK 和 Err ,
         * 当 io::Result 实例的值为 Err 时，expect 会导致程序崩溃，并打印函数调用时传递的参数,
         * 当 io::Result 实例的值是 Ok 时，expect 会获取 Ok 中的值并原样返回。
         * 在本例中，这个值是用户输入到标准输入中的字节数，
         * 假设我们输入了12，此时的字节数为3，因为最后还有一个换行符\n
         */
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");  

        /*
         * match 的作用相当于 case when;
         * trim() 去除字符串收尾空白字符，本例中去除换行符
         * parse() 将字符串解析成数字，由于类型不定，所有通过 u32 显式指定
         * 由于存在转换出错的情况，因此 parse 会返回一个 Result 类型
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        /*
         * cmp 方法用来比较两个值并可以在任何可比较的值上调用
         */ 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}