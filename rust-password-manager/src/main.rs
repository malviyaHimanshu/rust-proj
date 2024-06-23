use std::io::{self, Write};
use rpassword::read_password;

use rust_password_manager::PasswordManager;

fn main() {
    let mut password_manager = PasswordManager::new();

    // let args: Vec<String> = env::args().collect();
    // if args.len() < 2 {
    //     println!("usage: cargo run <password_file>");
    //     process::exit(1);
    // }
    // let data_file = &args[1];


    loop {
        println!("\n~ welcome to rust password manager ~");
        println!("1. add entry");
        println!("2. remove entry");
        println!("3. list entries");
        println!("4. load from file");
        println!("5. save & quit");
        // TODO: add few more options such as generate new password
        print!("choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("failed to choice");

        match choice.trim() {
            "1" => add_entry(&mut password_manager),
            "2" => remove_entry(&mut password_manager),
            "3" => password_manager.list_entries(),
            "4" => {
                password_manager = PasswordManager::load();
                println!("loaded from file successfully");
            }
            "5" => {
                password_manager.save();
                println!("saved successfully, quitting.");
                break;
            }
            _ => println!("invalid option, please try again!")
        }
    }

}

fn add_entry(manager: &mut PasswordManager) {
    print!("service: ");
    io::stdout().flush().unwrap();
    let mut service = String::new();
    io::stdin().read_line(&mut service).expect("failed to read service");

    print!("username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("failed to read username");

    print!("password: ");
    io::stdout().flush().unwrap();
    let password = read_password().expect("failed to read password");

    manager.add_entry(service.trim().to_string(), username.trim().to_string(), password);
}

fn remove_entry(manager: &mut PasswordManager) {
    print!("service to remove: ");
    io::stdout().flush().unwrap();
    let mut service = String::new();
    io::stdin().read_line(&mut service).expect("failed to read service");

    manager.remove_entry(service.trim().to_string());
}
