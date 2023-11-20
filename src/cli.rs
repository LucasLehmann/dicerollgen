use std::env;

fn main() {

}

fn print_help(mode: Option<&String>) {
    println!("Usage: dice [0] <dice> \n
    0: sort by value instead of frequency \n
    <dice>: <number of dice>d<number of sides> \n
    <number of dice>: number of dice to roll \n
    <number of sides>: number of sides on each die \n
    Example: dice 4d6 2d8");
    match mode {
        // TODO: add help for specific modes
        None => {}
    }
    
}
