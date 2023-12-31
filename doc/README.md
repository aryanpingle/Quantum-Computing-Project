# Comparative Study of Quantum Computing Tasks in Rust & Python

| Name of member | BITS ID | Email |
|---|---|---|
| Aryan Pingle | 2020B3A70362G | f20200362@goa.bits-pilani.ac.in |
| Aditya Dubey | 2020B5A70627G | f20200627@goa.bits-pilani.ac.in |
| Subrahmanyam Mantha | 2020B5A70502G | f20200502@goa.bits-pilani.ac.in |
| Aditya Bhat | 2020B5A72045G | f20202045@goa.bits-pilani.ac.in |

## Problem Statement

In this project, we aim to implement basic quantum computing operations and algorithms in Rust. We'll leverage Rust's robust memory management principles to ensure code reliability and safety. Our primary focus will be a comparative analysis, with a strong emphasis on speed, highlighting Rust's performance advantages over Python. This project serves as a valuable case study for the adoption of Rust in quantum computing.

> Where is the POPL angle in it?

* We are specifically exploring two of the guidelines given to us: performance and easy-of-use
* This study will assess the **performance** differences in the two languages chosen - Rust and Python, and we will profile the code to detect bottlenecks
* We are implementing modules common to quantum computing algorithms in order to create a makeshift library, providing **ease-of-use**
* Both languages deal with different abstraction layers and design philosophies, giving us a better perspective on the principles behind each

> Was it solved before? How is your solution different?

There are already libraries that implpement our algorithms and gates. But the problem in question is not just making the libraries, but implementing them in such a way that comparisons can be accurate. By creating modules from scratch, we can better ensure that the time complexities and overheads are the same in both versions.

## Executing the Code

To compile and test the rust code:

```bash
cd code-orig/qcrust
cargo build
cargo run
```

This will run the `main.rs` file which runs tests on the small library we have built. We have also put the test file in the `tests/` directory, but it cannot be run without cargo and its dependencies.

<hr>

To run and test the python code, run:

```bash
cd code-orig/qcpython
python TestQC.py
```

If you don't have numpy installed, you can install it with:

```bash
pip install numpy
```

Again, we have put `TestQC.py` in the `tests/` directory, but importing the `PyQC.py` file is left up to the programmer's folder structure.

## Software Architecture

> What is the software architecture of your soln? What parts have you reused and what parts have you developed on your own? Draw a figure to explain better. Is it a client-server architecture. Where is the testing component placed (local or remote)? Is there a database involved? etc.

This is a comparitive study using two languages:
1. Python
2. Rust

* The code runs completely locally with no servers (no client-server architecture is needed)
* We used the `numpy` library in Python and the `ndarray` crate in Rust
* Kronecker product (essential for Quantum computations) was implemented in Rust whereas it was readily available in the `numpy` module in Python
* `ndarray` package has been used to implement operations in Linear Algebra like matrix multiplication

## POPL Aspects

> What were the POPL aspects involved in the implementation. NOT theoretical answers. Have pointers to the lines of code and explain the POPL ideas/concepts involved and why they are necessary. I expect 5 to 10 points written on POPL aspects (bullet points, one after another). More the points you have the better it is. While writing the points also write your experience of the difficulties you faced.

```rust
use ndarray::{Array1, Array2, ArrayView1, Ix2};
```

* **Memory Safety**: The use of `Array1` and `Array2` from the ndarray crate, along with the ownership system and borrowing rules, contributes to memory safety.

<hr>

```rust
pub struct QCircuit {
    qubits : usize,
    size: usize,
    state: Array1<f64>,
}

impl QCircuit {
    pub fn new(num_qubits: usize) -> Self {
        let qubits = num_qubits;
        let size = 2_usize.pow(num_qubits as u32);
        let mut state = Array1::zeros(size);
        state[[0]] = 1.0; // Set the first element to 1.0
        QCircuit { qubits, size, state }
    }
    
    pub fn get_state(&self) -> ArrayView1<f64> {
        self.state.view()
    }
```

