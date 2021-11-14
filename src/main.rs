use std::io::Error;
use fs_extra::dir::get_size;
use byte_unit::Byte;
use colored::*;
use term_table::{
    TableBuilder,
    TableStyle,
    row::Row,
    table_cell::{Alignment, TableCell},
};

fn main() -> Result<(), Box<grass::Error>> {
    // let handle_sass = grass::from_path("./scss/", &grass::Options::default())?;

    // println!("{}", handle_sass);

    let folder_size = get_size("./").unwrap();
    let result = Byte::from_bytes(folder_size.into());

    let table = TableBuilder::new().style(TableStyle::extended()).rows(
        vec![
            Row::new(vec![
                TableCell::new_with_alignment("Size Report".cyan(), 2, Alignment::Center)
            ]),
            Row::new(vec![
                TableCell::new("Total Bytes".magenta()),
                TableCell::new_with_alignment(format!("{} B", result), 1, Alignment::Center)
            ]),
        ]
    ).build();
    
    println!("{}", table.render());

    Ok(())
}