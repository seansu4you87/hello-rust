use std::io;
use std::rand::Rng;

fn main() {
    let secret_num = std::rand::task_rng().gen_range(1i, 101);
    // println!("SECRET: {}", secret_num);
    let mut guesses = 5i;

    println!("INPUT:");
    let mut reader = io::stdin();

    loop {
        let input = reader.read_line().ok().expect("Failed to read line");
        let input_num: Option<int> = from_str(input.as_slice().trim());

        let num = match input_num {
            Some(num) => num,
            None      => {
                println!("HEY!, PUT IN A NUMBER");
                continue;
            }
        };

        match num.cmp(&secret_num) {
            Less    => println!("NOPE, HIGHER!"),
            Greater => println!("NOPE, LOWER!"),
            Equal   => {
                println!("YOU WIN! {} IS THE RIGHT ANSWER!", secret_num);
                break;
            }
        }

        guesses = guesses - 1;
        match guesses.cmp(&0) {
            Equal => {
                println!("OUT OF GUESSES! YOU LOSE! THE NUMBER WAS: {}", secret_num);
                break;
            },
            _     => continue
        }
    }
}
