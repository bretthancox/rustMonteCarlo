///////////////////////////////////////////////////
///// PERT type ///////////////////////////////////
///////////////////////////////////////////////////

#[derive(Debug)]
pub struct PertValues {
    pub name:               String,//&'static str,
    pub optimistic:         f32,
    pub most_likely:        f32,
    pub pessimistic:        f32,
    pub pert_estimate:      f32,
    pub pm_standard_dev:    f32,
    pub range_upper:        f32,
    pub range_lower:        f32,
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
    pub fn new(nom: String, opti: f32, most: f32, pes: f32) -> Self {
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