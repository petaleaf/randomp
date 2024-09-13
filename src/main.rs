use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use clap::Parser;


#[derive(Parser)]
#[command(name = "randomp")]
#[command(author = "tantan")]
#[command(version = "1.0")]
#[command(about = "一个简单的随机密码生成器")]
pub struct Args {
    #[arg(default_value = "8", help = "生成随机密码的长度")]
    length: usize,
}


fn generate_rand_string(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

fn check_length(length: usize) -> usize {
    let min_length = 4;
    let max_length = 128;
    std::cmp::max(min_length, std::cmp::min(length, max_length))
}

fn main() {
    let args = Args::parse();
    let length = check_length(args.length);
        
    if length != args.length {
        eprintln!("错误：密码长度必须在 4 到 128 位之间！");
        std::process::exit(1);
    }

    let rand_string = generate_rand_string(length);

    println!("{}", rand_string);
}
// fn main() {
//     use rand::Rng;
//     const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
//                             abcdefghijklmnopqrstuvwxyz\
//                             0123456789)(*&^%$#@!~";
//     const PASSWORD_LEN: usize = 30;
//     let mut rng = rand::thread_rng();

//     let password: String = (0..PASSWORD_LEN)
//         .map(|_| {
//             let idx = rng.gen_range(0..CHARSET.len());
//             CHARSET[idx] as char
//         })
//         .collect();

//     println!("{:?}", password);
// }
