#![allow(dead_code)]
/*
Rules are pretty much identical to most traditional shells:
    - Every command ends either with a newline or a redirection
    - The command is invoked with the args it specifies before terminating character
    - Default behavior is to wait for the command to finish, with all stdout and stderr being routed directly to the console.
    - However, if commands are piped together, the final command in the sequence will be the only one that prints to stdout.

*/

pub(crate) struct CommandParams{
    pub(crate) invoking: std::path::PathBuf,
    pub(crate) args: Vec<String>,
    pub(crate) is_builtin: bool,
}

#[derive(PartialEq, Eq)]
pub(crate) enum Terminator{
    Pipe,
    EndCmd,
    None,
}

impl CommandParams{
    
}
