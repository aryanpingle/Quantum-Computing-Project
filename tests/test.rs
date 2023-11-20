use ndarray::{Array1, Array2, ArrayView1, Ix2};
use std::f64::consts::FRAC_1_SQRT_2;
use std::time::{Duration, Instant};

mod rsqc;
use rsqc::QCircuit;

fn main() {
    // Time profiling for each gate
    let mut circ = QCircuit::new(10);

    let start = Instant::now();
    circ.h_gate(0);
    let duration = start.elapsed();
    println!("\nTime elapsed in Hadamard gate: {:?}s \n", duration);

    let start = Instant::now();
    circ.x_gate(0);
    let duration = start.elapsed();
    println!("Time elapsed in X gate: {:?}s \n", duration);

    let start = Instant::now();
    circ.cnot_gate(1,0);
    let duration = start.elapsed();
    println!("Time elapsed in CNOT gate: {:?}s \n", duration);

    let start = Instant::now();
    circ.z_gate(0);
    let duration = start.elapsed();
    println!("Time elapsed in Z gate: {:?}s \n", duration);

    // Create n qubit GHZ state which is a maximally entangled state
    let n = 10;
    let mut circ = QCircuit::new(n);
    let start = Instant::now();
    // println!("\n Start state = {:?}", circ.get_state());
    let start = Instant::now();
    circ.h_gate(0);
    let duration = start.elapsed();


    for index in 1..10 {
        circ.cnot_gate(index,0);
    }
    // println!("End state = {:?}\n", circ.get_state());
    let duration = start.elapsed();
    println!("Time elapsed in creating 10 qubits GHZ state: {:?}s \n", duration);

    // Implementing Deutsch's algorithm
    let start = Instant::now();
    let mut circ = QCircuit::new(4);
    // println!("\nStart gate = {:?}", circ.get_state());

    circ.h_gate(0);
    circ.h_gate(1);
    circ.h_gate(2);
    circ.x_gate(3);

    circ.x_gate(0);
    circ.x_gate(2);
    circ.h_gate(3);

    circ.cnot_gate(3,0);
    circ.cnot_gate(3,1);
    circ.cnot_gate(3,2);

    circ.x_gate(0);
    circ.h_gate(1);
    circ.x_gate(2);

    circ.h_gate(0);
    circ.h_gate(2);

    // println!("End state = {:?}\n", circ.get_state());

    let duration = start.elapsed();
    println!("Time elapsed for Deutsch-Jozsa Algorithm: {:?}s \n", duration);


}
