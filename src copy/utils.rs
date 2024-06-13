
pub fn normalize(values: &Vec<f64>) -> Vec<f64> {

    let sum = values.iter().sum::<f64>();
    let mut normalized = vec![];

    for i in 0..values.len() {
        normalized.push(values[i] / sum)
    }

    normalized
}

pub fn cumulative(values: &Vec<f64>) -> Vec<f64> {

    let mut cumulative = 0.0;
    let mut acc_values = Vec::new();

    for v in values.iter() {
        acc_values.push(cumulative + v);
        cumulative += v;
    }

    acc_values
}