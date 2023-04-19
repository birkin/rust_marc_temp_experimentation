// parallel version -----------------------------


/* 
Results (slower!)...

rayon-version...

cargo run  2.07s user 1.07s system 97% cpu 3.208 total
cargo run  2.08s user 1.07s system 97% cpu 3.238 total
cargo run  2.08s user 0.98s system 94% cpu 3.237 total  <--

cargo run --release  0.21s user 1.07s system 94% cpu 1.347 total
cargo run --release  0.22s user 1.29s system 109% cpu 1.382 total
cargo run --release  0.22s user 1.23s system 105% cpu 1.378 total  <--
*/

extern crate glob;
extern crate marc;
extern crate rayon;

use marc::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

const RECORD_TERMINATOR: u8 = 0x1D;

fn main() {
    let marc_path = "../../x_marc_data/sierra_marc_files_combined/combined_01.mrc";
    println!("marc_path, ``{:?}``", marc_path);

    let marc_records: Vec<marc::Record> = load_records(marc_path.to_string());
    let title_field_tag = "245";

    let output = Arc::new(Mutex::new(BufWriter::new(std::io::stdout())));

    marc_records.par_iter().for_each(|rec| {
        let mut output = output.lock().unwrap();
        writeln!(&mut output, "\nnew rec...").unwrap();
        for field in rec.field(Tag::from(title_field_tag)).iter() {
            writeln!(&mut output, "{}", field.get_data::<str>()).unwrap();
        }
    });
}

fn load_records(filename: String) -> Vec<marc::Record<'static>> {
    let mut result_vector: Vec<marc::Record> = Vec::new();
    let mut buffer = Vec::new();

    let path = Path::new(&filename);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why.to_string()),
        Ok(file) => file,
    };

    let mut file = BufReader::new(file);
    while file.read_until(RECORD_TERMINATOR, &mut buffer).unwrap() != 0 {
        if let Ok(record) = marc::Record::from_vec(buffer.clone()) {
            result_vector.push(record);
        }

        buffer.clear();
    }

    result_vector
}


// non-parallel version -----------------------------


/* 
Results (faster!)...

single-threaded...

cargo run  2.11s user 0.30s system 61% cpu 3.931 total
cargo run  2.10s user 0.31s system 77% cpu 3.119 total  <--
cargo run  2.10s user 0.31s system 78% cpu 3.080 total

cargo run --release  0.25s user 0.30s system 44% cpu 1.231 total
cargo run --release  0.25s user 0.30s system 44% cpu 1.234 total <--
cargo run --release  0.25s user 0.30s system 44% cpu 1.234 total
*/


// extern crate glob;  // <https://docs.rs/glob/0.3.0/glob/>
// extern crate marc; // <https://github.com/blackbeam/rust-marc>

// use marc::*;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::path::Path;

// const RECORD_TERMINATOR: u8 = 0x1D;


// fn main() {

//     // -- get marc file path
//     // let marc_path: String = "../../x_marc_data/sierra_marc_files_few_small/sierra_export_0000.mrc".to_string();
//     let marc_path: String = "../../x_marc_data/sierra_marc_files_combined/combined_01.mrc".to_string();
//     println!( "marc_path, ``{:?}``", marc_path);

//     let marc_records: Vec<marc::Record> = load_records( marc_path );
//     // println!("marc_records, ``{:?}``", marc_records);

//     let title_field_tag: String = "245".to_string();

//     for rec in marc_records.iter() {
//         println!("\nnew rec...");
//         for field in rec.field( Tag::from(title_field_tag.as_str()) ).iter() {
//             println!("{}", field.get_data::<str>());
//         }
//     }

// }


// fn load_records( filename: String ) -> Vec< marc::Record<'static> > {
//     // -- using marc_cli to grok
//     let mut result_vector: Vec<marc::Record> = Vec::new();
//     let mut buffer = Vec::new();

//     let file_path = filename.clone();

//     let path = Path::new(file_path.as_str());
//     let display = path.display();

//     let file = match File::open(&path) {
//         Err(why) => panic!("Couldn't open {}: {}", display, why.to_string()),
//         Ok(file) => file,
//     };

//     let mut file = BufReader::new(file);
//     while file.read_until(RECORD_TERMINATOR, &mut buffer).unwrap() != 0 {
//         match marc::Record::from_vec(buffer.clone()) {
//             Err(_) => (),
//             Ok(record) => result_vector.push(record.clone()),
//         }

//         buffer.clear();
//     }

//     return result_vector;

// }
