use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess");
    println!("Input: ");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("Secret Num is {}", secret_num);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Fail read");
        // 需要将 string 转成 num
        // rust 允许利用同名的变量来替换旧变量的值，通常出现在类型转换的场景
        let guess: u32 = guess.trim().parse().expect("Type a num");
    
        println!("You guess is {}", guess);
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                // 相等加个 break 来处理
                println!("win");
                break;
            }
        }
    }
}
