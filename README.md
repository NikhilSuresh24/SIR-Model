# SIR-Model
An Implementation of an SIRD Model in Rust

    S - Susceptible Population
    I - Infected Population
    R - Recovered Population
    D - Dead Population

To run the code:
  
    cargo run 400 0.0005 0.05 0.005 0.00 0.1
  
Where, in this case:  

    Total Population = 400
    Infection Rate = 0.0005
    Recovery Rate = 0.05
    Death Rate = 0.005
    Vaccination Rate = 0.00
    dt = 0.1

Feel free to change the parameters to model different diseses
