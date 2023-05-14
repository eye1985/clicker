use std::{env, thread, time};
use std::thread::sleep;
use enigo::*;

fn main() {
    #[derive(Debug)]
    struct Arguments{
        command : String,
        value :i32,
    }

    impl Arguments {
        fn new(command: &str, value: i32) -> Arguments {
            Arguments {
                command : command.to_string(),
                value
            }
        }
    }

    let args:Vec<String> = env::args().collect();
    let mut command_list:Vec<Arguments> = vec![];

    for (index, command) in args.iter().enumerate() {
        if command == "--interval"{
            if let None = args.get(index+1) {
                println!("No value provided");
                return;
            }

            let result_value = args.get(index+1).unwrap();
            if let Err(e) = result_value.parse::<i32>() {
                println!("Your interval number is not a valid numer: {}", e);
                return;
            }

            let seconds = match  args.get(index+1).unwrap().parse() {
                Ok(s) => s,
                Err(_) => 10,
            };

            let command_item = Arguments::new(command, seconds);
            command_list.push(command_item);
        }
    }

    let mut interval = 10;
    let mut enigo = Enigo::new();

    for command in command_list{
        println!("Command {}", command.command);
        println!("Value is: {}", command.value);
        interval = command.value;
    }

    let duration = time::Duration::from_secs(interval as u64);
    loop {
        enigo.mouse_down(MouseButton::Left);
        enigo.mouse_up(MouseButton::Left);
        println!("Left mouse clicked");
        thread::sleep(duration);
    }
}
