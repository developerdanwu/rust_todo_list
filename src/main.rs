use std::io;

fn main() {
    let mut program_running = true;
    let mut todos: Vec<String> = vec![];
    while program_running {
        println!(
            "\
        press exit to exit the program\n\
        press add to add a todo\n\
        press remove to remove a todo\n\
        press list to list all todos\n\
        "
        );
        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(result) => {
                println!("doing your bidding")
            }
            Err(e) => println!("error: {}", e),
        }

        let trimmed_command = command.trim();
        match trimmed_command {
            "add" => {
                println!("what do you want to add?");
                let mut new_todo = String::new();
                match io::stdin().read_line(&mut new_todo) {
                    Ok(_result) => todos.push(new_todo.trim().to_string()),
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            }
            "exit" => {
                println!("Exiting the program! Peace bitches");
                program_running = false
            }
            "list" => {
                println!("Print the todos for me {:?}", todos)
            }
            "remove" => {
                println!("removing last todo...");
                match todos.pop() {
                    Some(option) => {
                        println!("{} has been removed", option)
                    }
                    None => {
                        println!("an error has occurred when removing the todo")
                    }
                }
            }
            _ => {
                println!("{} is not a recognised command", trimmed_command);
            }
        }
    }
}
