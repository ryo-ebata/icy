mod ast;
mod drawio_writer;
mod parser;

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <hcl_file>", args[0]);
        std::process::exit(1);
    }

    let hcl_file = Path::new(&args[1]);
    let hcl_content = fs::read_to_string(hcl_file)?;

    match parser::parse_hcl(&hcl_content) {
        Ok(body) => {
            let ast = ast::AST::from_hcl_body(&body);
            match drawio_writer::ast_to_drawio(&ast) {
                Ok(drawio_output) => {
                    // Draw.ioファイルの出力パスを生成
                    let drawio_file_name =
                        hcl_file.file_stem().unwrap().to_str().unwrap().to_owned() + ".drawio";
                    let drawio_output_path = hcl_file.parent().unwrap().join(drawio_file_name);

                    // Draw.io XMLをファイルに書き込む
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
