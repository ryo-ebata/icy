mod drawio;
mod hcl;

use clap::Parser;
use std::fs;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(required = true)]
    hcl_file: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read the HCL file
    let hcl_content = fs::read_to_string(&args.hcl_file)?;

    match hcl::parser::parse_hcl(&hcl_content) {
        Ok(body) => {
            // Convert the HCL AST to a Draw.io diagram
            let ast = hcl::ast::AST::from_hcl_body(&body);
            match drawio::output::ast_to_drawio(&ast) {
                Ok(drawio_output) => {
                    // Save the Draw.io diagram to a file
                    let drawio_file_name = args
                        .hcl_file
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned()
                        + ".drawio";

                    let drawio_output_path = args.hcl_file.parent().unwrap().join(drawio_file_name);

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
