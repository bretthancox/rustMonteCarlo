use super::{
    montecarlo,
    pert
};
use std::{
    fs::File,
    io::{self, BufReader, ErrorKind::*, Read},
    str::FromStr,
    path::PathBuf
};

/////////////////////////////////////////////////////
///// File reading //////////////////////////////////
/////////////////////////////////////////////////////

fn file_open(filename: PathBuf) -> Result<String, io::Error> {
    println!("OPENING FILE");
    /*let day: Opt = get_args();
    let filename = format!{"{}/{}", INPUT_DIR, day.day};*/
    let mut ret = String::new();

    //println!("{:?}", filename);

    if let Ok(file) = File::open(filename) {
        let mut buf = BufReader::new(file);
        buf.read_to_string(&mut ret)?;
        Ok(ret)
    } else {
        Err(io::Error::new(InvalidData, "File not found"))
    }
}

/////////////////////////////////////////////////////
///// String handlers ///////////////////////////////
/////////////////////////////////////////////////////

fn splitter(input_str: String, split_point: &str) -> Vec<String> {
    println!("SPLITTING STRINGS {} at {:?}", &input_str, split_point);
    let vec_out: Vec<String> = input_str.split(split_point).map(|s| String::from(s.trim())).collect();
    vec_out
}


fn prep_strings_level_one(filename: PathBuf) -> Result<Vec<String>, io::Error> {
    match file_open(filename) {
        Ok(content) => {
            println!("CONTENT: {}", content);
            let output = splitter(content, "\n");
            println!("OUTPUT: {:?}", output);
            Ok(output)
        },
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        },
    }
}


pub fn prep_strings_level_two(filename: PathBuf, ocol: usize, mcol: usize, wcol: usize, ncol: usize, sep: &str, its: usize) -> Result<montecarlo::MonteCarlo, io::Error> {
    let vec_of_strings = prep_strings_level_one(filename).expect("SOMETHING WRONG WITH vec_of_strings");

    let mut monty_karlo = montecarlo::MonteCarlo::new(its);

    for string_input in vec_of_strings {
        let row = splitter(string_input, sep);
        let name = String::from(&row[ncol - 1]);
        let opp = f32::from_str(&row[ocol - 1]).expect("Failed on opp");
        let most = f32::from_str(&row[mcol - 1]).expect("Failed on most");
        let wor = f32::from_str(&row[wcol - 1]).expect("Failed on wor");
        let inner_pert = pert::PertValues::new(name, opp, most, wor);
        monty_karlo.perts.push(inner_pert);
    }

    Ok(monty_karlo)
}