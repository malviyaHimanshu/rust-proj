/*
    expected functionality
    - word count 
    - find and replace
    - line numbering 
    - text wrapping
*/

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::io::Read;
use std::process;

// returns number of words
fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

// replaces the target by replacement
fn find_and_replace(text: &str, target: &str, replacement: &str) -> String {
    text.replace(target, replacement)
}

// add line numbers
fn line_numbering(text: &str) -> String {
    text.lines()
        .enumerate()
        .map(|(i, line)| format!("{} {}", i+1, line))
        .collect::<Vec<_>>()
        .join("\n")
}

// wraps the text based on max width
fn wrap_text(text: &str, max_width: usize) -> String {
    let mut result = String::new();
    for line in text.lines() {
        let mut current_line = String::new();
        // keep pushing to current line until it reaches the max width
        for word in line.split_whitespace() {
            if current_line.len() + word.len() + 1 > max_width {
                result.push_str(&current_line.trim_end());
                result.push('\n');
                current_line.clear();
            }
            current_line.push_str(word);
            current_line.push(' ');
        }
        result.push_str(&current_line.trim_end());
        result.push(' ');
    }
    result
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run <filepath>");
        process::exit(1);
    }

    let file_path = &args[1];
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    loop {
        println!("\n~ choose a operation for file {} ~", file_path);
        println!("1. word count");
        println!("2. find and replace");
        println!("3. add line numbers");
        println!("4. wrap text");
        println!("5. exit");
        print!("enter you choice : ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        match choice {
            "1" => {
                let word_count = word_count(&contents);
                println!("word count: {}", word_count);
            },
            "2" => {
                print!("enter the word to find: ");
                io::stdout().flush()?;
                let mut target = String::new();
                io::stdin().read_line(&mut target)?;
                let target = target.trim();

                print!("enter the replacement word: ");
                io::stdout().flush()?;
                let mut replacement = String::new();
                io::stdin().read_line(&mut replacement)?;
                let replacement = replacement.trim();

                contents = find_and_replace(&contents, target, replacement);
                println!("✅ replacement done.");
            },
            "3" => {
                let numbered_content = line_numbering(&contents);
                println!("with line numbers:\n{}", numbered_content);
            },
            "4" => {
                print!("enter the maximum width for text wrapping: ");
                io::stdout().flush()?;
                let mut max_width = String::new();
                io::stdin().read_line(&mut max_width)?;
                let max_width = max_width.trim().parse().expect("please enter a valid number");

                contents = wrap_text(&contents, max_width);
                println!("wrapped text:\n{}", contents);
            },
            "5" => {
                let mut file = File::create(file_path)?;
                file.write_all(contents.as_bytes())?;
                println!("content saved. exiting.");
                break;
            },
            _ => {
                println!("invalid choice, please try again!");
            }
        }
    }

    Ok(())
}
