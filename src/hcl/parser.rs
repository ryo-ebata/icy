use hcl::Body;
use std::error::Error;

/// Parse an HCL string into an HCL body
pub fn parse_hcl(input: &str) -> Result<Body, Box<dyn Error>> {
    let body: Body = hcl::from_str(input)?;
    Ok(body)
}
