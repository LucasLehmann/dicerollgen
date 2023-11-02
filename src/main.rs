use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut list: Vec<String> = Vec::new();
    if args.len() == 0 {
        println!("Usage: dice [0] <dice> \n
        0: sort by value instead of frequency \n
        <dice>: <number of dice>d<number of sides> \n
        <number of dice>: number of dice to roll \n
        <number of sides>: number of sides on each die \n
        Example: dice 4d6 2d8");
        return;
    }
    let mut sorted: bool = true;
    if args[0] == "0" {
        sorted = false;
        args.remove(0);
    }
    if args.len() > 0 {
        let dice: Vec<[usize; 2]> = handle_input(args);
        let pools: Vec<Vec<usize>> = generate_rolls(dice);
        let results = roll_results(pools, sorted);
        list = result_list(results);
    }
    println!("Dice:");
    for roll in list {
        println!("{}", roll);
    }
}

fn handle_input(input: Vec<String>) -> Vec<[usize; 2]> {
    let mut dice: Vec<[usize; 2]> = Vec::new();
    for arg in input {
        let mut split = arg.split("d");
        let num: usize = split.next().unwrap().parse().unwrap();
        let sides: usize = split.next().unwrap().parse().unwrap();
        dice.push([num, sides]);
    }
    dice
}

fn generate_rolls(dice: Vec<[usize; 2]>) -> Vec<Vec<usize>> {
    let mut pools: Vec<Vec<usize>> = Vec::new();
    for die in dice {
        for _ in 0..die[0] {
            let mut pool: Vec<usize> = Vec::new();
            for i in 1..=die[1] {
                pool.push(i);
            }
            pools.push(pool.clone());
        }
    }
    let mut combinations: Vec<Vec<usize>> = vec![vec![]];
    for pool in &pools {
        let mut new_result: Vec<Vec<usize>> = Vec::new();
        for combination in &combinations {
            for element in pool {
                let mut new_combination: Vec<usize> = combination.clone();
                new_combination.push(element.clone());
                new_result.push(new_combination);
            }
        }
        combinations = new_result;
    }
    combinations
}

fn roll_results(rolls: Vec<Vec<usize>>, sort: bool) -> Vec<[usize; 2]> {
    let mut results: Vec<[usize; 2]> = Vec::new();
    for roll in rolls {
        let mut sum: usize = 0;
        for die in roll {
            sum += die;
        }
        let mut found: bool = false;
        for result in &mut results {
            if result[0] == sum {
                found = true;
                result[1] += 1;
            }
        }
        if !found {
            results.push([sum, 1]);
        }
    }
    if sort {
        results.sort_by(|a, b| b[1].cmp(&a[1]));
    } else {
        results.sort_by(|a, b| a[0].cmp(&b[0]));
    }
    results
}

fn result_list(results: Vec<[usize; 2]>) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    let rolls: usize = results.iter().map(|result| result[1]).sum();
    let length: usize = results.iter().map(|result| result[0].to_string().len()).max().unwrap();
    for result in &results {
        let mut string: String = format!("{:0>length$}", result[0], length = length);
        string.push_str(": ");
        let mut percent: f64 = (result[1] as f64 / rolls as f64) * 100.0;
        percent = (percent * 100.0).round() / 100.0;
        string.push_str(&format!("{:0>5}", percent.to_string()));
        string.push_str("% ");
        string.push_str(&result[1].to_string());
        string.push_str("/");
        string.push_str(&rolls.to_string());
        list.push(string);
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_input() {
        let input = vec!["4d6".to_string()];
        let expected = vec![[4, 6]];
        assert_eq!(handle_input(input), expected);
    }
    #[test]
    fn test_generate_rolls() {
        let input = vec![[1, 1], [2, 2]];
        let expected = vec![vec![1, 1, 1], vec![1, 1, 2], vec![1, 2, 1], vec![1, 2, 2]];
        assert_eq!(generate_rolls(input), expected);
    }
    #[test]
    fn test_roll_results() {
        let input = vec![vec![1, 1, 1], vec![1, 1, 2], vec![1, 2, 1], vec![1, 2, 2]];
        let expected = vec![[3, 1], [4, 2], [5, 1]];
        assert_eq!(roll_results(input, true), expected);
    }
    #[test]
    fn test_result_list() {
        let input = vec![[3, 2], [4, 2], [5, 1]];
        let expected = vec![
            "3: 1/5".to_string(),
            "4: 2/5".to_string(),
            "5: 1/4".to_string(),
        ];
        assert_eq!(result_list(input), expected);
    }
}