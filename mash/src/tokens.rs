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
        let mut all_builtins: HashSet<&str> = HashSet::with_capacity(2);
        all_builtins.insert("cd");
        all_builtins.insert("set");
        all_builtins.insert("pwd");
        let input: String = input.into();

        let estimate_commands: u16 = 0;
        for i in input.chars(){
            if Self::is_terminator(i.clone()) {
                if estimate_commands.checked_add(1).is_none(){
                    break;
                };
            };
        }


        let mut full_command: Vec<(CommandParams, Terminator)> = Vec::with_capacity(estimate_commands as usize);

        
        let mut current_buf: Vec<char> = Vec::with_capacity(input.len());
        let mut mode = TokenizeMode::Command;
        //TODO: Add a way to override builtin and search $PATH
        let mut unfinished_command: CommandParams = CommandParams { 
            invoking: "".into(),
            args: Vec::new(),
            is_builtin: false
        };
        let mut invoking_set = false;

        for (index, char) in input.chars().enumerate(){
            if Self::is_terminator(char.clone()){
                if !invoking_set{
                    return Err(
                        ParseErr { 
                            kind: ParseErrKind::TerminatorTooEarly, 
                            message: "Finish command before applying terminator".into(),
                            index
                        }
                    );
                }
                if mode != TokenizeMode::Arg || mode != TokenizeMode::Command{
                    return Err(
                        ParseErr { 
                            kind: ParseErrKind::TerminatorTooEarly,
                            message: "Cannot end variable or string assignment early".into(), 
                            index,
                         }
                    )
                }
                if mode == TokenizeMode::Arg{
                    unfinished_command.args.push(Self::parse_arg(&mut current_buf));
                }
                else {
                    unfinished_command.invoking = Self::parse_arg(&mut current_buf).into();
                }
                
                let term: Terminator;
                match char{
                    '|' => {
                        term = Terminator::Pipe;

                    },
                    ';' |'\n' => {
                        term = Terminator::EndCmd;
                    }
                    _ => {
                        return Err(
                            ParseErr { 
                                kind: ParseErrKind::InvalidTerminator, 
                                message: "Internal error parsing!".to_string(),
                                index
                            }
                        );
                    }
                }
                full_command.push((unfinished_command, term));
                unfinished_command = Self::clear_command();
                current_buf.clear();
                invoking_set = false;
                mode = TokenizeMode::Command;
                continue;
            }
            match mode{
                TokenizeMode::String => {
                    if char == '"'{
                        mode = TokenizeMode::Arg;
                        let finished_str = Self::parse_str(&mut current_buf);
                        
                        if invoking_set{
                            unfinished_command.args.push(finished_str);
                        }
                        else{
                            unfinished_command.invoking = finished_str.into();
                            invoking_set = true;
                        }
                    }
                    else{
                        current_buf.push(char)
                    };
                },
                TokenizeMode::Command => {
                    let is_empty = current_buf.is_empty();
                    if char == '$' && is_empty{
                        mode = TokenizeMode::Var;
                        continue;
                    }
                    else if char == '"' && is_empty{
                        mode = TokenizeMode::Var;
                        continue;
                    }
                    else if char.is_whitespace() && !is_empty{
                        unfinished_command.invoking = Self::parse_arg(&mut current_buf).into();
                        invoking_set = true;
                        mode = TokenizeMode::Arg;
                        
                        continue;
                    }
                    else if char.is_whitespace() && is_empty{
                        continue;
                    }
                    else {
                        current_buf.push(char)
                    };
                    continue; 
                }
                TokenizeMode::Arg => {
                    if char == '"'{
                        if !current_buf.is_empty(){
                            return Err(
                                ParseErr { 
                                    kind: ParseErrKind::InvalidChar, 
                                    message: "Error: cannot use string assignment ('\"') in argument".into(),
                                    index}
                            );
                        }
                        mode = TokenizeMode::String;
                        continue;
                    }
                    else if char == '$'{
                        if !current_buf.is_empty(){
                            return Err(
                                ParseErr { 
                                    kind: ParseErrKind::InvalidChar, 
                                    message: "Error: cannot use variable dereference ('$') in argument".into(),
                                    index}
                            );
                        }
                        mode = TokenizeMode::Var;
                        continue;
                    
                    }
                    else if char.is_whitespace(){
                        if current_buf.is_empty(){
                            continue;
                        }
                        else{
                            let arg = Self::parse_arg(&mut current_buf);
                        
                            unfinished_command.args.push(arg);

                            continue;
                        }
                    }
                    else{
                        current_buf.push(char);
                        continue;
                    }
                },
                TokenizeMode::Var => {
                    if char.is_whitespace(){
                        let var_dereferenced = Self::get_env(&mut current_buf);
                        
                        if !invoking_set{
                            unfinished_command.invoking = var_dereferenced.into();
                            invoking_set = true;
                            mode = TokenizeMode::Arg;
                            continue;
                        }
                        else{
                            unfinished_command.args.push(Self::get_env(&mut current_buf));
                            mode = TokenizeMode::Arg;
                        }
                    }
                }
            }

        }
        if invoking_set == true{
            full_command.push((unfinished_command, Terminator::None));
        }
        return Ok(full_command);
    
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