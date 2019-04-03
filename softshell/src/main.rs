use std::env; 
use std::io::{stdin, stdout, Write}; 
use std::path::Path; 
use std::process::{Child, Command, Stdio};

fn main() {
    loop { 

        // Use '$' as prompt 
        print!("# ");
        stdout().flush().unwrap(); // need to explictly flush to make sure it prints before read_line executes

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        
        // Everything else after first whitespace will be command args
        let mut parts = input.trim().split_whitespace(); 
        let command = parts.next().unwrap(); 
        let args = parts; 

        match command { 
            "cd" => { 
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir); 
                if let Err(e) = env::set_current_dir(&root) { 
                    eprintln!("error: {}", e); 
                }
            }, 
            "exit" => return,
            command => { 
                let mut child = Command::new(command).args(args).spawn();
                
                // handle errors / typos:
                match child { 
                    Ok(mut child) => { child.wait(); }, 
                    Err(e) => { eprintln!("error: {}", e); }
                };
            }
        }
    }
}
