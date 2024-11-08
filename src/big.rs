use std::{collections::hash_map, fs, str::FromStr};
use serde_json::Value;
use std::collections::HashMap;
use num_bigint::{BigInt, BigUint};
use num_traits::Num;
use num_rational::BigRational;
use num_traits::{Zero, One, Signed};

fn decode_value(encoded_value: &str, base: u32) -> BigInt {
    let mut value = BigInt::from(0u32);
    let mut power = BigInt::from(1u32);

    for digit_char in encoded_value.chars().rev() {
        let digit = digit_char.to_digit(base).unwrap();
        value += BigInt::from(digit) * &power;
        power *= BigInt::from(base);
    }

    value
}


#[derive(Debug, Clone)]
struct Polynomial {
    coefficients: Vec<BigRational>,
}

impl Polynomial {
    fn print(&self) {
        let mut equation = String::new();
        for (i, coef) in self.coefficients.iter().enumerate().rev() {
            if !coef.is_zero() {
                let sign = if coef.is_negative() { " - " } else if !equation.is_empty() { " + " } else { "" };
                equation.push_str(&format!("{}{}", sign, coef.abs()));

                if i > 0 {
                    equation.push_str("x");
                    if i > 1 {
                        equation.push_str(&format!("^{}", i));
                    }
                }
            }
        }
        println!("Interpolated polynomial: {}", equation);
    }
}

fn lagrange_interpolation(x_values: &[BigInt], y_values: &[BigInt]) -> Polynomial {
    let n = x_values.len();
    let mut coefficients = vec![BigRational::zero(); n];

    for i in 0..n {
        let mut term_coeffs = vec![BigRational::one()]; 
        for j in 0..n {
            if i != j {
                let xj = BigRational::from_integer(x_values[j].clone());
                let mut new_term_coeffs = vec![BigRational::zero(); term_coeffs.len() + 1];

                for (k, coef) in term_coeffs.iter().enumerate() {
                    new_term_coeffs[k] -= coef * &xj;
                    new_term_coeffs[k + 1] += coef;
                }
                let denom = BigRational::from_integer(&x_values[i] - &x_values[j]);
                for coef in new_term_coeffs.iter_mut() {
                    *coef /= &denom;
                }
                term_coeffs = new_term_coeffs;
            }
        }

        let yi = BigRational::from_integer(y_values[i].clone());
        for (k, coef) in term_coeffs.iter().enumerate() {
            coefficients[k] += coef * &yi;
        }
    }

    Polynomial { coefficients }
}

fn insert_entry(
    arr: &mut HashMap<String, HashMap<String, String>>,
    x: &str,
    base: &str,
    value: &str,
) {
    let mut inner_map = HashMap::new();
    inner_map.insert("base".to_string(), base.to_string());
    inner_map.insert("value".to_string(), value.to_string());

    arr.insert(x.to_string(), inner_map);
}


// Function to read data from a HashMap and convert it
fn read_values_from_map(map: &HashMap<String, HashMap<String, String>>) -> (Vec<BigInt>, Vec<BigInt>) {
    let mut x_values = Vec::new();
    let mut y_values = Vec::new();

    for (key, data) in map.iter() {
        let base_str = data.get("base").unwrap();
        let value_str = data.get("value").unwrap();

        let base = base_str.parse::<u32>().unwrap();
        
        let y = decode_value(&value_str, base);

        let x = BigInt::from_str(key).unwrap();

        x_values.push(x);
        y_values.push(y);
    }

    (x_values, y_values)
}

pub fn big() -> Result<(), Box<dyn std::error::Error>> {

    let file_content = fs::read_to_string("test_case1.json")?;
    let data: Value = serde_json::from_str(&file_content)?;
    let k = data["keys"]["k"].as_u64().unwrap_or(0) as usize;
    let mut count = 0;

    let mut points: HashMap<u32, BigUint> = HashMap::new();
    let mut arr: HashMap<String, HashMap<String, String>> = HashMap::new();

    if let Value::Object(map) = data {
        for (key, entry) in map {

            if key == "keys" {
                continue;
            }

            if count >= k {
                break;
            }


            let x: u32 = key.parse()?;


            if let Value::Object(entry_map) = entry {
                if let (Some(base), Some(encoded_value)) = (
                    entry_map.get("base").and_then(|v| v.as_str()),
                    entry_map.get("value").and_then(|v| v.as_str()),
                ) {
                    insert_entry(&mut arr, &x.to_string(), base, encoded_value);
                    // let base: u32 = base.parse()?;
                    // match decode_value(base, encoded_value) {
                    //     Ok(y) => {
                    //         points.insert(x, y);
                    //         count += 1;
                    //     }
                    //     Err(e) => eprintln!("Failed to decode value for x = {}: {}", x, e),
                    // }
                }
            }
        }
    }

    let (mut x_arr, mut y_arr) = read_values_from_map(&arr);
    let polynomial = lagrange_interpolation(&x_arr, &y_arr);
    polynomial.print();

    Ok(())
}