* **Ownership & Borrowing**: The ownership of the quantum state vector in the QCircuit struct and the borrowing in methods like x_gate where the state is mutated

<hr>

```rust
pub struct QCircuit {
    qubits : usize,
    size: usize,
    state: Array1<f64>,
}
```

* **Zero-Cost Abstractions**: The use of high-level abstractions like Array1 and Array2 for quantum states and gates without incurring significant runtime overhead

<hr>

```rust
// X gate implementation
pub fn x_gate(&mut self, target_index: usize) {
    let pauli_x =  Array2::from_shape_vec(Ix2(2, 2), vec![0.0, 1.0, 1.0, 0.0]).unwrap();
    let identity_2 = Array2::from_shape_vec(Ix2(2, 2), vec![1.0, 0.0, 0.0, 1.0]).unwrap();
    let mut x_n;
    if target_index == 0{
        x_n = pauli_x.clone();
        
    }
    else {
        x_n = identity_2.clone();
    }
    for index in 1..self.qubits {
        if index == target_index {
            x_n = QCircuit::kron(&x_n, &pauli_x);
        }
        else {
            x_n = QCircuit::kron(&x_n, &identity_2);
        }
    }
    
    self.state = x_n.dot(&self.state);
}
```

* **Pattern Matching**: The match keyword is not explicitly used in this code, but the structure of the code, especially in functions like x_gate and z_gate, involves conditional logic that can be seen as a form of pattern matching

<hr>

```rust
let pauli_x =  Array2::from_shape_vec(Ix2(2, 2), vec![0.0, 1.0, 1.0, 0.0]).unwrap();
let identity_2 = Array2::from_shape_vec(Ix2(2, 2), vec![1.0, 0.0, 0.0, 1.0]).unwrap();
```

* **Immutable by default**: Variables like pauli_x, identity_2, etc., are declared as immutable by default, promoting a functional programming style

## Results

> Tests conducted. Dataset used. Benchmarks run. Show graphs. Line graphs, bar graphs, etc. How are you checking/validating that these results align with your initial problem statement. Data-driven proof points that the solution/system is working. Why should I be convinced it is working?

| Gates & Algorithms | Execution Time in Rust | Execution Time in Python |
|---|---|---|
| Hadamard Gate | 1.337247838ss | 0.05498323440551758ss |
| X Gate | 1.426829651ss | 0.04555361270904541ss |
| C-NOT Gate | 2.745869783ss | 0.10531795024871826ss |
| Z gate | 1.333971906ss | 0.053670501708984374ss |
| Deutsch Jozsa Algorithm | 0.006171026ss | 0.003129434585571289ss |
| 10 Qbit GHZ State | 26.087039155ss | 1.0489354133605957ss |

The results obtained however are contradictory to the assumptions made in the initial stages of the project. <br>
The code was micro-profiled and it was observed that the pyhton code performed significantly better owing to numpy. <br>
Numpy has some of its implementations based on cython, a C-Extension for python that's well optimized compared to our implementation in Rust. <br>
What is interesting about the results however is the execution times for Deutsch-Jozsa algorithm, here both the Python and Rust implementations perform similarly, this warrants further investigation.

## Potential for Future Work

> If you were given more time what else would you do. What other POPL aspects might come into play?

- Multi-threading and multiprocessing can be used to improve the execution  time of the algorithm.
- Future comparative studies can be made against Qiskit modules that implement the same function/algorithm
- Random measurements can be iintroduced, it refers to measurements made on quantum bits (qubits) that are not predetermined by the program but instead involve some element of randomness, this allows for probabilistic outcomes when measuring the state of a qubit. 
- Random measurements can be leveraged in certain quantum algorithms, such as those involved in quantum random number generation or optimization problems.
- Grover's algorithm, Shor's algorithm, bernstein vazirani algorithm are some algorithms that can be implemented at the back of random measurements
