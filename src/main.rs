mod ast;
mod drawio_writer;
mod parser;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // Check if the user provided a file
    if args.len() != 2 {
        eprintln!("Usage: {} <hcl_file>", args[0]);
        std::process::exit(1);
    }

    // Read the HCL file
    let hcl_file = Path::new(&args[1]);
    let hcl_content = fs::read_to_string(hcl_file)?;

    match parser::parse_hcl(&hcl_content) {
        Ok(body) => {
            // Convert the HCL AST to a Draw.io diagram
            let ast = ast::AST::from_hcl_body(&body);
            match drawio_writer::ast_to_drawio(&ast) {
                Ok(drawio_output) => {
                    // Save the Draw.io diagram to a file
                    let drawio_file_name =
                        hcl_file.file_stem().unwrap().to_str().unwrap().to_owned() + ".drawio";
                    let drawio_output_path = hcl_file.parent().unwrap().join(drawio_file_name);

                    // Write the Draw.io XML to the file
                    let mut file = fs::File::create(&drawio_output_path)?;
                    file.write_all(drawio_output.as_bytes())?;

                    println!("Draw.io diagram saved to: {}", drawio_output_path.display());
                }
                Err(e) => eprintln!("Error converting AST to Draw.io XML: {}", e),
            }
        }
        Err(e) => eprintln!("Error parsing HCL: {}", e),
    }

    Ok(())
}
