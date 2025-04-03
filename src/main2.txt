use std::{io, result};
use std::f64::consts::E;
use clap::{Arg, Command};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::File;
use std::fs;
use serde_json;


#[derive(Serialize, Deserialize, Debug)]
struct Planet {
    mass: f64,
    radius: f64,
}





const G: f64 = 6.67430e-11;
//const GRAVITY: f64 = 9.81;

fn escape_velocity(mass: f64, radius: f64) -> f64 {
    let escape_velocity: f64 = (2.0 * G * mass / radius).sqrt();
    return escape_velocity;
}




// type broken down T is  Result<T, E> T = success type , E error type || BOX = smart pointer own data ,,,, dyn any type of error
fn load_planets() -> Result<HashMap<String, Planet>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("planet.json")?;

    println!("{}", data);
    let planets: HashMap<String, Planet> = serde_json::from_str(&data)?;

    println!("{:?}", planets);

    Ok(planets)
}
/* 
fn main() {
    load_planets().unwrap();
} */

fn main() {
    match load_planets() {
        Ok(planets) => {
            println!("Loaded planets:");
            for (name, planet) in planets {
                println!("{}: mass = {} kg, radius = {} m", name, planet.mass, planet.radius);
            }
        },
        Err(e) => println!("Error: Could not load planet.json - {}", e),
    }
}
/* 
fn main() {
    let mut input_mass = String::new();  
    let mut input_radius = String::new();  

    println!("Enter the mass of the object: ");
    io::stdin().read_line(&mut input_mass).expect("Failed to read line");
    let mass: f64 = input_mass.trim().parse().expect("Please enter a valid number");
    println!("Input value: {}", mass);

    println!("Enter the radius of the object: ");
    io::stdin().read_line(&mut input_radius).expect("Failed to read line");
    let radius: f64 = input_radius.trim().parse().expect("Please enter a valid number");
    println!("Input value radius: {}", radius);

    let escape_velocity: f64 = escape_velocity(mass, radius);
    println!("Escape velocity: {}", escape_velocity);
} */




// How match result work and Result type HOW its working Result<T, E>
/* fn main() {
    let result: Result<f64, String> = divide(10.0, 1.0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e), 
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a / b)
} */