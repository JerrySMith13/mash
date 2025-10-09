#![allow(dead_code)]
/*
Rules are pretty much identical to most traditional shells:
    - Every command ends either with a newline or a redirection
    - The command is invoked with the args it specifies before terminating character
    - Default behavior is to wait for the command to finish, with all stdout and stderr being routed directly to the console.
    - However, if commands are piped together, the final command in the sequence will be the only one that prints to stdout.

*/

use std::collections::HashSet;
#[derive(Debug)]
pub(crate) struct CommandParams{
    pub(crate) invoking: std::path::PathBuf,
    pub(crate) args: Vec<String>,
    pub(crate) is_builtin: bool,
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum Terminator{
    Pipe,
    EndCmd,
    None,
}


/*
Parsing

first things first: 
- Separate by single-character terminators (currently only pipe, semicolon, and newline as of rn)
- Once we have a list of commands, we simply turn it into a command and arguments, each separated by whitespace
- IF we encounter a string, we just interpret it as the next argument, allowing one to capture any character they want (escape characters for '"' is \")

for example: 
    cd  hello/ will take you to "hello/"
    cd " hello/" will try and take you to " hello/".

currently the only argument spacer is whitespace but may experiment with dynamic spacers, meaning users can specify how the tokens are interacted with 

Variables use the $ sign, and a variable name is ended by whitespace. All other characters are valid for a variable.
*/
#[derive(PartialEq, Eq)]
enum TokenizeMode{
    Arg,
    String,
    Command,
    Var,
}
#[derive(Debug)]
pub struct ParseErr{
    kind: ParseErrKind,
    message: String,
    index: usize,
}
#[derive(Debug)]
pub enum ParseErrKind{
    InvalidChar,
    InvalidTerminator,
    TerminatorTooEarly
}
type ParseResult = Result<Vec<(CommandParams, Terminator)>, ParseErr>;
impl CommandParams{

    fn clear_command() -> CommandParams{
        let new_cmd: CommandParams = CommandParams { 
            invoking: "".into(),
            args: Vec::new(),
            is_builtin: false
        };
        return new_cmd;
    }
    
    // Argument terminators are whitespace, while command terminators are the pipe |, newline \n, or simply ;
    pub fn tokenize<T>(input: T) -> ParseResult
        where T: Into<String>
    {
        return Ok(Vec::new());
    }
    
    fn parse_str(raw: &mut Vec<char>) -> String {
        let mut new_str: String = String::with_capacity(raw.len());

        let mut index = 0;
        while index < raw.len(){
            let current = raw[index];
            if current == '\\'{
                let (char, offset) = Self::decode_escape(index, raw);
                index += offset;
                new_str.push(char);
            }
            new_str.push(current);
            index += 1;
            
        }
        raw.clear(); 
        return new_str;

    }

    fn decode_escape(index: usize, raw: &Vec<char>) -> (char, usize){
        let current_char = raw[index];
        match current_char{
            '"' => ('"', 1),
            '\\' => ('\\', 1),
            _ => ('\0', 0),
        }
    }
    fn parse_arg(raw: &mut Vec<char>) -> String{
        let mut string = String::with_capacity(raw.len());
        raw.iter().for_each(
            |c| {string.push(*c);}
        );
        raw.clear();
        return string;
    }

    fn get_env(raw: &mut Vec<char>) -> String{
        raw.clear();
        return String::from("");
    }

    fn is_terminator(c: char) -> bool{
        const TERMINATORS: &str = "|;\n";
        TERMINATORS.contains(c)
       
    }
}