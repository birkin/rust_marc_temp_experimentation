extern crate glob;  // <https://docs.rs/glob/0.3.0/glob/>
extern crate marc; // <https://github.com/blackbeam/rust-marc>

// use glob::glob;
// use std::error::Error;
// use std::fmt::Debug;
// use std::fmt::Display;
// use std::io::{BufRead, BufReader, Read};
// use std::io;
use marc::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const RECORD_TERMINATOR: u8 = 0x1D;





fn main() {

    // -- get marc file path
    let marc_path: String = "../../x_marc_data/sierra_marc_files_few_small/sierra_export_0000.mrc".to_string();
    println!( "marc_path, ``{:?}``", marc_path);

    let marc_records: Vec<marc::Record> = load_records( marc_path );
    println!("marc_records, ``{:?}``", marc_records);

    let title_field_tag: String = "245".to_string();

    for rec in marc_records.iter() {
        println!("\nnew rec...");
        // for field in rec.field(Tag::from(field_tag.as_str())).iter() {
        for field in rec.field( Tag::from(title_field_tag.as_str()) ).iter() {
            println!("{}", field.get_data::<str>());
        }

    }

}


fn load_records( filename: String ) -> Vec< marc::Record<'static> > {
    // -- using marc_cli to grok
    let mut result_vector: Vec<marc::Record> = Vec::new();
    let mut buffer = Vec::new();

    let file_path = filename.clone();

    let path = Path::new(file_path.as_str());
    let display = path.display();

    // let file = match File::open(&path) {
    //     Err(why) => panic!("Couldn't open {}: {}", display, why.description()),
    //     Ok(file) => file,
    // };

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    let mut file = BufReader::new(file);
    while file.read_until(RECORD_TERMINATOR, &mut buffer).unwrap() != 0 {
        match marc::Record::from_vec(buffer.clone()) {
            Err(_) => (),
            Ok(record) => result_vector.push(record.clone()),
        }

        buffer.clear();
    }

    return result_vector;

}


// fn main() {

//     // -- get marc file path
//     let marc_path: String = "../../files/sierra_export_0726.mrc".to_string();
//     println!( "marc_path, ``{:?}``", marc_path);

//     let marc_records: Vec<marc::Record> = load_records( marc_path );
//     println!("marc_records, ``{:?}``", marc_records);

//     let title_field_tag: String = "245".to_string();

//     for rec in marc_records.iter() {
//         println!("\nnew rec...");
//         // for field in rec.field(Tag::from(field_tag.as_str())).iter() {
//         for field in rec.field( Tag::from(title_field_tag.as_str()) ).iter() {
//             println!("{}", field.get_data::<str>());
//         }

//     }

//     // -- get list of marc files
//     // let file_download_dir: String = "../../files".to_string();
//     // let pattern: String = format!( "{}/*.mrc", file_download_dir );
//     // println!("pattern, ``{:?}``", pattern);

//     // -- apply the pattern
//     // let paths = glob( &pattern ).unwrap_or_else( |err| {
//     //     panic!("could not glob the pattern; error, ``{}``", err);
//     // });
//     // println!("paths, ``{:?}``", paths);  // error: `glob::Paths` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`

//     // for entry in paths {
//     //     let path = entry.unwrap_or_else( |err| {  // path without unwrap is: enum `std::result::Result<std::path::PathBuf, glob::GlobError>`
//     //         panic!("could not access the path; error, ``{}``", err);
//     //     });
//     //     // println!("path, ``{:?}``", path);
//     //     // let zz: () = path;  // yields: found struct `std::path::PathBuf`

//     //     let path_str = path.to_str().unwrap_or_else( || {
//     //      panic!("could turn the path into a string");
//     //     });
//     //     // let zz: () = path_str;  // yields: found `&str`

//     //     let path_string: String = path_str.into();
//     //     println!("path_string, ``{:?}``", path_string);
//     //     // let zz: () = path_string;  // yields: found struct `std::string::String`

//     //     // let validity = open_and_check_file( &path_string );

//     //     // let validity = open_and_check_file_02( &path_string );

//     //     open_and_check_file_03( &path_string );

//     // }

// }


// fn open_and_check_file_03( path: &str ) {
//     let mut fh = std::fs::File::open( &path ).unwrap();
//     println!("fh, ``{:?}``", fh);
//     // let zz: () = fh;  // yields: found struct `std::fs::File`

