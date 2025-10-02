use crate::tokens::CommandParams;
use std::io;

pub fn exec_builtin(command: &CommandParams) -> io::Result<()>{
    match command.invoking.to_str()
        .ok_or(
            io::Error::new(
                io::ErrorKind::InvalidData, 
                "Invalid builtin")
            )?
    {
        "cd" => return builtin_functions::cd(&command.args),
        _ => return Err(
            io::Error::new(io::ErrorKind::Unsupported, 
            "Builtin does not exist")
        ),
    }
}

mod builtin_functions{
    use std::env::set_current_dir;
    use std::io;

    pub(super) fn cd(args: &Vec<String>) -> io::Result<()>{
        if args.len() != 1{
            return Err(io::Error::new(
                io::ErrorKind::ArgumentListTooLong, 
                "Builtin cd takes one argument, the path to change to"));
        }

        set_current_dir(args[0].clone())?;
        

        return Ok(())
    }
}