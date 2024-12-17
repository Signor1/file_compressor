# **Rust File Compressor and Decompressor**

## **Project Description**  
This is a simple command-line tool written in **Rust** that allows you to compress and decompress files using the **Gzip** format. The tool is designed for efficiency, providing file size reports and execution time for each operation.  

---

## **Features**
- **File Compression**: Compress any file into Gzip format (`.gz`).
- **File Decompression**: Decompress `.gz` files back to their original state.
- **Performance Reporting**:  
   - Displays file sizes before and after compression.  
   - Shows time taken to complete the operation.  

---

## **Requirements**
- **Rust** installed on your system. (Follow [Rust installation guide](https://www.rust-lang.org/tools/install) if not already installed.)

---

## **How to Clone and Set Up the Project**

1. **Clone the Repository**:  
   Use the following command to clone the project to your local machine:
   ```bash
   git clone https://github.com/Signor1/rust_file_compressor.git
   cd rust_file_compressor
   ```

2. **Build the Project**:  
   Use `cargo build` to build the project:
   ```bash
   cargo build --release
   ```

3. **Run the Executable**:  
   After building, the binary can be found in the `target/release/` directory.

---

## **How to Use**

### **1. Compress a File**  
Run the following command to compress a file.  
**Usage**:  
```bash
cargo run --release -- compress <input_file> <output_file.gz>
```

**Example**:  
To compress the included `input.txt` file to `compressed.gz`:  
```bash
cargo run --release -- compress input.txt compressed.gz
```

**Output**:
```
Original size: 7143 bytes
Compressed size: 2347 bytes
Compression completed in 9.494892ms
```

### **2. Decompress a File**  
Run the following command to decompress a file.  
**Usage**:  
```bash
cargo run --release -- decompress <input_file.gz> <output_file>
```

**Example**:  
To decompress `compressed.gz` back to `output.txt`:  
```bash
cargo run --release -- decompress compressed.gz output.txt
```

**Output**:
```
Decompressed file size: 7143 bytes
Decompression completed in 534.788µs
```

---

## **Testing the Tool**

### **Included Sample File (`input.txt`)**  
The repository already includes a sample file named `input.txt` that you can use for testing compression and decompression:

1. **Compress `input.txt`**:
   ```bash
   cargo run --release -- compress input.txt compressed.gz
   ```

2. **Verify Compression**:
   Check that `compressed.gz` is created:
   ```bash
   ls -lh compressed.gz
   ```

3. **Decompress `compressed.gz`**:
   ```bash
   cargo run --release -- decompress compressed.gz output.txt
   ```

4. **Verify Decompression**:
   Compare `input.txt` and `output.txt`:
   ```bash
   diff input.txt output.txt
   ```

   If there is no output, the files are identical.

---

## **Error Handling**
- Ensure the correct number of arguments are provided:
   - **Compress**: Requires 3 arguments (`compress`, `input_file` and `output_file.gz`).
   - **Decompress**: Requires 3 arguments (`decompress`, `input_file.gz` and `output_file`).

   Example error:
   ```
   Usage: compress <source> <target> OR decompress <source> <target>
   ```

- The program will exit gracefully if a file does not exist or permissions are insufficient.

---

## **Dependencies**
This project uses the following crate:
- [flate2](https://docs.rs/flate2/latest/flate2/) - Provides Gzip compression and decompression.

---

## **Contribution**  
Feel free to fork this repository, make improvements, and submit a pull request. Suggestions and issues are welcome!

---

## **License**  
This project is licensed under the MIT License.  

---

## **Contact**  
For any questions or feedback, feel free to reach out:  
- **GitHub**: [Signor1](https://github.com/signor1)
