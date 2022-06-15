mod notebook;
use crate::notebook::*;

fn main() -> Result<()> {
    print!("You can see that the data persists across multiple runs if you just run this multiple times in a row.\n`mut` Notebook is required for write operations\n\n");
    let mut nb = Notebook::new(&"./notebook.db3".to_string())?;

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
