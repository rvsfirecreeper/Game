use text_io::read;
use fastrand;
fn main() {
    println!("Let's play a game. The Mysterious Random Number Generator will select a number from 1 to the number you choose. Then we'll have to guess it.");
    println!("Difficulty select: Pick a number");
    let mut difficulty:i32 = read!();
    let mut num = (fastrand::i32(..) % difficulty).abs() + 1;
    let mut i:i32;
    let mut flag:bool;
    let mut newrecord:bool;
    let mut record = difficulty;
    let mut playagain:String;
    loop {
        i = 1;
        newrecord = false;
        loop {
            flag = false;
            println!("What will you guess!");
            let numguess: i32 = read!();
            if numguess < 1 || numguess > difficulty {
                println!("Bro, don't be an idiot I literally said 1 to {difficulty} that's not 1 to {difficulty}!!!");
                flag = true;
            } else if num == numguess {
                println!("YOU WON AFTER {} MOVES!", i);
                if i < record {
                    record = i.clone();
                    newrecord = true;
                }
                break;
            } else if num < numguess {
                println!("Hmm.... My gut tells me you were too high.")
            } else if num > numguess {
                println!("Hmm... My gut tells me you were too low.");
            }
            if !flag {
                i = i + 1;
            }
        }
        if newrecord {
            println!("Woah! You set a new record! Can you do it again? Say yes(Lowercase I struggle with Emphasis) if you wanna play again, if not just do anything and we're chill");
        }
        else {
            println!("Awww! You didn't set a new record... Can you do it next time? Say yes(Lowercase I struggle with Emphasis) if you wanna play again, if not just do anything and we're chill");
        }
        playagain = read!();
        if !(playagain == "yes") {
            break;
        }
        println!("Was that a little too easy or a little too hard? I can change the difficulty if you'd like.");
        playagain = read!();
        if playagain == "yes" {
            println!("Nice! What to change it to?");
            difficulty = read!();
            record = difficulty;
        }
        num = fastrand::i32(..) % difficulty;
    }
}
