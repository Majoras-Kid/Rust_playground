extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("[+] Welcome to the guessing game");
    let secret_number = rand::thread_rng().gen_range(1,101);
    //println!("[+] Secret_number = {}", secret_number);

    loop {

        println!("[+] Please input a number");
        let mut guess = String::new();

        //let random_generator = rand::thread_rng();
        //let random_value:i32 = random_generator.gen(1..100);
        //println!("[+] Random Value {}",random_value);

        io::stdin().read_line(&mut guess)
            .expect("[-] Failed to read line");

        let guess: u32 = match guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => {
                    println!("[-] Error with guess variable {}", guess);
                    continue
                },
            };
            

        println!("[+] You guessed: {}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("[<] Too small!"),
            Ordering::Greater => println!("[>] Too big!"),
            Ordering::Equal => {
                println!("[=] You win!");
                break;},
        }
            
    }
}
