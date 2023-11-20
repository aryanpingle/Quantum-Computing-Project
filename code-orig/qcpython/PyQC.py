import numpy as np
import time

class qCircuit:
    def __init__(self, numQubits):
        self.size = numQubits #size of the quantum circuit
        self.state = np.zeros(2**numQubits).reshape(-1, 1) 
        self.state[0] = 1 #initializing the n qubit circuit to |000...> state

    def get_state(self):
        return self.state #return state of the circuit

    def x_gate(self, targetIndex):
        X_n = 1
        X_1 = np.array([[0, 1],
                        [1, 0]]) #one qubit not gate

        for index in range(self.size): #padding the gates with idenitities 
            if(index == targetIndex):
                X_n = np.kron(X_n, X_1)
            else:
                X_n = np.kron(X_n, np.identity(2))
                
    def z_gate(self, targetIndex):
        Z_n = 1
        Z_1 = np.array([[1, 0],
                        [0, -1]]) #one qubit Z gate

        for index in range(self.size): #padding the gates with idenitities 
            if(index == targetIndex):
                Z_n = np.kron(Z_n, Z_1)
            else:
                Z_n = np.kron(Z_n, np.identity(2))
        self.state = Z_n @ self.state #applying the gate to the state 

    def h_gate(self, targetIndex):
        H_n = 1
        H_1 = np.sqrt(0.5)*np.array([[1,  1],
                                     [1, -1]]) #one qubit hadamard

        for index in range(self.size): #padding the gates with idenitities 
            if(index == targetIndex):
                H_n = np.kron(H_n, H_1)
            else:
                H_n = np.kron(H_n, np.identity(2))
        self.state = H_n @ self.state #applying the gate to the state 

    def cnot_gate(self, controlIndex, targetIndex):
        X_1 = np.array([[0, 1],
                        [1, 0]]) #single qubit not
        P_0 = np.array([[1, 0],
                        [0, 0]]) #|0><0| projector
        P_1 = np.array([[0, 0],
                        [0, 1]]) #|1><1| projector
        CNOT_n = 1
        CNOT_n1 = 1
        CNOT_n2 = 1
        for index in range(self.size): #padding the gates with idenitities 
            if(index == targetIndex):
                CNOT_n1 = np.kron(CNOT_n1, np.identity(2))
                CNOT_n2 =  np.kron(CNOT_n2, X_1)
            elif(index == controlIndex):
                CNOT_n1 = np.kron(CNOT_n1, P_0)
                CNOT_n2 = np.kron(CNOT_n2, P_1)
            else:
                CNOT_n1 = np.kron(CNOT_n1, np.identity(2))
                CNOT_n2 = np.kron(CNOT_n2, np.identity(2))
        CNOT_n = CNOT_n1 + CNOT_n2 #CNOT = |0><0| x I + |1><1| x X (x is Kroneker product)
        self.state = CNOT_n @ self.state
