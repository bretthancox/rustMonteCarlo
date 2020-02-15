use rand::{thread_rng, Rng};
use std::{
    fs::File,
    io::{self, BufReader, ErrorKind::*, Read},
    str::FromStr,
    path::PathBuf
};
use structopt::StructOpt;
//use std::mem;


///////////////////////////////////////////////////
///// Command line options ////////////////////////
///////////////////////////////////////////////////

#[derive(StructOpt, Debug)]
#[structopt(name = "Monte Carlo", about = "Monte Carlo simulator. 
Assumes that the input file is laid out as 'task name, optimistic estimate, most likely estimate, worst case estimate'.
If that is not the case, specify the column numbers for the name and each estimate.")]
struct Opt {
    /// Provide the csv column/field number for the task name
    #[structopt(short, long = "--name-column", default_value = "1")]
    name: usize,
    
    /// Provide the csv column/field number for the optimistic estimate
    #[structopt(short, long = "--optimistic-column", default_value = "2")]
    optimistic: usize,

    /// Provide the csv column/field number for the most likely estimate
    #[structopt(short, long = "--most-likely-column", default_value = "3")]
    most: usize,

    /// Provide the csv column/field number for the worst case estimate
    #[structopt(short, long = "--worst-case-column", default_value = "4")]
    worst: usize,

    /// Provide the full (i.e. absolute) file path for the csv
    #[structopt(short, long = "--file", parse(from_os_str))]
    file: PathBuf,

    /// Provide the csv field separator used in the csv file. Cannot be special characters (e.g. tab).
    #[structopt(short, long = "--separator", default_value = ",")]
    separator: String,

    /// Number of iterations to run
    #[structopt(short, long = "--iterations", default_value = "10000")]
    iterations: usize,
}

fn get_args() -> Opt {
    let opt = Opt::from_args();
    opt
}



///////////////////////////////////////////////////
///// PERT type ///////////////////////////////////
///////////////////////////////////////////////////

#[derive(Debug)]
pub struct PertValues {
    name:               String,//&'static str,
    optimistic:         f32,
    most_likely:        f32,
    pessimistic:        f32,
    pert_estimate:      f32,
    pm_standard_dev:    f32,
    range_upper:        f32,
    range_lower:        f32,
}

impl PartialEq for PertValues {
    /*Without implementing PartialEq, could not compare
    names between different nodes*/
    fn eq(&self, comparator: &Self) -> bool {
        self.name == comparator.name
        &&
        self.optimistic == comparator.optimistic
        &&
        self.most_likely == comparator.most_likely
        &&
        self.pessimistic == comparator.pessimistic
    }
}

impl PertValues {
    fn new(nom: String, opti: f32, most: f32, pes: f32) -> Self {
        let pert_estimate = (opti + (4.0 * most) + pes) / 6.0;
        let pm_std_dev = (pes - opti) / 6.0;

        Self {
            name:               nom,
            optimistic:         opti,
            most_likely:        most,
            pessimistic:        pes,
            pert_estimate:      pert_estimate,
            pm_standard_dev:    pm_std_dev,
            range_upper:        pert_estimate + pm_std_dev,
            range_lower:        pert_estimate - pm_std_dev,
        }
    }
}

////////////////////////////////////////////////////
///// Monte Carlo type /////////////////////////////
////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MonteCarlo {
    perts:          Vec<PertValues>,
    iterations:     usize,
    estimates:      Vec<f32>,
}

impl MonteCarlo {
    fn new(its: usize) -> Self {
        Self {
            iterations:     its,
            perts:          Vec::with_capacity(its),
            estimates:      Vec::with_capacity(its),
        }
    }

    fn single_estimate(&self, lower: f32, upper: f32) -> f32 {
        /* Produces a single estimate within the bounds of a lower and upper value */
        let mut rng = thread_rng();
        let estimate = rng.gen_range(lower, upper);
        estimate
    }

    fn single_iteration(&mut self) {
        /* For a single pass through all tasks/projects, produce a sequential estimate total
           and add it to the estimates vector in the MonteCarlo type. */
        let mut estimate_sum = 0.0;
        for perts in &self.perts {
            estimate_sum += self.single_estimate(perts.range_lower, perts.range_upper)
        }
        self.estimates.push(estimate_sum);
    }

