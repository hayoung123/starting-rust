extern crate rand;
use rand::Rng;
use std::io;

fn main() {
  let mut count = 0;
  let answer = get_baseball_answer();
  println!("정답은? {}{}{}",answer[0],answer[1],answer[2]);

  loop{
    println!("숫자 세개를 순서대로 입력해주세요");
    count+=1;
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
          .expect("입력에 실패했습니다.");
    
    let guess: i32 = guess.trim().parse()
          .expect("숫자 3개를 입력해주세요.");
  

  
    let (strike_count, ball_count) = check_result(answer,get_char_from_number(guess));
  
    if strike_count==3 {
      println!("정답입니다! 총 시도 횟수는: {}번 입니다.",count);
      break;
    }else{
      println!("시도 횟수: {}번", count);
      println!("스크라이크: {} 볼: {}",strike_count, ball_count);
    }
  }
}

fn get_random_number() -> i32{
  return rand::thread_rng().gen_range(1, 10);
}

fn get_baseball_answer() -> Vec<i32>{
  let answer:Vec<i32> = vec![];

  loop{
    let ran_number = get_random_number();

    if answer.contains(&ran_number){
      continue;
    }else{
      answer.push(ran_number);
    }

    if answer.len()==3 {
      break;
    }
  }

  return answer;
}


fn check_result(answer:Vec<i32>, guess:Vec<i32>) -> (i32, i32){
  let mut strike_count = 0;
  let mut ball_count = 0;

  println!("정답은? {}{}{}",answer[0],answer[1],answer[2]);
  println!("입력한 값 {}{}{}",guess[0],guess[1],guess[2]);



  for i in 0..3 {
    let guess_number = guess[i];

    if answer[i]==guess_number{
      strike_count+=1;
    };
    
    for j in 0..3{
      if j==i{ 
        continue;
      }
      else{
        if answer[j]== guess_number{
          ball_count+=1;
        }
      }
    }
  }

  return (strike_count,ball_count);
}

fn get_char_from_number(number:i32)->Vec<i32>{
  let third = number % 10;
  let number = number - third;
  let second = (number % 100)/10;
  let number = number - second * 10;
  let first = number/100;

  return vec![first,second,third];
}