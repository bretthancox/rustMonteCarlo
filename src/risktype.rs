///////////////////////////////////////////////////
///// Risk type ///////////////////////////////////
///////////////////////////////////////////////////

#[derive(Debug)]
pub struct RiskValues {
    pub name:               String,//&'static str,
    pub probability:        String,
    pub consequence:        String,
    pub optimistic:         f32,
    pub most_likely:        f32,
    pub pessimistic:        f32,
    pub pert_estimate:      f32,
    pub pm_standard_dev:    f32,
    pub range_upper:        f32,
    pub range_lower:        f32,
}

impl PartialEq for RiskValues {
    /*Without implementing PartialEq, could not compare
    names between different nodes*/
    fn eq(&self, comparator: &Self) -> bool {
        self.name == comparator.name
        &&
        self.probability == comparator.probability
        &&
        self.consequence == comparator.consequence
        &&
        self.optimistic == comparator.optimistic
        &&
        self.most_likely == comparator.most_likely
        &&
        self.pessimistic == comparator.pessimistic
    }
}

impl RiskValues {
    pub fn new(nom: String, prob: String, cons: String, opti: f32, most: f32, pes: f32) -> Self {
        let pert_estimate = (opti + (4.0 * most) + pes) / 6.0;
        let pm_std_dev = (pes - opti) / 6.0;

        Self {
            name:               nom,
            probability:        prob,
            consequence:        cons,
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