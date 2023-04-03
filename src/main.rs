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
        if rand::thread_rng().gen_range(0..2) == 0 {
            println!("\x1b[95m{0}\x1b[0m\n\x1b[93m1. \x1b[0m{1}\n\x1b[93m2.\x1b[0m {2}\n\x1b[93m3.\x1b[0m {3}\n\nType \x1b[92mAnswer\x1b[0m Here: ", 
                 questions[itertimes], 
                 answers[itertimes], 
                 fakeopts1[itertimes], 
                 fakeopts2[itertimes]
                 );
            } else if rand::thread_rng().gen_range(0..2) == 1 {
                println!("\x1b[95m{0}\x1b[0m\n\x1b[93m1. \x1b[0m{1}\n\x1b[93m2.\x1b[0m {2}\n\x1b[93m3.\x1b[0m {3}\n\nType \x1b[92mAnswer\x1b[0m Here: ",
                         questions[itertimes], 
                         fakeopts1[itertimes],
                         answers[itertimes],  
                         fakeopts2[itertimes]
                         );
                } else {
                    println!("\x1b[95m{0}\x1b[0m\n\x1b[93m1. \x1b[0m{1}\n\x1b[93m2.\x1b[0m {2}\n\x1b[93m3.\x1b[0m {3}\n\nType \x1b[92mAnswer\x1b[0m Here: ", 
                             questions[itertimes],  
                             fakeopts1[itertimes],
                             fakeopts2[itertimes],
                             answers[itertimes] 
                             );
                    }

        let mut gottenans = String::new();
        io::stdin()
            .read_line(&mut gottenans)
            .expect("Fail");
        
        let gottenans: &str = gottenans.trim_end();
        
        if gottenans == answers[itertimes] {
            println!("Correct!");
            itertimes+=1;
            } else {
                println!("Incorrect...");
                }
        }
    println!("Wohoo! You win!");

    }

