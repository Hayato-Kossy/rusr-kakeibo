use std::io;
use kakeibo::services;

fn main() {
    let mut service_type = String::new();
    println!("サービスタイプを入力してください: (0:登録、1:集計)");
    io::stdin().read_line(&mut service_type).unwrap();
    let service_type: u8 = service_type.trim().parse().expect("数値で入力してください");
    services::validate::InputValidater::validate_service_type(service_type);
    
    if service_type == 0 {
        println!("登録サービス");
    } else if service_type == 1{
        println!("集計サービス");
    }
}