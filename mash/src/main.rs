use crate::tokens::CommandParams;
mod tokens;
mod builtins;
mod execution;
fn main() {
    let command = "/bin/echo hello world | /usr/bin/tee -a /Users/jerem-mac/projects/mash/test.txt; /bin/echo \"bye ig\"";
    let tokenized = CommandParams::tokenize(command).unwrap();
    println!("{:?}", tokenized);
    CommandParams::execute_statement(tokenized).unwrap();

}
