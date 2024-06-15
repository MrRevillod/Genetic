use colored::CustomColor;


pub fn normalize(values: &mut Vec<f64>) {

    let sum = values.iter().sum::<f64>();

    for i in 0..values.len() {
        values[i] = values[i] / sum;
    }
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

pub fn to_rgb(color: (f64, f64, f64)) -> CustomColor {
    let (r, g, b) = (
        (color.0 * 255.0) as u8,
        (color.1 * 255.0) as u8,
        (color.2 * 255.0) as u8
    );

    CustomColor::new(r, g, b)
}