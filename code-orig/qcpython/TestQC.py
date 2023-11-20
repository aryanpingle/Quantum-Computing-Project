import numpy as np
import time
from PyQC import qCircuit

#X gate in a 10 qubit circuit
def test_x(circ_x):
    circ_x.x_gate(0)

#H gate in a 10 qubit circuit
def test_h(circ_x):
    circ_x.h_gate(0)

#Z gate in a 10 qubit circuit
def test_z(circ_x):
    circ_x.z_gate(0)

#CNOT gate in a 10 qubit circuit
def test_cnot(circ_x):
    circ_x.cnot_gate(0, 1)

#10 qubit GHZ state
def test_ghz(circ_x, size):
    circ_x.h_gate(0)
    for i in range(size):
        circ_x.cnot_gate(0, 1)

#Deutsch-Jozsa algorithm 
def test_dj(circ_dj):
    circ_dj.h_gate(0)
    circ_dj.h_gate(1)
    circ_dj.h_gate(2)
    circ_dj.x_gate(3)
    circ_dj.x_gate(0)
    circ_dj.x_gate(2)
    circ_dj.h_gate(3)
    circ_dj.cnot_gate(0, 3)
    circ_dj.cnot_gate(1, 3)
    circ_dj.cnot_gate(2, 3)
    circ_dj.x_gate(0)
    circ_dj.x_gate(2)
    circ_dj.h_gate(0)
    circ_dj.h_gate(1)
    circ_dj.h_gate(2)

tim_h = []
for i in range(10):
    start_h = time.time()
    circ_h = qCircuit(10)
    test_h(circ_h)
    end_h = time.time()
    tim_h.append((end_h-start_h))
tim_h = np.array(tim_h)
print("Time elapsed for Hadamard gate:", np.average(tim_h))
print("")

tim_x = []
for i in range(10):
    start_x = time.time()
    circ_x = qCircuit(10)
    test_x(circ_x)
    end_x = time.time()
    tim_x.append((end_x-start_x))
tim_x = np.array(tim_x)
print("Time elapsed for X gate:", np.average(tim_x))
print("")


tim_cx = []
for i in range(10):
    start_cx = time.time()
    circ_cx = qCircuit(10)
    test_cnot(circ_cx)
    end_cx = time.time()
    tim_cx.append((end_cx-start_cx))
tim_cx = np.array(tim_cx)
print("Time elapsed for CNOT gate:", np.average(tim_cx))
print("")


tim_z = []
for i in range(10):
    start_z = time.time()
    circ_z = qCircuit(10)
    test_z(circ_z)
    end_z = time.time()
    tim_z.append((end_z-start_z))
tim_z = np.array(tim_z)
print("Time elapsed for Z gate:", np.average(tim_z))
print("")


tim_ghz = []
for i in range(10):
    start_ghz = time.time()
    circ_ghz = qCircuit(10)
    test_ghz(circ_ghz, 10)
    end_ghz = time.time()
    tim_ghz.append((end_ghz-start_ghz))
tim_ghz = np.array(tim_ghz)
print("Time elapsed for preparing 10 qubit GHZ state:", np.average(tim_ghz))
print("")


tim_dj = []
for i in range(10):
    start_dj = time.time()
    circ_dj = qCircuit(4)
    test_dj(circ_dj)
    end_dj = time.time()
    tim_dj.append((end_dj-start_dj))
tim_dj = np.array(tim_dj)

print("Time elapsed for Deutsch-Jozsa algorithm :", np.average(tim_dj))
print("")
