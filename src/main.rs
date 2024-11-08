mod big;

use std::fs;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    fn print(&self) {
        let mut equation = String::new();
        for (i, &coef) in self.coefficients.iter().enumerate().rev() {
            if coef != 0.0 {
                if !equation.is_empty() && coef > 0.0 {
                    equation.push_str(" + ");
                } else if coef < 0.0 {
                    equation.push_str(" - ");
                }

                let abs_coef = coef.abs();
                if i == 0 {
                    equation.push_str(&format!("{}", abs_coef));
                } else if i == 1 {
                    equation.push_str(&format!("{}x", abs_coef));
                } else {
                    equation.push_str(&format!("{}x^{}", abs_coef, i));
                }
            }
        }
        println!("Interpolated polynomial: {}", equation);
    }
}

fn lagrange_interpolation(x_values: &[f64], y_values: &[f64]) -> Polynomial {
    let n = x_values.len();
    let mut coefficients = vec![0.0; n];

    for i in 0..n {
        let mut term_coeffs = vec![1.0]; 
        for j in 0..n {
            if i != j {
                let xj = x_values[j];
                let mut new_term_coeffs = vec![0.0; term_coeffs.len() + 1];

                for (k, &coef) in term_coeffs.iter().enumerate() {
                    new_term_coeffs[k] -= coef * xj;
                    new_term_coeffs[k + 1] += coef;
                }
                let denom = x_values[i] - xj;
                for coef in new_term_coeffs.iter_mut() {
                    *coef /= denom;
                }
                term_coeffs = new_term_coeffs;
            }
        }

        for (k, &coef) in term_coeffs.iter().enumerate() {
            coefficients[k] += coef * y_values[i];
        }
    }

    Polynomial { coefficients }
}

fn decode_y_value(encoded_value: &str, base: u32) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(encoded_value, base)
}

fn main() {

    big::big();

    let file_content = fs::read_to_string("test_case.json").unwrap();
    let data: Value = serde_json::from_str(&file_content).unwrap();
    let k = data["keys"]["k"].as_u64().unwrap_or(0) as usize;
    let mut count = 0;

    let mut points: HashMap<u32, u32> = HashMap::new();

    if let Value::Object(map) = data {
        for (key, entry) in map {

            if key == "keys" {
                continue;
            }

            if count >= k {
                break;
            }

            let x: u32 = key.parse().unwrap();

            if let Value::Object(entry_map) = entry {
                if let (Some(base), Some(encoded_value)) = (
                    entry_map.get("base").and_then(|v| v.as_str()),
                    entry_map.get("value").and_then(|v| v.as_str()),
                ) {
                    let base: u32 = base.parse().unwrap();
                    if let Ok(y) = decode_y_value(encoded_value, base) {
                        points.insert(x, y);
                        count += 1;
                    } else {
                        eprintln!("Failed to decode value for x = {}", x);
                    }
                }
            }
        }
    }
    for (x, y) in &points {
        println!("x: {}, y: {}", x, y);
    }
    let x: Vec<f64> = points.keys().map(|&key| key as f64).collect();
    let y: Vec<f64> = points.values().map(|&value| value as f64).collect();


    let polynomial = lagrange_interpolation(&x, &y);
    polynomial.print();
}
