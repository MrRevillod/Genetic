
use uuid::Uuid;
use rand::RngCore;
use colored::CustomColor;

pub use crate::random::random;

pub fn normalize(values: &Vec<f64>) -> Vec<f64> {

    let mut normalized: Vec<f64> = Vec::new();

    let sum = values.iter().sum::<f64>();

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

pub fn to_rgb(color: (f64, f64, f64)) -> CustomColor {

    let mut color = vec![color.0, color.1, color.2];

    color = normalize(&color);

    CustomColor::new(
        (color[0] * 255.0) as u8,
        (color[1] * 255.0) as u8,
        (color[2] * 255.0) as u8
    )
}

pub fn trunc_uuid(uuid: &Uuid) -> String {
    return uuid.to_string()[..4].to_string()
}

pub fn uuid() -> Uuid {
    let mut bytes = [0u8; 16];
    random().fill_bytes(&mut bytes);
    Uuid::from_bytes(bytes)
}

