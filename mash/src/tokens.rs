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
        let mut io_out: Option<Stdio> = None;
        let mut all_spawned: VecDeque<Child> = VecDeque::with_capacity(commands.len());

        for i in 0..commands.len(){
            let command: &CommandParams;
            let terminator_before: &Terminator;
            let terminator_after: &Terminator;
            if i == 0{
                terminator_before = &Terminator::None;
            }
            else { terminator_before = &commands[i - 1].1}
            terminator_after = &commands[i].1;
            command = &commands[i].0;

            let mut input_from: Option<Stdio> = None;
            let mut output_to: Option<Stdio> = None;

            if *terminator_before == Terminator::Pipe {
                input_from = io_out.take();
            }

            if *terminator_after == Terminator::Pipe {
                output_to = Some(Stdio::piped());
            }

            let mut child = command.exec_self::<Stdio>(input_from, output_to)?;

            if *terminator_after == Terminator::Pipe {
                io_out = Some(child.stdout.take().expect("Error grabbing a process's stdout").into());
            } else {
                io_out = None;
            }
            
            all_spawned.push_back(child);
        }
        //EXIT SIGNAL HERE
        'continuous: loop{
            let mut i = 0;
            while i < all_spawned.len() {
                if all_spawned[i].try_wait()?.is_some() {
                all_spawned.remove(i);
                } 
                else {
                    i += 1;
                }
            }
        }
        return Ok(());
    }

    /// THIS COMMAND ASSUMES THE INPUT_FROM PIPE IS STILL ACTIVE ON CALL-TIME
    fn exec_self<T>(&self, input_from: Option<T>, output_to: Option<T>) -> std::io::Result<Child>
    where T: Into<Stdio>
    {
        let mut exec = Command::new(&self.invoking);
        exec.args(&self.args);

        if let Some(stdin) = input_from {
            exec.stdin(stdin.into());
        }

        if let Some(stdout) = output_to {
            exec.stdout(stdout.into());
        }

        exec.spawn()
    }
}

#[cfg(test)]
mod tests{
    use std::path::PathBuf;

    use crate::tokens::{CommandParams, Terminator};

    
    #[test]
    fn test_commands(){
        let SampleLs: CommandParams = CommandParams{
            invoking: PathBuf::from("/usr/bin/ls"),
            args: vec!["-l".to_string()]
        };

        let SampleGrep: CommandParams = CommandParams { 
            invoking: PathBuf::from("/usr/bin/grep"), 
            args: vec!["rs".to_string()]
        };

        let command_list = vec![(SampleLs, Terminator::None), (SampleGrep, Terminator::Pipe)];
        CommandParams::execute_statement(command_list).unwrap();
    }
}
