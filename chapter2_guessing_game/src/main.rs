use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);                   //랜덤 수 생성 1~100
    println!("the secret number is: {}",secret_number);
    loop {                                                                            //반복문
        println!("please input your guess.");                                         
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");     //input 읽기

        let guess: i32= match guess.trim().parse(){                                   // parse() : string => i32 로 형변환
            Ok(num) => num,                                                      //trim()   : " ","\n","\r" ect. 삭재
            Err(_) => continue,                                                       // match ... ... { OK,Err} 예외처리
        };

        println!("You guessed: {guess}");
        
        match guess.cmp(&secret_number) {                                             // 입력수와 랜덤수 비교
            Ordering::Equal => {
                println!("You win");
                break;
            },
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big")
        }
    }
}
