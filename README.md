<h1 align="center">Icy</h1>


This tool parses HashiCorp Configuration Language (HCL) files and generates a Draw.io compatible diagram, representing the structure of the HCL configuration.

## Features

- Parses HCL files using the `hcl-rs` crate
- Generates an Abstract Syntax Tree (AST) from the parsed HCL
- Converts the AST to a Draw.io compatible XML format
- Saves the generated diagram as a `.drawio` file

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/ryo-ebata/icy.git
   cd icy
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the tool with the path to your HCL file as an argument:

```
cargo run --release -- path/to/your/file.hcl
```

The tool will generate a `.drawio` file in the same directory as the input HCL file.

## Example

Given an HCL file `example.hcl`:

```hcl
resource "aws_instance" "example" {
  ami           = "ami-12345678"
  instance_type = "t2.micro"

  tags = {
    Name = "example-instance"
  }
}
```

Run the tool:

```
cargo run --release -- example.hcl
```

This will generate `example.drawio` in the same directory, which can be opened with Draw.io.

## Dependencies

- `hcl-rs`: For parsing HCL files
- `xml-rs`: For generating XML output

For a complete list of dependencies, see the `Cargo.toml` file.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache License-2.0 License - see the LICENSE file for details.

## Acknowledgments

- HashiCorp for the HCL format
- The Rust community for excellent libraries and tools
