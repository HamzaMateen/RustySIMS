### RustySIMS: An Inventory Management System


### Project Details
**RustySIMS** is a command-line inventory management system designed for product tracking and organization. It enables managers with the following capabilities:


- **Product Management:** *Add, delete, edit, and view* detailed information about products in your inventory.

- **Manager Accounts:** Create and manage accounts for multiple managers.

- **Foundational Database:** Leverages **rusqlite** (Sqlite) to persistently store inventory data.

- **Secure Password Hashing:** Employs the industry-standard **bcrypt** library to securely hash and store manager passwords.


**Usage**

1. **Installation:**
   - Ensure you have Rust and cargo installed:
     https://www.rust-lang.org/tools/install
     
   - Clone this repository:
     ```bash
     git clone https://github.com/HamzaMateen/RustySIMS
     ```
   - Navigate to the project directory:
     ```bash
     cd rusty_sims
     ```
   - Build the project:
     ```bash
     cargo build
     ```


2. **Running the Application:**
   - Execute the compiled binary:
     ```bash
     ./target/debug/rusty_sims
     ```


**Demo**

![one](./demo/1.png)
![two](./demo/2.png)
![three](./demo/3.png)
![four](./demo/4.png)


### Thank You!
