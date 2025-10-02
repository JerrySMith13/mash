use std::{io::Read, process::{self, Command}};
use std::process::Stdio;
use std::io;
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

enum Terminator{
    Pipe,
    End,
}

impl CommandParams{
    fn execute_statement(commands: Vec<(CommandParams, Terminator)>){
        for (command, term) in commands{

        }
    }

    fn exec_self<T>(&self, input_from: Option<T>) -> std::io::Result<Stdio>
    where T: Into<Stdio>
    {
        let mut exec = &mut Command::new(self.invoking.clone());
        exec = exec.args(self.args.clone());
        if input_from.is_some(){
            exec = exec.stdin(input_from.unwrap());
        }
        match exec.spawn()?.stdout{
            Some(out) => return Ok(out.into()),
            None => return Err(io::Error::new(io::ErrorKind::BrokenPipe, "Error: spawned process's stdout couldn't be grabbed")),
        }
        

    }
}
