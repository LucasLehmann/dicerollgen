use std::env;
use std::process::ExitCode;

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

fn main() -> ExitCode {
    match get_args() {
        Ok((args, mode)) => {
            #[cfg(debug_assertions)]
            {
                println!("Args: {:?}", args);
                println!("Mode: {:X}", mode);
            }
            match mode & 0xF0 {
                STATS => stats(args, mode),
                HELP => print_help("help"),
                ROLL => roll(args, mode),
                _ => {}
            }
        }
        Err(err) => {
            print_help("help");
            println!("Error: {}", err);
            return ExitCode::FAILURE;
        }
    };
    ExitCode::SUCCESS
}

#[cfg(not(feature = "roll"))]
fn roll(_args: Vec<[usize; 2]>, _mode: usize) {
    println!("The roll feature was not compiled into this build. \n");
    print_help("roll");
}

#[cfg(feature = "roll")]
fn roll(dice: Vec<(usize, usize)>, mode: usize) {
    match mode {
        ROLL_SUM => roll_sum(dice),
        ROLL_ROLLS => roll_rolls(dice),
        ROLL_SUM_ROLLS | _ => {
            roll_sum_rolls(dice);
        }
    }
}

#[cfg(feature = "roll")]
fn roll_sum(dice: Vec<(usize, usize)>) {
    let mut rng = rand::thread_rng();
    let mut sum: usize = 0;
    for die in dice {
        for _ in 0..die.0 {
            sum += rng.gen_range(1..=die.1);
        }
    }
    println!("The sum of the rolled die is: {}", sum);
}

#[cfg(feature = "roll")]
fn roll_rolls(dice: Vec<(usize, usize)>) {
    let mut rng = rand::thread_rng();
    let mut rolls: Vec<usize> = Vec::new();
    for die in dice {
        rolls.extend((0..die.0).map(|_| rng.gen_range(1..=die.1)));
    }
    println!("The die you rolled are {:?}", rolls);
}

#[cfg(feature = "roll")]
fn roll_sum_rolls(dice: Vec<(usize, usize)>) {
    roll_sum(dice.clone());
    roll_rolls(dice);
}

#[cfg(not(feature = "stats"))]
fn stats(_args: Vec<[usize; 2]>, _mode: usize) {
    println!("The stats feature was not compiled into this build. \n");
    print_help("stats");
}

#[cfg(feature = "stats")]
fn stats(dice: Vec<(usize, usize)>, mode: usize) {
    let mut rolls: Vec<Vec<usize>> = Vec::new();
    for die in dice {
        for _ in 0..die.0 {
            rolls.push((1..=die.1).collect());
        }
    }
    let product: HashMap<usize, usize> = cartesian_product(rolls).iter().map(|vec| vec.iter().sum())
    .fold(HashMap::new(), |mut map, sum| {
        *map.entry(sum).or_insert(0) += 1;
        map
    }).into_iter().collect();
    let mut product: Vec<(usize, usize)> = product.into_iter().collect();
    match mode {
        STATS_VAL => {
            product.sort_by(|a, b| a.0.cmp(&b.0));
            product.iter().for_each(|(val, freq)| {
                println!("{}: {}", val, freq);
            });
        }
        STATS_FREQ | _ => {
            product.sort_by(|a, b| a.1.cmp(&b.1));
            product.iter().for_each(|(val, freq)| {
                println!("{:03}: {:02}", val, freq);
            });
        }
    }
}

#[cfg(feature = "stats")]
fn cartesian_product(pools: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = vec![vec![]];
    for pool in pools {
        let mut new_result: Vec<Vec<usize>> = Vec::new();
        for item in pool {
            for vec in &result {
                let mut vec = vec.clone();
                vec.push(item);
                new_result.push(vec);
            }
        }
        result = new_result;
    }
    #[cfg(debug_assertions)]
    {
        println!("Cartesian product: {:?}", result);
    }
    return result;
}

fn get_args() -> Result<(Vec<(usize, usize)>, usize), &'static str> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mode: usize;
    mode = match args.get(0).unwrap_or(&"".to_string()).as_str() {
        "stats" => {
            args.remove(0);
            match args.get(0).unwrap_or(&"".to_string()).as_str() {
                "freq" => {
                    args.remove(0);
                    STATS_FREQ
                }
                "val" => {
                    args.remove(0);
                    STATS_VAL
                }
                _ => STATS,
            }
        }
        "help" => {
            args.remove(0);
            match args.get(0).unwrap_or(&"".to_string()).as_str() {
                "stats" => {
                    args.remove(0);
                    HELP_STATS
                }
                "roll" => {
                    args.remove(0);
                    HELP_ROLL
                }
                _ => HELP,
            }
        }
        "roll" => {
            args.remove(0);
            match args.get(0).unwrap_or(&"".to_string()).as_str() {
                "sum" => {
                    args.remove(0);
                    ROLL_SUM
                }
                "rolls" => {
                    args.remove(0);
                    ROLL_ROLLS
                }
                "sum_rolls" => {
                    args.remove(0);
                    ROLL_SUM_ROLLS
                }
                _ => ROLL,
            }
        }
        _ => ROLL,
    };
    let mut dice = Vec::new();
    let mut no_args = true;
    for arg in args {
        no_args = false;
        let mut arg = arg.split("d");
        if let (Some(arg1), Some(arg2)) = (arg.next(), arg.next()) {
            match (arg1.parse::<usize>(), arg2.parse::<usize>()) {
                (Ok(num1), Ok(num2)) => dice.push((num1, num2)),
                _ => return Err("Invalid argument"),
            }
        } else {
            return Err("Invalid argument");
        }
    }
    if no_args {
        match mode {
            STATS => return Err("No dice provided"),
            _ => dice.push((1, 6)),
        }
    }
    return Ok((dice, mode));
}

fn print_help(mode: &str) {
    match mode {
        "help" => {
            println!(
                "
            Usage: dice [mode] [args] \n
            Modes: \n
            \thelp [mode] - prints help message for [mode] \n
            \troll [dice] - rolls dice \n
            \tstats [dice] - prints stats for dice \n
            \n
            Dice: <number of dice>d<sides on dice> 
            \n
            Examples: \n
            \tdice roll 2d6 - rolls two six-sided dice \n
            \tdice - rolls one six-sided die \n
            \n"
            );
            if cfg!(feature = "stats") {
                println!("The stats feature was not compiled into this build. \n");
            }
            if cfg!(feature = "roll") {
                println!("The roll feature was not compiled into this build. \n");
            }
        }
        "roll" => {
            if cfg!(feature = "roll") {
                println!("The roll feature was not compiled into this build. \n");
            }
            println!(
                "
            Usage: dice roll [display] [dice] \n
            The assumed mode if no mode is given. \n
            Assumes 1d6 if no dice are given. \n
            Generates a random dice roll with the given dice. \n
            \n
            Display: can be sum, rolls or sum_rolls \n
            Dice: <number of dice>d<sides on dice> 
            \n"
            )
        }
        "stats" => {
            if cfg!(feature = "stats") {
                println!("The stats feature was not compiled into this build. \n");
            }
            println!(
                "
            Usage: dice stats [sort] [dice] \n
            Generates statistics for dice rolls. \n
            \n
            Sort: can be freq or val \n
            Dice: <number of dice>d<sides on dice> \n
            \n"
            );
        }
        _ => print_help("help"),
    }
}
