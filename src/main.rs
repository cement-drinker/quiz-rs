use std::{
    io,
    thread,
    time,
    };
use rand::{
    Rng
    };

fn main() {
    const QUIZLEN: usize = 3;
    let questions: [&str; QUIZLEN] = [
        "What is the capital of India?", 
        "What is the Population of the World?",
        "What is glass made of?"
        ];
    let answers: [&str; QUIZLEN] = [
        "Delhi",
        "8B",
        "Sand"
        ];
    let fakeopts1: [&str; QUIZLEN] = [
        "Mumbai",
        "1T",
        "Snow",
        ];
    let fakeopts2: [&str; QUIZLEN] = [
        "London",
        "6B",
        "Water"
        ];
    let mut itertimes: usize = 0;

    while itertimes <= questions.len() - 1 {
        println!("--------------------------------------------");
        let mut num: &str;
        if rand::thread_rng().gen_range(0..2) == 0 {
            println!("\x1b[92m {0} \x1b[0m", questions[itertimes]);
            println!("\x1b[93m1. \x1b[0m{0}", fakeopts1[itertimes]);
            println!("\x1b[93m2.\x1b[0m {0}", answers[itertimes]);
            println!("\x1b[93m3.\x1b[0m {0}", fakeopts2[itertimes]);
            num = "1";
            } else if rand::thread_rng().gen_range(0..2) == 1 {
                println!("\x1b[92m {0} \x1b[0m", questions[itertimes]);
                println!("\x1b[93m1. \x1b[0m{0}", answers[itertimes]);
                println!("\x1b[93m2.\x1b[0m {0}", fakeopts2[itertimes]);
                println!("\x1b[93m3.\x1b[0m {0}", fakeopts1[itertimes]);
                num = "2";
                } else {
                    println!("\x1b[92m {0} \x1b[0m", questions[itertimes]);
                    println!("\x1b[93m1. \x1b[0m{0}", fakeopts2[itertimes]);
                    println!("\x1b[93m2.\x1b[0m {0}", fakeopts1[itertimes]);
                    println!("\x1b[93m3.\x1b[0m {0}", answers[itertimes]);
                    num = "3";
                    }
        println!("\nType \x1b[92mAnswer\x1b[0m Here: "); 
        let mut gottenans = String::new();
        io::stdin()
            .read_line(&mut gottenans)
            .expect("Fail");
        
        let gottenans: &str = gottenans.trim_end();
        
        if gottenans == answers[itertimes] || gottenans == num {
            println!("Correct!");
            itertimes+=1;
            } else {
                println!("Incorrect...");
                }
        }
    println!("Wohoo! You win!");

    }

