extern crate rand;
use std::io;
use std::any::type_name;
use rand::Rng;


fn main() {
    println!("추리게임을 시작합니다.");
    println!("당신이 예상하시는 추리 숫자를 입력해주세요.");

    let password = rand::thread_rng().gen_range(1, 101);
    
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("입력에 실패했습니다.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("숫자를 입력해주세요!!");
                continue
            },
        };
            
        println!("당신이 예상한 숫자는 {}", guess);

        let difference: i32 = guess - password;

        if difference==0{
            println!("정답입니다.");
            break;
        }else if difference > 0 {
            println!("더 작은 수 입니다.");
        }else {
            println!("더 큰 수 입니다.");
        }
    }
}



fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

