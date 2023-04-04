use std::
    {
    io,
    thread,
    time,
    };

use rand::
    {
    Rng
    };

fn main() {
    // [[ VARIABLES ]] // 
    const QUIZLEN: usize = 3;
    let questions: [&str; QUIZLEN] = 
        [
        "What is the capital of India?", 
        "What is the Population of the World?",
        "What is glass made of?"
        ];
    let answers: [&str; QUIZLEN] = 
        [
        "Delhi",
        "8B",
        "Sand"
        ];
    let fakeopts1: [&str; QUIZLEN] = 
        [
        "Mumbai",
        "1T",
        "Snow",
        ];
    let fakeopts2: [&str; QUIZLEN] = 
        [
        "London",
        "6B",
        "Water"
        ];
    let mut itertimes: usize = 0;
    let mut questions_correct: i32 = 0;

    // [[ QUESTION-GIVING LOGIC ]] //
    while itertimes <= questions.len() - 1 
    {
        println!("--------------------------------------------");
        let mut num: &str;
        if rand::thread_rng().gen_range(0..2) == 0 
        {
            println!("\x1b[92m {0} \x1b[0m", questions[itertimes]);
            println!("\x1b[93m1. \x1b[0m{0}", fakeopts1[itertimes]);
            println!("\x1b[93m2.\x1b[0m {0}", answers[itertimes]);
            println!("\x1b[93m3.\x1b[0m {0}", fakeopts2[itertimes]);
            num = "2";
        } else if rand::thread_rng().gen_range(0..2) == 1 
        {
            println!("\x1b[92m {0} \x1b[0m", questions[itertimes]);
            println!("\x1b[93m1. \x1b[0m{0}", answers[itertimes]);
            println!("\x1b[93m2.\x1b[0m {0}", fakeopts2[itertimes]);
            println!("\x1b[93m3.\x1b[0m {0}", fakeopts1[itertimes]);
            num = "1";
        } else 
        {
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
        
        if gottenans == answers[itertimes] || gottenans == num 
        {
            println!("Correct!");
            itertimes+=1;
            questions_correct +=1;
        } else 
        {
            println!("Incorrect...");
            itertimes+=1;
        }
    }
    
    // [[ END OF QUESTION-GIVING LOGIC ]] //

    // [[ END MESSAGE ]] //
    if questions_correct == 3 
    {
        println!("Wohoo! You \x1b[92mwin!\x1b[0m");
    } else 
    {
        println!("Wohoo! You \x1b[93mcompleted the quiz!\x1b[0m");
        println!("\x1b[92m{questions_correct}\x1b[0m/{QUIZLEN} correct");
    }
}