    fn iterate(&mut self) {
        /* For the iterations in the MonteCarlo type, perform a full task estimate. Sort the
           estimates vector from smallest to largest. */
        for _x in 0..self.iterations {
            self.single_iteration();
        }
        self.estimates.sort_by(|a, b| a.partial_cmp(b).expect("Problem unwrapping estimates in fn iterate"));
    }

    fn print_confidence(&self, confidence: f32) {
        /* Print the estimate found at the confidence number supplied, where 
           the confidence is an integer. Should allow a float? */
        let single_percent: f32 = (self.iterations / 100) as f32;

        let index_float: f32 = single_percent * confidence;

        let conf_result: f32 = self.estimates[index_float.round() as usize];

        println!("Over {} iterations there is a(n) {}% confidence of completing in {:.1} days.", self.iterations, confidence, conf_result);
    }
}

/////////////////////////////////////////////////////
///// File reading //////////////////////////////////
/////////////////////////////////////////////////////

fn file_open(filename: PathBuf) -> Result<String, io::Error> {
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
    let vec_out: Vec<String> = input_str.split(split_point).map(|s| String::from(s.trim())).collect();
    vec_out
}


fn prep_strings_level_one(filename: PathBuf) -> Result<Vec<String>, io::Error> {
    match file_open(filename) {
        Ok(content) => {
            //println!("{}", content);
            let output = splitter(content, "\n");
            //println!("{:?}", output);
            Ok(output)
        },
        Err(e) => {
            println!("Error: {}", e);
            Err(e)
        },
    }
}


fn prep_strings_level_two(filename: PathBuf, ocol: usize, mcol: usize, wcol: usize, ncol: usize, sep: &str, its: usize) -> Result<MonteCarlo, io::Error> {
    let vec_of_strings = prep_strings_level_one(filename)?;

    let mut monty_karlo = MonteCarlo::new(its);

    for string_input in vec_of_strings {
        let row = splitter(string_input, sep);
        let name = String::from(&row[ncol - 1]);
        let opp = f32::from_str(&row[ocol - 1]).expect("Failed on opp");
        let most = f32::from_str(&row[mcol - 1]).expect("Failed on most");
        let wor = f32::from_str(&row[wcol - 1]).expect("Failed on wor");
        let inner_pert = PertValues::new(name, opp, most, wor);
        monty_karlo.perts.push(inner_pert);
    }

    Ok(monty_karlo)
}

/////////////////////////////////////////////////////
///// Main //////////////////////////////////////////
/////////////////////////////////////////////////////


fn main() {
    let arguments: Opt = get_args();

    //println!("{:?}", &arguments);

    let most =              arguments.most;
    let opti =              arguments.optimistic;
    let worst =             arguments.worst;
    let name =              arguments.name;
    let sep =               arguments.separator;
    let iterations =        arguments.iterations;
    let confidence_levels = vec![50.0, 80.0, 90.0, 95.0, 99.0, 99.9];


    if let Ok(mut monty) = prep_strings_level_two(arguments.file, opti, most, worst, name, &sep, iterations) {
        monty.iterate();
        for conf in confidence_levels {
            monty.print_confidence(conf);
        }
    } else {
        println!("Errored!!");
    }

    /*
    println!("{}", mem::size_of::<f32>());
    println!("{}", mem::size_of::<usize>());
    println!("{}", mem::size_of::<MonteCarlo>());
    println!("{}", mem::size_of::<PertValues>());
    println!("{:?}", mem::size_of_val(&monty));
    println!("{:?}", mem::size_of_val(&monty.estimates));
*/
}


/////////////////////////////////////////////////////
///// Test //////////////////////////////////////////
/////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pert() {
        let test_pert = PertValues::new(String::from("Bertie"), 4.5, 7.8, 15.0);
        let comparator = PertValues {
            name:               String::from("Bertie"),
            optimistic:         4.5,
            most_likely:        7.8,
            pessimistic:        15.0,
            pert_estimate:      8.45,
            pm_standard_dev:    1.75,
            range_upper:        10.2,
            range_lower:        6.7,
        };
        assert_eq!(test_pert, comparator);  
    }


    /*let test_pert = PertValues::new("Bertie", 4.5, 7.8, 15.0);
    let test_pert_2 = PertValues::new("Joanie", 7.0, 14.3, 28.9);
    let test_pert_3 = PertValues::new("Archie", 13.0, 28.65, 48.0);

    let mut monty = MonteCarlo::new(100);
    monty.perts.push(test_pert);
    monty.perts.push(test_pert_2);
    monty.perts.push(test_pert_3);
    monty.iterate();*/
}