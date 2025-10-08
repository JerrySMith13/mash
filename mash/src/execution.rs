use crate::tokens::*;
use std::{collections::VecDeque, io, process::{Child, Command}};
use std::process::Stdio;
use crate::builtins::exec_builtin;

impl CommandParams{
    pub fn execute_statement(commands: Vec<(CommandParams, Terminator)>) -> io::Result<()>{
        
        let mut all_spawned: VecDeque<Child> = VecDeque::with_capacity(commands.len());
        let mut last_output: Option<Stdio> = None;
        for i in 0..commands.len(){
            let command = &commands[i].0;
            let terminator_after = &commands[i].1;;
            //Referenced for borrow checker, so it's not dropped by calling functions to it
            let last_output = &mut last_output;
            
            
            
            if command.is_builtin{
                //Verbose errors
                if *terminator_after == Terminator::Pipe{
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput, 
                        "Shell builtins have no output, and cannot be piped"));
                }
                exec_builtin(command)?;
                continue;
            }

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

    fn execute_builtin(&self) -> io::Result<()>{
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
            args: vec!["/Users/jerem-mac/djkw.circ".to_string()],
            is_builtin: false,
        };

        let sample_grep: CommandParams = CommandParams { 
            invoking: PathBuf::from("/usr/bin/grep"), 
            args: vec!["text".to_string()],
            is_builtin: false,
        };

        let command_list = vec![(sample_ls, Terminator::Pipe), (sample_grep, Terminator::EndCmd)];
        CommandParams::execute_statement(command_list).unwrap();
    }
}
