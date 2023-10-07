# abc-practice-rs

## About This Repository
This repository is dedicated to practicing the solution of AtCoder problems using the Rust programming language.

## How to Setup

### Prerequisites

- Ensure that [Rust](https://www.rust-lang.org/) is installed on your system.

### Steps
1. **Clone the Repository**
   ```sh
   git clone https://github.com/yuta0428/abc-practice-rs.git
   cd abc-practice-rs
   ```

2. **Install the `oj` Command**

   Run `install-oj.sh` to install the [oj](https://github.com/online-judge-tools/oj) command.
   ```bash
   sh ./install-oj.sh
   ```
   
## How to Solve Problems
1. **Run the Program**

   Run `cargo run` and input the contest number and problem number when prompted.
   ```bash
   cargo run
   ```
   
2. **Implement the Answer**
   
   - Open the created folder and implement your answer in `main.rs`.
   - Note: To utilize VSCode’s rust-analyzer, it is recommended to open the target folder and work from there, e.g., `code abc001/a`.

3. **Check Test Cases**
   
   After implementing your answer, run `ojtest.sh` to validate it against the test cases.
   ```bash
   sh ./ojtest.sh
   ```
   
4. **Submit the Answer**
   
   If there are no issues with the test cases, run `ojsubmit.sh` to submit your answer.
   ```bash
   sh ./ojsubmit.sh
   ```
