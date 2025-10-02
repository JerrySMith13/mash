#![allow(dead_code)]

use std::{collections::VecDeque, io, process::{Child, Command}};
use std::process::Stdio;

/*
Rules are pretty much identical to most traditional shells:
    - Every command ends either with a newline or a redirection
    - The command is invoked with the args it specifies before terminating character
    - Default behavior is to wait for the command to finish, with all stdout and stderr being routed directly to the console.
    - However, if commands are piped together, the final command in the sequence will be the only one that prints to stdout.

*/

struct CommandParams{
    invoking: std::path::PathBuf,
    args: Vec<String>
}

#[derive(PartialEq, Eq)]
enum Terminator{
    Pipe,
    EndCmd,
    None,
}

impl CommandParams{
    fn execute_statement(commands: Vec<(CommandParams, Terminator)>) -> io::Result<()>{
        
        let mut all_spawned: VecDeque<Child> = VecDeque::with_capacity(commands.len());
        let mut last_output: Option<Stdio> = None;
        for i in 0..commands.len(){
            let command: &CommandParams;
            let terminator_after: &Terminator;
            //Referenced for borrow checker, so it's not dropped by calling functions to it
            let last_output = &mut last_output;
            terminator_after = &commands[i].1;
            command = &commands[i].0;

            let mut executable = &mut Command::new(&command.invoking);
            executable = executable.args(&command.args);
            if let Some(stdin) = last_output.take(){
                executable = executable.stdin(stdin);
            }

            
            if *terminator_after == Terminator::Pipe{
                executable = executable.stdout(Stdio::piped());
            }
            else {
                executable = executable.stdout(Stdio::inherit());
            }
            let mut child = executable.spawn()?;
            if *terminator_after == Terminator::Pipe{
                let stdout = child.stdout.take().unwrap();
                *last_output = Some(Stdio::from(stdout));
            }
            else {
                *last_output = None;
            }
            all_spawned.push_back(child);        
        }
        //EXIT SIGNAL HERE
        'outer: loop{
            let mut i = 0;
            while i < all_spawned.len() {
                if all_spawned[i].try_wait()?.is_some() {
                all_spawned.remove(i);
                } 
                else {
                    i += 1;
                }
            }
            if all_spawned.len() == 0{
                break 'outer;
            }
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests{
    use std::path::PathBuf;

    use crate::tokens::{CommandParams, Terminator};

    
    #[test]
    fn test_commands(){
        let sample_ls: CommandParams = CommandParams{
            invoking: PathBuf::from("/bin/cat"),
            args: vec!["/Users/jerem-mac/djkw.circ".to_string()]
        };

        let sample_grep: CommandParams = CommandParams { 
            invoking: PathBuf::from("/usr/bin/grep"), 
            args: vec!["text".to_string()]
        };

        let command_list = vec![(sample_ls, Terminator::Pipe), (sample_grep, Terminator::EndCmd)];
        CommandParams::execute_statement(command_list).unwrap();
    }
}
