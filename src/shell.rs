use crate::print;
use crate::println;
use crate::interrupts::MAX_LEN;
use crate::commands;
// use crate::panic;

const MAX_ARGS_LEN: usize = 20;
pub const MAX_COMMAND_ARGS_LEN: usize = MAX_LEN/2;

pub fn execute(input: [u8; MAX_LEN + 1]) {
    let args_len = count_whitespaces(&input);
    let args = format_args(&input);

    // println!("{:?}", args);
    if let Ok(first_arg) = core::str::from_utf8(args[1]) {
        let mut command_args = [None; MAX_COMMAND_ARGS_LEN];

        for (i, arg) in args.into_iter().skip(2).enumerate(){
            if arg != [0] {
                command_args[i] = Some(core::str::from_utf8(arg).unwrap_or_else(|_| panic!("Cannot format args")))
            }        
        }
        // println!("{:?}", command_args);
        match first_arg{
            "\n" => println!(),
            "ping" => println!("\npong"),
            "cowsay" => commands::cowsay(command_args),
            "clear" => for i in 1..100 {println!()},
            "whoami" => commands::whoami(),
            "help" => commands::help(),
            "aboutme" => commands::aboutme(),
            _ => println!("Unknown command"),
        }
    }
}

fn format_args<'a>(input: &'a [u8; MAX_LEN + 1] ) -> [&'a [u8]; MAX_ARGS_LEN]{
    let mut args = [&input[0..1]; MAX_ARGS_LEN];
    
    let mut previous_is_ws: bool = false;
    let mut before_args: bool = true;
    let mut in_quotes: bool = false;

    let mut words_count = 1;
    
    let mut first_index: isize = -1;
    let mut last_index: isize = -1;
            
    for (i, byte) in input.into_iter().enumerate() {
        if *byte == b'"' || *byte == b'\'' {
           if in_quotes {
               in_quotes = false;

                previous_is_ws = true;

                last_index = i as isize;

                args[words_count] = &input[first_index as usize..last_index as usize ];

                words_count += 1;
                first_index = -1;
                last_index = -1;

                continue;
            }
           else {
               in_quotes = true;
           }
        }
        if *byte == b' ' && !(before_args && previous_is_ws) && first_index != -1 || *byte == 0 && first_index != -1 || (in_quotes && (*byte == b'"' || *byte == b'\'')){
            if !in_quotes {
                previous_is_ws = true;

                last_index = i as isize;

                args[words_count] = &input[first_index as usize..last_index as usize];

                words_count += 1;
                first_index = -1;
                last_index = -1;
            }
        }
        else if *byte != b' ' && first_index == -1 && *byte != 0{
           
            first_index = i as isize;
            
            before_args = false;
            previous_is_ws = false;
        }
        
    }

    args
    
}

fn count_whitespaces<'a>(input: &'a [u8; MAX_LEN + 1]) -> usize {
    let mut count: usize = 0;

    let mut previous_is_ws: bool = false;
    let mut before_args: bool = true;
    let mut in_quotes: bool = true;
    
    for byte in input.into_iter().skip(1){
        if *byte == b'"' || *byte == b'\'' {
            if in_quotes {
                in_quotes = false;
            }
            else {
                in_quotes = true;
            }
        }
        else if *byte == b' ' && !(before_args && previous_is_ws){
            if !in_quotes {
                count += 1;
                previous_is_ws = true;
            }
        }
        else if *byte != b' ' {
            before_args = false;
            previous_is_ws = false;
        }

    }
    count
}
