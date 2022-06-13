use std::io;
use rand::prelude::*;

const NUM_DIGITS : usize = 4;
const GREEN: &'static str = "🟩";
const YELLOW: &'static str = "🟨";
const BLANK: &'static str = "⬜";


fn calc_green_and_yellow(guess: &[i32], secret: &[i32]) -> String {
    let mut secret = Vec::from(secret);
    let mut guess = Vec::from(guess);
    let mut result: [&'static str; 4] = [BLANK, BLANK, BLANK, BLANK];

    for ((g, s), r) in guess
        .iter_mut()
        .zip(secret.iter_mut())
        .zip(result.iter_mut())
    {
        if *g == *s {
            *s = 0;
            *g = -1;
            *r = GREEN;
        }
    }

    for (g, r) in guess.iter().zip(result.iter_mut()) {
        if let Some(pos) = secret.iter().position(|s| *s == *g) {
            secret[pos] = 0;
            if *r == BLANK {
                *r = YELLOW;
            }
        }
    }

    result.join(" ")
}






fn main() {
    println!("Let's play Number Wordle!");

    let secret: Vec<_> = (0..NUM_DIGITS).map(|_|rand::thread_rng().gen_range(1..10)).collect();
    
    let stdin = io::stdin();

    let mut buf = String::new();


    

    loop {
        buf.clear();
        print!("guess: ");
        stdin.read_line(&mut buf).unwrap();
        let guess : Result<Vec<i32>, _> = buf.trim().split(' ').map(|s| s.parse()).collect();
    
        if let Ok(guess) = guess {
            if guess.iter().filter(|x| **x <= 9).count() == NUM_DIGITS {
                let squares = calc_green_and_yellow(&guess, &secret);

                println!("{:?} gave {}", guess, squares);

                if squares == "🟩 🟩 🟩 🟩" {
                    break;
    
                }
            }

        }
    }
}


 #[test]
 fn test_green_and_yellow() {
     assert_eq!(calc_green_and_yellow(&[1, 2, 3, 4], &[1, 2, 3, 4]), "🟩 🟩 🟩 🟩".to_string());
     assert_eq!(calc_green_and_yellow(&[1, 2, 3, 5], &[1, 2, 3, 4]), "🟩 🟩 🟩 ⬜".to_string());
     assert_eq!(calc_green_and_yellow(&[4, 3, 2, 1], &[1, 2, 3, 4]), "🟨 🟨 🟨 🟨".to_string());
     assert_eq!(calc_green_and_yellow(&[1, 2, 3, 1], &[1, 2, 3, 4]), "🟩 🟩 🟩 ⬜".to_string());
     assert_eq!(calc_green_and_yellow(&[1, 1, 1, 1], &[1, 2, 3, 4]), "🟩 ⬜ ⬜ ⬜".to_string());
     assert_eq!(calc_green_and_yellow(&[1, 2, 2, 2], &[2, 2, 2, 1]), "🟨 🟩 🟩 🟨".to_string());
     assert_eq!(calc_green_and_yellow(&[1, 3, 3, 2], &[2, 2, 2, 1]), "🟨 ⬜ ⬜ 🟨".to_string());
 }