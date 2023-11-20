use ndarray::{Array1, Array2, ArrayView1, Ix2};
use std::f64::consts::FRAC_1_SQRT_2;

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
    // Z gate implementation
    pub fn z_gate(&mut self, target_index: usize) {
        let pauli_z =  Array2::from_shape_vec(Ix2(2, 2), vec![1.0, 0.0, 0.0, -1.0]).unwrap();
        let identity_2 = Array2::from_shape_vec(Ix2(2, 2), vec![1.0, 0.0, 0.0, 1.0]).unwrap();
        let mut z_n;
        if target_index == 0{
            z_n = pauli_z.clone();
            
        }
        else {
            z_n = identity_2.clone();
        }
        for index in 1..self.qubits {
            if index == target_index {
                z_n = QCircuit::kron(&z_n, &pauli_z);
            }
            else {
                z_n = QCircuit::kron(&z_n, &identity_2);
            }
        }
        
        self.state = z_n.dot(&self.state);
    }

    // CNOT gate implementation
    pub fn cnot_gate(&mut self, target_index: usize, control_index: usize) {
        let pauli_x =  Array2::from_shape_vec(Ix2(2, 2), vec![0.0, 1.0, 1.0, 0.0]).unwrap();
        let p_0 =  Array2::from_shape_vec(Ix2(2, 2), vec![1.0, 0.0, 0.0, 0.0]).unwrap();
        let p_1 =  Array2::from_shape_vec(Ix2(2, 2), vec![0.0, 0.0, 0.0, 1.0]).unwrap();
        let identity_2 = Array2::from_shape_vec(Ix2(2, 2), vec![1.0, 0.0, 0.0, 1.0]).unwrap();
        let mut cx_n;
        let mut cx_n1;
        let mut cx_n2;
        if target_index == 0{
            cx_n1 = identity_2.clone();
            cx_n2 = pauli_x.clone();
        }
        if control_index == 0{
            cx_n1 = p_0.clone();
            cx_n2 = p_1.clone();
        }
        else {
            cx_n1 = identity_2.clone();
            cx_n2 = identity_2.clone();
        }
        for index in 1..self.qubits {
            if index == target_index {
                cx_n1 = QCircuit::kron(&cx_n1, &identity_2);
                cx_n2 = QCircuit::kron(&cx_n2, &pauli_x);
            }
            else if index == control_index {
                cx_n1 = QCircuit::kron(&cx_n1, &p_0);
                cx_n2 = QCircuit::kron(&cx_n2, &p_1);
            }
            else {
                cx_n1 = QCircuit::kron(&cx_n1, &identity_2);
                cx_n2 = QCircuit::kron(&cx_n2, &identity_2);
            }
        }
        cx_n = &cx_n1 + &cx_n2;
        self.state = cx_n.dot(&self.state);
    }
    
    // Hadamard gate implementation
    pub fn h_gate(&mut self, target_index: usize) {
        let h =  Array2::from_shape_vec(Ix2(2, 2), vec![FRAC_1_SQRT_2, FRAC_1_SQRT_2, FRAC_1_SQRT_2, -FRAC_1_SQRT_2]).unwrap();
        let identity_2 = Array2::from_shape_vec(Ix2(2, 2), vec![1.0, 0.0, 0.0, 1.0]).unwrap();
        let mut x_n;
        if target_index == 0{
            x_n = h.clone();
        }
        else {
            x_n = identity_2.clone();
        }
        for index in 1..self.qubits {
            if index == target_index {
                x_n = QCircuit::kron(&x_n, &h);
            }
            else {
                x_n = QCircuit::kron(&x_n, &identity_2);
            }
        }
        
        self.state = x_n.dot(&self.state);
    }
    
    // Kronecker product implementation
    pub fn kron(a: &Array2<f64>, b: &Array2<f64>) -> Array2<f64> {
        let dima = a.shape()[0];
        let dimb = b.shape()[0];
        let dimout = dima * dimb;
        let mut out = Array2::zeros((dimout, dimout));
        for (mut chunk, elem) in out.exact_chunks_mut((dimb, dimb)).into_iter().zip(a.iter()) {
            chunk.assign(&(*elem * b));
        }
        out
    }
}
