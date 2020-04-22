mod pert;
mod montecarlo;
mod risktype;
mod filehandler;
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

/////////////////////////////////////////////////////
///// Main //////////////////////////////////////////
/////////////////////////////////////////////////////


fn main() {
    println!("MAIN");
    let arguments: Opt = get_args();

    println!("{:?}", &arguments);

    let most =              arguments.most;
    let opti =              arguments.optimistic;
    let worst =             arguments.worst;
    let name =              arguments.name;
    let sep =               arguments.separator;
    let iterations =        arguments.iterations;
    let confidence_levels = vec![50.0, 80.0, 90.0, 95.0, 99.0, 99.9];

    if let Ok(mut monty) = filehandler::prep_strings_level_two(arguments.file, opti, most, worst, name, &sep, iterations) {
        monty.iterate("pert");
        for conf in &confidence_levels {
            monty.print_confidence(*conf, "pert");
        }
    } else {
        println!("Errored!!");
    }

    let risk_1 = risktype::RiskValues::new(String::from("Risk 1"), String::from("Likely"), String::from("Severe"), 10_000.0, 22_000.0, 43_000.0);
    let risk_2 = risktype::RiskValues::new(String::from("Risk 2"), String::from("Unlikely"), String::from("Severe"), 18_000.0, 27_000.0, 65_000.0);
    let risk_3 = risktype::RiskValues::new(String::from("Risk 3"), String::from("Unlikely"), String::from("Severe"), 108_000.0, 130_000.0, 190_000.0);

    let mut risky_carlo = montecarlo::MonteCarlo::new(10_000);
    risky_carlo.risks.push(risk_1);
    risky_carlo.risks.push(risk_2);
    risky_carlo.risks.push(risk_3);
    risky_carlo.iterate("risk");
    for conf in confidence_levels {
        risky_carlo.print_confidence(conf, "risk");
    }

    //println!("{:?}", risky_carlo.estimates);
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
        let test_pert = pert::PertValues::new(String::from("Bertie"), 4.5, 7.8, 15.0);
        let comparator = pert::PertValues {
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