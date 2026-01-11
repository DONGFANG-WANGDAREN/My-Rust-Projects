use rand :: Rng;
use std :: cmp :: Ordering;
use std :: io;

fn main(){
    println!("猜数字");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("猜一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你输入的数字为：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering :: Less => println!("错了，小了。"),
            Ordering :: Greater => println!("错了，大了。"),
            Ordering :: Equal => {println!("对了。");break;}
        }
    }

}