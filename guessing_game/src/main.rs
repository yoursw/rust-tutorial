/*
 * Guessing Game based on: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // TODO: Check if there is a way to secure this in memory
    let secret_number: u8 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // guess is a positive integer - shadow and parse as such.
        // TODO: Add validation for numbers 1-100
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number between 1-256.");
                continue;
            }
        };

        // TODO: If Debug :)
        // println!("The secret number is: {secret_number}");
        println!("You guessed {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
