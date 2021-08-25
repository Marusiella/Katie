// I'm not very good, so please bear with me. Don't be scared of the code.



use std::io::{stdin, stdout, Write}; // For read input
use colored::Colorize; // For colorize output
use std::{thread, time}; // For do "sleep statement" to exits program in 5 seconds.
use std::process::Command; // For run statement
use chrono::Utc; // For date
use chrono::Datelike; // For date
use rand::Rng; // For random numbers generator
use std::num::ParseIntError; // Just I need that
use std::io; // I didn't feel like inputting as before, because my head hurt already
// use std::num::sqrt; // For sqrt numbers in calculator

// Reading input
fn input(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn run_notepad() -> Vec<u8> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start notepad"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo Error in command")
            .output()
            .expect("failed to execute process")
    };

    output.stdout
}

fn run_mspaint() -> Vec<u8> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start mspaint"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo Error in command")
            .output()
            .expect("failed to execute process")
    };

    output.stdout
}

fn run_cmd() -> Vec<u8> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "start cmd"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo Error in command")
            .output()
            .expect("failed to execute process")
    };

    output.stdout
}

fn time_s() -> i32 {
    let date_t = Utc::now();
    let timestamp: i64 = date_t.timestamp();

    timestamp as i32 // Convert timestamp to i32 for better visualization
}

fn date_t() -> String {
    let actually_date = chrono::Utc::now();
    let year: u32 = actually_date.year() as u32; // Year is in i32 so i must convert that to u32
    let month = actually_date.month();
    let day = actually_date.day();
    let result = year + month + day;
    return format!("{}.{}.{}", year, month, day);
}

pub fn add_letters() -> String {
    println!();
    let mut rng = rand::thread_rng();
    let mut letters_len = String::new();
    let mut ret = String::new();

    println!("Write length of letters in password");
    println!();

    io::stdin().read_line(&mut letters_len)
        .ok()
        .expect("failed to parse");


    for random_letters in 0..letters_len.trim().parse::<u32>().unwrap() {
        let letter: char = rng.gen_range('A'..='z');
        print!("{}", letter);
        ret.push(letter);
    }
    println!();
    ret
}

pub fn add_numbers() -> String {
    println!();
    let mut rng = rand::thread_rng();
    let mut numbers_len = String::new();
    let mut ret = String::new();

    println!("Type length of numbers in password");
    println!();

    io::stdin().read_line(&mut numbers_len)
        .ok()
        .expect("failed to parse");


    for random_numbers in 0..numbers_len.trim().parse::<u32>().unwrap() {
        let number: char = rng.gen_range('0'..='9');
        print!("{}", number);
        ret.push(number);
    }
    println!();
    ret
}

fn add_special_char() -> String {
    println!();
    let mut rng = rand::thread_rng();
    let mut special_char_len = String::new();
    let mut ret = String::new();

    println!("Type length of special chars in password");
    println!();

    io::stdin().read_line(&mut special_char_len)
        .ok()
        .expect("failed to parse");


    for random_special_chars in 0..special_char_len.trim().parse::<u32>().unwrap() {
        let special_char: char = rng.gen_range('!'..='#');
        print!("{}", special_char);
        ret.push(special_char);
    }
    println!();
    ret
}

