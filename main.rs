mod token;
mod tokenizer;
mod parser;
mod statement;

use std::io::{self, Write};
use tokenizer::Tokenizer;
use parser::Parser;

fn main() {
    //instructions on how to use the program
    println!("Simple SQL Parser CLI (multiline)");
    println!("Enter SQL statements ending with `;`. Press Ctrl+Z to exit.\n");

    let stdin = io::stdin();
    let mut buffer = String::new(); //collect multiple lines until complete statement formed

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); //flush stdout so that its printed immediately

        let mut line = String::new();
        
        //read input, if reading fails exit loop
        if stdin.read_line(&mut line).is_err() {
            break; //read input, if reading fails exit loop
        }

        //exit if cntrl+z is pressed on empty line
        if line.trim().is_empty() && buffer.trim().is_empty() {
            break;
        }

        buffer.push_str(&line); //add new line to input buffer

        //check if the sql statement complete or not
        if buffer.trim_end().ends_with(';') {
            let tokens: Vec<_> = Tokenizer::new(&buffer).collect(); //tokenizing the entire sql statement
            let mut parser = Parser::new(tokens); //new parser using list of tokens
            
            //parse the sql statement, if it can print, if it cannot show error
            match parser.parse_statement() {
                Ok(stmt) => println!("{:#?}", stmt),
                Err(err) => eprintln!(" Error: {}", err),
            }

            buffer.clear(); //clear buffer for next input
        }
    }

    println!("\n Goodbye!"); //sayonara
}