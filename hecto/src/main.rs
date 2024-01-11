use std::io::{self, stdout, Read};
fn main() {
    // 遍历标准输入流中的每一个字节
    for b in io::stdin().bytes() {
        // 将字节转换为字符
        let c = b.unwrap() as char; 
        // 如果字符为q，则终止程序
        if c == 'q' {
            break;
        }
        // 打印字符
        println!("{}", c); 
    }
}