use rusqlite::{named_params, Connection, Result};
use std::fmt;

pub struct Notebook {
    conn: Connection,
}

#[derive(Debug)]
pub struct Note {
    pub title: String,
    pub text: String,
}

#[derive(Debug)]
pub struct NoteRow{
    pub id: i64,
    pub title: String,
    pub text: String,
}

impl fmt::Display for NoteRow{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:1}\t| {:2}\n\t|\t{:3}", self.id, self.title, self.text)
    }
}

impl Notebook {
    pub fn new(path: &String) -> Result<Notebook> {
        let conn = Connection::open(path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS notebook (
                id              INTEGER PRIMARY KEY,
                title           TEXT NOT NULL,
                text            TEXT
                );",
            [],
        )?;

        Ok(Notebook { conn })
    }

    //returns the ROWID aka primary key value of the row it just inserted
    pub fn insert(&mut self, note: &Note) -> Result<i64> {
        let mut stmt = self
            .conn
            .prepare_cached("INSERT INTO notebook(title, text) VALUES(:title, :text)")?;
        stmt.insert(named_params! {":title":note.title, ":text":note.text})
    }

    pub fn get_note(&self, id: i64) -> Result<Note> {
        let mut stmt = self
            .conn
            .prepare_cached("SELECT title, text FROM notebook WHERE id = ?")?;
        stmt.query_row([id], |row| {
            Ok(Note {
                title: row.get(0)?,
                text: row.get(1)?,
            })
        })
    }

    pub fn get_all_notes(&self) -> Result<Vec<NoteRow>> {
        let mut stmt = self
            .conn
            .prepare_cached("SELECT Id, title, text FROM notebook")?;
       let x = stmt
            .query_map([], |row| {
                Ok(NoteRow {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    text: row.get(2)?,
                })
            })?
            .collect();
        x
    }

    pub fn print_all_notes(&self) -> Result<()>{
        let notes = self.get_all_notes()?;
        println!("id\t| Title\n\t|\tText");
        for note_row in notes{
            println!("{:}", note_row);
        }
        Ok(())
    }

    //returns how many notes were updated
    pub fn update_note(&mut self, id: i64, note: &Note) -> Result<usize> {
        let mut stmt = self
            .conn
            .prepare_cached("UPDATE notebook SET title = :title, text = :text WHERE id = :id")?;
        stmt.execute(named_params! {
        ":title":note.title,
        ":text":note.text,
        ":id":id})
    }

       //returns how many notes were updated
    pub fn update_note_row(&mut self, note_row: &NoteRow) -> Result<usize> {
        let mut stmt = self
            .conn
            .prepare_cached("UPDATE notebook SET title = :title, text = :text WHERE id = :id")?;
        stmt.execute(named_params! {
        ":title":note_row.title,
        ":text":note_row.text,
        ":id":note_row.id})
    }

    //returns how many notes were deleted
    pub fn delete_note(&mut self, id: i64) -> Result<usize> {
        let mut stmt = self
            .conn
            .prepare_cached("DELETE FROM notebook WHERE id = ?")?;
        stmt.execute([id])
    }
}