use colored::{ColoredString, Colorize};
use std::{
    io::{stdin, stdout, Read, Write},
    process::Command,
};
mod folders;
mod todo_item;
mod todo_list;

fn main() {
    let mut list = todo_list::TodoList::new();
    let mut folders = folders::Folders::new();
    let mut current_folder: Option<folders::Folder> = None;
    list.get_conn();
    list.load_all_todos();
    let mut instruction = String::new();
    display_start();
    reset(&mut instruction, None);
    loop {
        println!(
            "== {} ==",
            current_folder
                .clone()
                .unwrap_or(folders::Folder::new("Default".to_string()))
        );
        println!("{}", list);

        std::io::stdin()
            .read_line(&mut instruction)
            .expect("Could not read instruction");
        instruction = instruction.trim().to_string();
        let intermediate = instruction.clone();
        let tokens: Vec<&str> = intermediate.split(' ').collect();
        match tokens[0] {
            "f" => {
                if tokens.len() > 1 {
                    let folder = tokens[1..].join(" ");
                    if folder.is_empty() {
                        reset(&mut instruction, Some("Please enter a folder name".red()));
                    } else {
                        let new_folder = folders.add_folder(folder);
                        current_folder = Some(new_folder);
                    }
                } else {
                    println!("{}", folders);
                    pause();
                }
            }
            "switch" => {
                if tokens.len() > 1 {
                    let folder = tokens[1..].join(" ");
                    if folder.is_empty() {
                        reset(&mut instruction, Some("Please enter a folder name".red()));
                    } else {
                        let index = folders
                            .list
                            .iter()
                            .position(|f| f.name == folder)
                            .unwrap_or(0);
                        current_folder = folders.get_val(index).cloned();
                    }
                } else {
                    reset(&mut instruction, Some("Please enter a folder name".red()));
                }
            }
            "a" => {
                let item = tokens[1..].join(" ");
                if item.is_empty() {
                    reset(&mut instruction, Some("Please enter an item".red()));
                } else {
                    list.add_todo(item);
                }
            }
            "m" => handle_indexed_instruction(&tokens, &mut instruction, |index| {
                !list.mark_done(index)
            }),
            "!m" => handle_indexed_instruction(&tokens, &mut instruction, |index| {
                !list.mark_not_done(index)
            }),
            "d" => {
                if tokens.len() > 2 {
                    reset(&mut instruction, Some("Please enter an index".red()));
                } else {
                    let first = tokens[1];
                    if first == "all" {
                        list.remove_all();
                        instruction.clear();
                    } else {
                        handle_indexed_instruction(&tokens, &mut instruction, |index| {
                            !list.remove(index)
                        });
                    }
                }
            }
            "i" => handle_indexed_instruction(&tokens, &mut instruction, |index| {
                !list.mark_important(index)
            }),
            "!i" => handle_indexed_instruction(&tokens, &mut instruction, |index| {
                !list.mark_unimportant(index)
            }),
            "clean" => list.remove_completed(),
            "help" => {
                reset(&mut instruction, None);
                print_help();
                instruction.clear();
                pause();
            }
            "exit" => {
                reset(&mut instruction, None);
                break;
            }
            _ => {
                list.add_todo(instruction.clone());
            }
        }
        reset(&mut instruction, None);
    }
}

fn handle_indexed_instruction(
    tokens: &Vec<&str>,
    mut instruction: &mut String,
    mut function: impl FnMut(usize) -> bool,
) {
    if tokens.len() < 2 {
        reset(&mut instruction, Some("Please enter an index".red()));
    } else {
        let index = tokens[1].parse::<usize>();
        if index.is_err() {
            reset(&mut instruction, Some("Please enter a valid index".red()));
        } else if function(index.unwrap()) {
            reset(&mut instruction, Some("Please enter a valid index".blue()));
        }
    }
}

fn print_help() {
    println!("Commands:");
    println!("{}{}", "\tfolders".red(), " - Display all folders");
    println!("{}{}", "\tfolders <folder>".red(), " - Switch to a folder");
    println!("{}{}", "\ta <item>".red(), " - (a)dd an item to the list");
    println!("{}{}", "\tm <index>".red(), " - (m)ark an item as done");
    println!(
        "{}{}",
        "\t!m <index>".red(),
        " - (m)ark an item as not done"
    );
    println!(
        "{}{}",
        "\td <index>".red(),
        " - (d)elete an item from the list"
    );
    println!(
        "{}{}",
        "\td all".red(),
        " - (d)elete all items from the list"
    );
    println!(
        "{}{}",
        "\ti <index>".red(),
        " - Mark an item as (i)mportant"
    );
    println!(
        "{}{}",
        "\t!i <index>".red(),
        " - Mark an item as un(i)mportant"
    );
    println!(
        "{}{}",
        "\tclean".red(),
        " - Remove all completed items from the list"
    );
    println!("{}{}", "\texit".red(), " - Exit the program");
}

fn reset(instruction: &mut String, message: Option<ColoredString>) {
    instruction.clear();
    Command::new("clear")
        .status()
        .expect("Failed to clear terminal");

    if let Some(m) = message {
        println!("{}", m);
        pause();
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn display_start() {
    Command::new("clear")
        .status()
        .expect("Failed to clear terminal");
    print!(
        "{}",
        "    ___________________  ________   ________    .____    .___  ____________________
    "
        .red()
        .bold()
    );
    print!(
        "{}",
        "\\__    ___/\\_____  \\ \\______ \\  \\_____  \\   |    |   |   |/   _____/\\__    ___/
    "
        .red()
        .bold()
    );
    print!(
        "{}",
        "  |    |    /   |   \\ |    |  \\  /   |   \\  |    |   |   |\\_____  \\   |    |   
    "
        .red()
        .bold()
    );
    print!(
        "{}",
        "  |    |   /    |    \\|    `   \\/    |    \\ |    |___|   |/        \\  |    |   
    "
        .red()
        .bold()
    );
    print!(
        "{}",
        "  |____|   \\_______  /_______  /\\_______  / |_______ \\___/_______  /  |____|   
    "
        .red()
        .bold()
    );
    println!(
        "{}",
        "                   \\/        \\/         \\/          \\/           \\/            "
            .red()
            .bold()
    );
    println!(
        "{}",
        "-----------------------------------------------------------------------------------------"
            .red()
    );
    pause();
}
