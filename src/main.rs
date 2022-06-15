mod notebook;

use crate::notebook::*;
use std::{
    io::{self, Write},
    num::ParseIntError,
};

#[derive(Debug)]
enum IoOrSqlError {
    IoError(std::io::Error),
    SqlError(rusqlite::Error),
}

fn handle_io_error(e: std::io::Error) -> IoOrSqlError {
    IoOrSqlError::IoError(e)
}
fn handle_sql_error(e: rusqlite::Error) -> IoOrSqlError {
    IoOrSqlError::SqlError(e)
}

fn example_code() -> rusqlite::Result<()> {
    print!("You can see that the data persists across multiple runs if you just run this multiple times in a row.\n`mut` Notebook is required for write operations\n\n");
    let mut nb = Notebook::new(&"./notebook.db3".to_string()).unwrap();

    let first_id = nb.insert(&Note {
        title: "First title".to_string(),
        text: "first text\n ladedadeda".to_string(),
    })?;
    println!("Inserted a note! here it is: {:?}", nb.get_note(first_id)?);

    let num_updated = nb.update_note(
        first_id,
        &Note {
            title: "hey look mom I updated a note".to_string(),
            text: "fancy dancy text".to_string(),
        },
    )?;

    println!("updated {:} notes", num_updated);
    let second_id = nb.insert(&Note {
        title: "Another title!!!".to_string(),
        text: "Aren't I cool😊".to_string(),
    })?;

    println!("Just inserted another one! Let's show all the notes we've got:");

    let notes = nb.get_all_notes()?;
    for note in notes {
        println!("{:?}", note);
    }

    println!("Time to delete the second one with id {:}", second_id);
    let num_deleted = nb.delete_note(second_id)?;

    println!(
        "Tada! deleted {:}. here's all the notes we've got:",
        num_deleted
    );
    let notes = nb.get_all_notes()?;
    for note in notes {
        println!("{:?}", note);
    }

    Ok(())
}

fn insert_from_cli(nb: &mut Notebook, input: &String) -> std::result::Result<(), IoOrSqlError> {
    let mut text = String::new();
    let title = input
        .trim_end()
        .split_once(" ")
        .map(|s| s.1)
        .unwrap_or_default()
        .to_string();
    io::stdin()
        .read_line(&mut text)
        .map_err(handle_io_error)
        .and_then(|_| {
            nb.insert(&Note { title, text })
                .map(|_| ())
                .map_err(handle_sql_error)
        })
}

fn parse_id_from_command(input: &String) -> Option<Result<i64, ParseIntError>> {
    input
        .trim_end()
        .split_once(" ")
        .map(|s| s.1)
        .map(|s| s.parse::<i64>())
}

fn update_from_cli(nb: &mut Notebook, input: &String) -> std::result::Result<(), IoOrSqlError> {
    let mut new_text = String::new();
    match parse_id_from_command(input) {
        Some(Ok(id)) => nb.get_note(id).map_err(handle_sql_error).and_then(|existing_note| {
            println!("{:1} previously said:\n--{:2}\nnew text:", existing_note.title, existing_note.text);
            io::stdout()
                .flush()
                .map_err(handle_io_error)
                .and_then(|_| io::stdin().read_line(&mut new_text).map_err(handle_io_error))
                .and_then(|_| 
                    nb.update_note(id, &Note{title:existing_note.title, text:new_text})
                        .map_err(handle_sql_error)
                        .map(|_| ())
                )
        }),
        _ => Ok(println!(
            "invalid input to update. Try `update [noteId]` where noteId is a number. You will be prompted for the new text. Can't change title"
        )),
    }
}

fn delete_from_cli(nb: &mut Notebook, input: &String) -> std::result::Result<(), IoOrSqlError> {
    match parse_id_from_command(input) {
        Some(Ok(id)) => nb
            .delete_note(id)
            .map_err(handle_sql_error)
            .map(|r| println!("Deleted {:} row(s)", r)),
        _ => Ok(println!(
            "invalid input to delete. Try `delete [noteId]` where noteId is a number"
        )),
    }
}

fn print_one_note(nb: &Notebook, input: &String) -> std::result::Result<(), IoOrSqlError> {
    match parse_id_from_command(input) {
        Some(Ok(id)) => nb
            .get_note(id)
            .map_err(handle_sql_error)
            .map(|v| println!("{:1}\n\t{:2}", v.title, v.text)),
        _ => Ok(println!("invalid input to get. Try `get [noteId]`")),
    }
}

fn main() {
    println!(
        "Welcome to your persistant notebook with Rust and Sqlite3! type `help` to get started"
    );
    let mut nb = Notebook::new(&"./notebook.db3".to_string()).unwrap();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let operation_success = match input.as_str().to_lowercase().split_whitespace().next() {
            Some("quit") | Some("exit") => {
                println!("Ok, goodbye! 👋");
                break;
            }
            Some("all") => nb.print_all_notes().map_err(handle_sql_error),
            Some("get") => print_one_note(&mut nb, &input),
            Some("insert") => insert_from_cli(&mut nb, &input),
            Some("delete") => delete_from_cli(&mut nb, &input),
            Some("update") => update_from_cli(&mut nb, &input),
            Some("example") => example_code().map_err(handle_sql_error),
            Some("help") | _ => Ok(println!("Usage: This is a mini database that runs locally.\n  `help` to see this message.\n  `exit` or `quit` to leave this dialogue\n  `insert [title]` and then be prompted one line of text.\n  `all` lists all notes and their IDs\n  `get [id]` prints the note with the specified ID\n  `delete [id]` removes the specified note from the database.\n  `update [id]` prints the specified note and changes the text to what you input")),
        };
        match operation_success {
            Err(e) => println!(
                "Oops! That threw an error😢: {:?}. Try something different.",
                e
            ),
            _ => (),
        }
    }
}
