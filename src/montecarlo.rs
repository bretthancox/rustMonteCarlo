use super::pert;
use super::risktype;
use rand::{thread_rng, Rng};
////////////////////////////////////////////////////
///// Monte Carlo type /////////////////////////////
////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MonteCarlo {
    pub perts:          Vec<pert::PertValues>,
    pub risks:          Vec<risktype::RiskValues>,
    pub iterations:     usize,
    pub estimates:      Vec<f32>,
}

impl MonteCarlo {
    pub fn new(its: usize) -> Self {
        Self {
            iterations:     its,
            perts:          Vec::with_capacity(its),
            risks:          Vec::with_capacity(its),
            estimates:      Vec::with_capacity(its),
        }
    }

    fn single_estimate(&self, lower: f32, upper: f32) -> f32 {
        /* Produces a single estimate within the bounds of a lower and upper value */
        let mut rng = thread_rng();
        let estimate = rng.gen_range(lower, upper);
        estimate
    }

    fn single_risk_occurrence(&self, probability: &str) -> bool {
        let mut rng = thread_rng();
        let occurrence: f32 = rng.gen_range(0.0, 100.0);

        let probability_percent = define_upper_bound(probability);

        if probability_percent >= occurrence {
            //println!("Occurrence: {} Probability: {}", &occurrence, &probability_percent);
            true
        } else {
            false
        }
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

    fn single_risk_iteration(&mut self) {
        let mut total_impact = 0.0;

        for risk in &self.risks {
            let risk_occurred = self.single_risk_occurrence(&risk.probability);
            //println!("The risk occurred: {}", &risk_occurred);
            if risk_occurred {
                total_impact += self.single_estimate(risk.range_lower, risk.range_upper)
            } else {
                total_impact += 0.0
            }
        }
        self.estimates.push(total_impact);
    }

    pub fn iterate(&mut self, task: &str) {
        /* For the iterations in the MonteCarlo type, perform a full task estimate. Sort the
           estimates vector from smallest to largest. */
        println!("fn iterate");
        match task {
            "risk" => {
                for _x in 0..self.iterations {
                    self.single_risk_iteration();
                }
            },
            "pert" => {
                for _x in 0..self.iterations {
                    self.single_iteration();
                }
            },
            &_ => {
                panic!("Not a valid selection. Pick 'risk' or 'pert'");
            }
        }
        self.estimates.sort_by(|a, b| a.partial_cmp(b).expect("Problem unwrapping estimates in fn iterate"));
    }

    pub fn print_confidence(&self, confidence: f32, task: &str) {
        /* Print the estimate found at the confidence number supplied, where 
           the confidence is an integer. Should allow a float? */
        let single_percent: f32 = (self.iterations / 100) as f32;

        let index_float: f32 = single_percent * confidence;

        let conf_result: f32 = self.estimates[index_float.round() as usize];

        match task {
            "risk" => println!("Over {} iterations there is a(n) {}% confidence of risk impact being {:.1} or less.", self.iterations, confidence, conf_result),
            "pert" => println!("Over {} iterations there is a(n) {}% confidence of completing in {:.1} days or less.", self.iterations, confidence, conf_result),
            &_ => panic!("Not a valid selection. Pick 'risk' or 'pert'"),
        }
    }
}



fn define_upper_bound(risk_category: &str) -> f32 {
    let upper: f32;
    let lower: f32;

    match risk_category {
        "Certain" => {
            upper = 99.0;
            lower = 90.0;
        },
        "Likely" => {
            upper = 90.0;
            lower = 50.0;
        },
        "Moderate" => {
            upper = 50.0;
            lower = 10.0;
        },
        "Unlikely" => {
            upper = 10.0;
            lower = 3.0;
        },
        "Rare" => {
            upper = 3.0;
            lower = 0.1;
        },
        &_ => {
            panic!("Did not get a valid risk category")
        },
    }

    let mut rng = thread_rng();
    let estimate = rng.gen_range(lower, upper);
    //println!("{}", &estimate);
    estimate
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_bound_certain() {
        let risk_category = "Certain";
        let risk_result_lower = 90.0; 
        let risk_result = define_upper_bound(risk_category);
        println!("{}", risk_result);
        assert!(risk_result > risk_result_lower);
    }

    #[test]
    fn test_risk_bound_likely() {
        let risk_category = "Likely";
        let risk_result_lower = 50.0; 
        let risk_result = define_upper_bound(risk_category);
        println!("{}", risk_result);
        assert!(risk_result > risk_result_lower);
    }

    #[test]
    fn test_risk_bound_moderate() {
        let risk_category = "Moderate";
        let risk_result_lower = 10.0; 
        let risk_result = define_upper_bound(risk_category);
        println!("{}", risk_result);
        assert!(risk_result > risk_result_lower);
    }

    #[test]
    fn test_risk_bound_unlikely() {
        let risk_category = "Unlikely";
        let risk_result_lower = 3.0; 
        let risk_result = define_upper_bound(risk_category);
        println!("{}", risk_result);
        assert!(risk_result > risk_result_lower);
    }

    #[test]
    fn test_risk_bound_rare() {
        let risk_category = "Rare";
        let risk_result_lower = 0.1; 
        let risk_result = define_upper_bound(risk_category);
        println!("{}", risk_result);
        assert!(risk_result > risk_result_lower);
    }

    #[test]
    #[should_panic]
    fn test_risk_bound_panics() {
        let risk_category = "Pumpkin";
        let risk_result_lower = 0.1; 
        let risk_result = define_upper_bound(risk_category);
        println!("{}", risk_result);
        assert!(risk_result > risk_result_lower);
    }
}