//     let mut contents = String::new();
//     fh.read_to_string( &mut contents ).unwrap();
//     // print!("contents, ``{}``", contents);
//     // let zz: () = contents;  // yields: found struct `std::string::String`

//     let mut data = vec![];
//     data.extend_from_slice( contents.as_bytes() );
//     let input = io::Cursor::new(data);
//     let mut records = marc::Records::new(input);
//     // let zz: () = records;  // yields: found struct `marc::Records`

//     let record = records.next().unwrap().unwrap();
//     // println!("record, ``{:?}``", record);
//     // let zz: () = record;  // yields: found struct `marc::Record`

//     // println!("record-directory, ``{:?}``", record.directory);  // says it's private
//     // println!( "fields, ``{:?}``", record.fields() );  // yields: `marc::Fields<'_>` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`

//     // for field in record.fields() {
//     //     println!("field, ``{:?}``", field);
//     // }

//     let title_fields = record.field("245");
//     println!( "title_fields, ``{:?}``", title_fields );
//     // let zz: () = title_fields;  // yields: struct `std::vec::Vec`

//     let first_title_field = &title_fields[0];
//     println!( "first_title_field, ``{:?}``", first_title_field );
//     // let zz: () = first_title_field;  // yields: found `&marc::Field<'_>`

//     // for subfield in title_fields {
//     //     println!("subfield, ``{:?}``", subfield);
//     //     // let zz: () = subfield;  // yields: found struct `marc::Field`

//     //     // println!("data, ``{:?}``", subfield.data);  // yields: error[E0616]: field `data` of struct `marc::Field` is private
//     //     // println!("data, ``{:?}``", subfield["data"]);  // yields: error[E0608]: cannot index into a value of type `marc::Field<'_>`
//     //     // println!("tag, ``{:?}``", subfield.tag);  // yields: error[E0616]: field `tag` of struct `marc::Field` is private
//     //     // println!("subfield.as_ref(), ``{:?}``", subfield.as_ref());  // yields: error[E0599]: no method named `as_ref` found for struct `marc::Field<'_>` in the current scope
//     // }


// }



// fn open_and_check_file_02( path: &str ) -> String {
//     let mut fh = std::fs::File::open( &path ).unwrap();
//     println!("fh, ``{:?}``", fh);
//     // let zz: () = fh;  // yields: found struct `std::fs::File`

//     let mut buffer = Vec::new();
//     // read the whole file
//     let something = fh.read_to_end( &mut buffer ).unwrap();
//     println!("something, ``{:?}``", something);
//     // let zz: () = something; // yields: found `usize`

//     println!("buffer, ``{:?}``", buffer);

//     return "foo".to_string();
// }


// fn open_and_check_file( path: &str ) {
//     // -- <https://www.tutorialspoint.com/rust/rust_file_input_output.htm>
//     let mut fh = std::fs::File::open( &path ).unwrap();
//     println!("fh, ``{:?}``", fh);
//     // let zz: () = fh;  // yields: found struct `std::fs::File`

//     let mut contents = String::new();
//     fh.read_to_string( &mut contents ).unwrap();
//     print!("contents, ``{}``", contents);
//     // let zz: () = contents;  // yields: found struct `std::string::String`


//     // Interesting, if I comment out `use std::io::Read;` at the top, I get this:

//     //     (env3_marc) birkin@bbox-2015$
//     //     (env3_marc) birkin@bbox-2015$ cargo check
//     //         Checking marc_read v0.1.0 (/Users/birkin/Documents/rust_projects/marc_read_stuff/marc_read_comparison/marc_read)
//     //     error[E0599]: no method named `read_to_string` found for struct `std::fs::File` in the current scope
//     //       --> src/main.rs:49:10
//     //        |
//     //     49 |     file.read_to_string(&mut contents).unwrap();
//     //        |          ^^^^^^^^^^^^^^ method not found in `std::fs::File`
//     //        |
//     //        = help: items from traits can only be used if the trait is in scope
//     //     help: the following trait is implemented but not in scope; perhaps add a `use` for it:
//     //        |
//     //     6  | use std::io::Read;
//     //        |

//     //     error: aborting due to previous error

//     //     For more information about this error, try `rustc --explain E0599`.
//     //     error: could not compile `marc_read`.

//     //     To learn more, run the command again with --verbose.
//     //     (env3_marc) birkin@bbox-2015$

// }
