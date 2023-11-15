use rand::Rng;
use std::env;

fn main() {
    //Collect command line arguments
    let args: Vec<String> = env::args().collect();
    
    //Get number of command line arguments with handling
    let mut num_of_args = 0;
    match get_num_of_arguments(&args) {
        Ok(user_args) => {
            num_of_args = user_args;
        },
        Err(err) => {println!("Error: {}", err);},
    };

    //Loop through command line arguments
    //Tried to subract the actually command but this shit doesn't like that
    for n in 1..num_of_args {
        let current_arg: &str = &args[n];
        match current_arg {
            "-6" => { roll(6) }
            "-20" => { roll(20) }
            _ => { println!("Hasn't been written yet"); }
        }
    }
}

fn get_num_of_arguments(args: &Vec<String>) -> Result<usize, String>{
    //Get command line arguments 
    if args.len() < 2{
        return Err("Not enough arguments".to_string());
    }
    else {
        return Ok(args.len());
    }
}

fn roll(size: u32) {
    let val: u32 = rand::thread_rng().gen_range(1..=size);
    println!("You rolled a {val}");
}