# VRF Oracle

This repository contains the necessary code for deploying a Verifiable Random Function (VRF) Oracle. The project utilizes Rust for the core logic and Noir for the cryptographic circuits.

## Repository Structure

### 1. Core
This folder contains the main Rust code for the VRF Oracle.

**Key Features:**
- Implements the VRF functionality in Rust.
- Provides APIs for generating and verifying random values.
- Handles interaction with the Noir circuit for cryptographic proofs.

**Setup Instructions:**
1. Navigate to the `repo` folder:
   ```bash
   cd <repo-location>
   ```
2. Ensure you have Rust installed:
   - Install Rust: [https://rustup.rs/](https://rustup.rs/)
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Run the VRF service:
   ```bash
   cargo run
   ```

### 2. Circuits
This folder contains Noir-based cryptographic circuits used in the VRF Oracle.

**Key Features:**
- Defines the mathematical circuit for generating zero-knowledge proofs.
- Verifies VRF outputs against the provided public inputs.

**Setup Instructions:**
1. Navigate to the `circuits` folder:
   ```bash
   cd circuits
   ```
2. Install Noir:
   - Follow the official Noir installation guide: [https://noir-lang.org/](https://noir-lang.org/)
3. Compile the circuit:
   ```bash
   nargo compile
   ```
4. Generate and verify proofs:
   - To generate a proof:
     ```bash
     nargo prove
     ```
   - To verify a proof:
     ```bash
     nargo verify
     ```


## Prerequisites
- **Rust** (latest stable version)
- **Cargo** (comes with Rust installation)
- **Noir** (for compiling and interacting with circuits)

## Running the Project
1. Build and run the Rust core code to start the VRF service.
2. Compile the Noir circuits and generate proofs.


## Contributions
- Contributions are welcome! Feel free to fork the repository and submit pull requests.
- Open issues for bugs or feature requests.

## License
This project is licensed under the MIT License. See the `LICENSE` file for more details.

---

Feel free to reach out for support or feedback regarding the VRF Oracle!

