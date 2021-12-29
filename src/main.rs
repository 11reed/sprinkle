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

fn handle_sass(data: &[u8]) {
    use rsass::{compile_value, output};
    let format = output::Format {
        style: output::Style::Compressed,
        precision: 5,
    };
    let _ = compile_value(data, format);
}

fn main() -> Result<(), Box<Error>> {
    let sass_path = "./sass/sprinkle.scss";
    let data = std::fs::read(sass_path).unwrap();
    handle_sass(&data);

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