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

*Commands to execute the code here*

## Software Architecture

> What is the software architecture of your soln? What parts have you reused and what parts have you developed on your own? Draw a figure to explain better. Is it a client-server architecture. Where is the testing component placed (local or remote)? Is there a database involved? etc. 

## POPL Aspects

> What were the POPL aspects involved in the implementation. NOT theoretical answers. Have pointers to the lines of code and explain the POPL ideas/concepts involved and why they are necessary. I expect 5 to 10 points written on POPL aspects (bullet points, one after another). More the points you have the better it is. While writing the points also write your experience of the difficulties you faced. 

## Results

> Tests conducted. Dataset used. Benchmarks run. Show graphs. Line graphs, bar graphs, etc. How are you checking/validating that these results align with your initial problem statement. Data-driven proof points that the solution/system is working. Why should I be convinced it is working?

## Potential for Future Work

> If you were given more time what else would you do. What other POPL aspects might come into play?

- Multi-threading and multiprocessing can be used to improve the execution  time of the algorithm.
- Future comparative studies can be made against Qiskit modules that implement the same function/algorithm
- Random measurements can be iintroduced, it refers to measurements made on quantum bits (qubits) that are not predetermined by the program but instead involve some element of randomness, this allows for probabilistic outcomes when measuring the state of a qubit. 
- Random measurements can be leveraged in certain quantum algorithms, such as those involved in quantum random number generation or optimization problems.
- Grover's algorithm, Shor's algorithm, bernstein vazirani algorithm are some algorithms that can be implemented at the back of random measurements
