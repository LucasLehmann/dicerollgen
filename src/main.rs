use std::env;

#[cfg(feature = "stats")]
use std::collections::HashMap;

#[cfg(feature = "roll")]
use rand::Rng;

const STATS: usize = 0x00;
    const STATS_FREQ: usize = STATS | 0x01;
    const STATS_VAL: usize = STATS | 0x02;
const HELP: usize = 0x10;
    const HELP_STATS: usize = HELP | 0x01;
    const HELP_ROLL: usize = HELP | 0x02;
const ROLL: usize = 0x20;
    const ROLL_SUM: usize = ROLL | 0x01;
    const ROLL_ROLLS: usize = ROLL | 0x02;
    const ROLL_SUM_ROLLS: usize = ROLL | ROLL_SUM | ROLL_ROLLS; // 0x23

fn main() {
    let (args, mode) = get_args().unwrap_or_else(|err| {
        print_help("help");
        panic!("Error: {}", err);
    });

    #[cfg(debug_assertions)]{
        println!("Args: {:?}", &args);
        println!("Mode: {:X}", &mode);
    }

    match mode & 0xF0 {
        STATS => stats(args, mode),
        HELP => print_help("help"),
        ROLL => roll(args, mode),
        _ => {}
    }
}

#[cfg(not(feature = "roll"))]
fn roll(args: Vec<[usize; 2]>, mode: usize) {
    println!("The roll feature was not compiled into this build. \n");
    print_help("roll");
}

#[cfg(feature = "roll")]
fn roll(dice: Vec<[usize; 2]>, mode: usize) {
    match mode {
        ROLL_SUM => println!("{}", roll_sum(dice)),
        ROLL_ROLLS => println!("{:?}", roll_rolls(dice)),
        ROLL_SUM_ROLLS => {
            let (sum, rolls) = roll_sum_rolls(dice);
            println!("{} {:?}", sum, rolls);
        }
        _ => println!("{}", roll_sum(dice)),
    }
}

#[cfg(feature = "roll")]
fn roll_sum(dice: Vec<[usize; 2]>) -> usize {
    let mut rng = rand::thread_rng();
    let mut sum: usize = 0;
    for die in dice {
        for _ in 0..die[0] {
            sum += rng.gen_range(1..=die[1]);
        }
    }
    sum
}

#[cfg(feature = "roll")]
fn roll_rolls(dice: Vec<[usize; 2]>) -> Vec<usize>{
    let mut rng = rand::thread_rng();
    let mut rolls: Vec<usize> = Vec::new();
    for die in dice {
        rolls.extend((0..die[0]).map(|_| rng.gen_range(1..=die[1])));
    }
    rolls
}

#[cfg(feature = "roll")]
fn roll_sum_rolls(dice: Vec<[usize; 2]>) -> (usize, Vec<usize>) {
    (roll_sum(dice.clone()), roll_rolls(dice))
}

#[cfg(not(feature = "stats"))]
fn stats(args: Vec<[usize; 2]>, mode: usize) {
    println!("The stats feature was not compiled into this build. \n");
    print_help("stats");
}

#[cfg(feature = "stats")]
fn stats(dice: Vec<[usize; 2]>, mode: usize) {
    todo!();
}

fn get_args() -> Result<(Vec<[usize; 2]>, usize), &'static str> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.is_empty() {
        return Err("No arguments provided");
    }
    let mode: usize;
    mode = match args.get(0).unwrap_or(&"".to_string()).as_str() {
        "stats" => {
            args.remove(0);
            match args.get(0).unwrap_or(&"".to_string()).as_str() {
                "freq" => {args.remove(0); STATS_FREQ}
                "val" => {args.remove(0); STATS_VAL}
                _ => STATS,
            }
        }
        "help" => {
            args.remove(0);
            match args.get(0).unwrap_or(&"".to_string()).as_str() {
                "stats" => {args.remove(0); HELP_STATS}
                "roll" => {args.remove(0); HELP_ROLL}
                _ => HELP,
            }
        }
        "roll" => {
            args.remove(0);
            match args.get(0).unwrap_or(&"".to_string()).as_str() {
                "sum" => {args.remove(0); ROLL_SUM}
                "rolls" => {args.remove(0); ROLL_ROLLS}
                "sum_rolls" => {args.remove(0); ROLL_SUM_ROLLS}
                _ => ROLL,
            }
        }
        _ => STATS,
    };
    let mut result = Vec::new();
    let mut no_args = true;
    for arg in args {
        no_args = false;
        let mut arg = arg.split("d");
        if let (Some(arg1), Some(arg2)) = (arg.next(), arg.next()) {
            match (arg1.parse::<usize>(), arg2.parse::<usize>()) {
                (Ok(num1), Ok(num2)) => result.push([num1, num2]),
                _ => return Err("Invalid argument"),
            }
        }
    }
    if no_args {
        match mode {
            STATS => return Err("No dice provided"),
            _ => result.push([1, 6]),
        }
    }
    Ok((result, mode))
}

fn print_help(mode: &str) {
    match mode {
        "help" => {
            println!("
            Usage: dice [mode] [args] \n
            Modes: \n
            \thelp [mode] - prints help message for [mode] \n
            \troll [dice] - rolls dice \n
            \tstats [dice] - prints stats for dice \n
            \n
            Dice: <number of dice>d<sides on dice> \n
            ");
        }
        "roll" => {
            if cfg!(feature = "roll") {
                println!("The roll feature was not compiled into this build. \n");
            }
            println!("
            Usage: dice roll [display] [dice] \n
            The assumed mode if no mode is given. \n
            Generates a random dice roll with the given dice. \n
            \n
            Display: can be sum or rolls\n
            Dice: <number of dice>d<sides on dice> \n
            ")
        }
        "stats" => {
            println!("
            Usage: dice stats [sort] [dice] \n
            Generates statistics for dice rolls. \n
            \n
            Sort: can be freq or val \n
            Dice: <number of dice>d<sides on dice> \n
            \n
            ");
        }
        _ => {}
    }
}