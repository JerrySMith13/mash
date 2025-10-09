use crate::tokens::CommandParams;
mod tokens;
mod builtins;
mod execution;
fn main() {
    let stdin = std::io::stdin();
    let mut statement: String = String::new();
    while statement != "exit"{
        statement.clear();
        stdin.read_line(&mut statement).unwrap();
        statement.push('\n');
        println!("{}", statement);
        let tokens = CommandParams::tokenize(statement.clone());
        let full_command: Vec<(CommandParams, tokens::Terminator)>;
        match tokens{
            Ok(cmd) => {
                full_command = cmd;
            }
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        }
        match CommandParams::execute_statement(full_command){
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e);
            }

        }
    }

}