fn main() {
    println!("{}", "
██╗░░██╗░█████╗░████████╗██╗███████╗
██║░██╔╝██╔══██╗╚══██╔══╝██║██╔════╝
█████═╝░███████║░░░██║░░░██║█████╗░░
██╔═██╗░██╔══██║░░░██║░░░██║██╔══╝░░
██║░╚██╗██║░░██║░░░██║░░░██║███████╗
╚═╝░░╚═╝╚═╝░░╚═╝░░░╚═╝░░░╚═╝╚══════╝".bold().blue());

    println!("{}", r#"Hi, I'm Katie - Virtual Computer Assistant. Enter "help" to see what I can do for you!"#.blue());

    loop {
        println!();

        let mut help = String::new();
        let for_moment = input(&mut help);
        let help_trimmed = help.trim();

        match help_trimmed {
            // Help command
            "help" => { println!("{}", "
                1. help -> Displays all commands in Katie. \n
                2. run -> Displays all the apps Katie can open for you. \n
                3. exit -> Exiting program in 5 seconds \n
                4. date -> Display current date \n
                5. timestamp -> Display timestamp \n
                6. password -> Password generating program\n
                7. calculator -> Calculating two numbers\n".green());
            },

            // Description of "run" command
            "run" => { println!("{}", "
                1. run notepad -> Will open notepad. \n
                2. run paint -> Will open paint. \n
                3. run cmd -> Will open cmd \n".green());
                },

            // Exit statement
            "exit" => {
                let mut time_to_exit = 5; // Variable for exit program in 5 second. Not immediately, cuz it would look weird.

                while(time_to_exit > -1) {
                    println!("Program exit in {} seconds", time_to_exit);

                    time_to_exit = time_to_exit - 1;
                    thread::sleep(time::Duration::from_millis(1000)); // Sleep for 1 second.

                    if(time_to_exit == 0) {
                        println!("{}", "Goodbye user!".red().bold());
                        std::process::exit(0);
                    }
                }
            },

            // --------------------------------
            // SECOND DAY OF CODING THIS PROGRAM
            // --------------------------------

            // Run notepad function
            "run notepad" => {
                println!("{}", "Running notepad...".blue());
                run_notepad();
            },

            // Run paint function
            "run paint" => {
                println!("{}", "Running paint...".blue());
                run_mspaint();
            },

            // Run cmd function
            "run cmd" => {
                println!("{}", "Running cmd...".blue());
                run_cmd();
            },

            // Printing actually date
            "date" => {
                println!("Current date: {}", date_t())
            },

            // Printing timestamp
            "timestamp" => {
                println!("Timestamp: {}", time_s())
            },

            "password" => {
                println!();
                println!("You'r generate password: {}{}{}", add_letters(), add_numbers(), add_special_char());
            },

            "calculator" => {
                println!("Enter first number");

                let mut firstNumber = String::new();
                let firstNumber_before_trim = input(&mut firstNumber);
                let firstNumber_trimmed = firstNumber.trim();
                println!("Enter second number");

                let mut secondNumber = String::new();
                let secondNumber_before_trim = input(&mut secondNumber);
                let secondNumber_trimmed = secondNumber.trim();

                let firstNumber_trimmed: i32 = firstNumber_trimmed.parse().unwrap();
                let secondNumber_trimmed: i32 = secondNumber_trimmed.parse().unwrap();

                let adding_result = firstNumber_trimmed + secondNumber_trimmed;
                println!("{} + {} = {}", firstNumber_trimmed, secondNumber_trimmed, adding_result);

                let subtracting_result = firstNumber_trimmed - secondNumber_trimmed;
                println!("{} - {} = {}", firstNumber_trimmed, secondNumber_trimmed, subtracting_result);

                let multiplicating_result = firstNumber_trimmed * secondNumber_trimmed;
                println!("{} * {} = {}", firstNumber_trimmed, secondNumber_trimmed, multiplicating_result);

                let divising_result = firstNumber_trimmed / secondNumber_trimmed;
                println!("{} / {} = {}", firstNumber_trimmed, secondNumber_trimmed, divising_result);

                let test_result = firstNumber_trimmed ^ secondNumber_trimmed;
                println!("{} ^ {} = {}", firstNumber_trimmed, secondNumber_trimmed, test_result);

                let upperLimit_result1: u64 = (firstNumber_trimmed * firstNumber_trimmed) as u64;
                let upperLimit_result2: u64 = (secondNumber_trimmed * secondNumber_trimmed) as u64;

                println!("First number to square: {}", upperLimit_result1);
                println!("Second number to square: {}", upperLimit_result2);
            }

            // Error statement
            _ => println!("{}", r#"Invaild command. Enter "help" to see what I can do for you!"#.bold().red()),
        }
    }
}