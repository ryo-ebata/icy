use hcl::structure::Block;
use hcl::Attribute as HclAttribute;
use hcl::BlockLabel;
use hcl::Body;
use hcl::Expression;

#[derive(Debug)]
pub struct AST {
    pub nodes: Vec<ASTNode>,
}

#[derive(Debug)]
pub struct ASTNode {
    pub name: String,
    pub labels: Vec<String>,
    pub attributes: Vec<Attribute>,
    pub blocks: Vec<ASTNode>,
}

#[derive(Debug)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

impl AST {
    /// Create an AST from an HCL body
    ///
    /// This function is used to create an AST from an HCL body.
    ///
    /// # Example
    ///
    /// ```
    /// let body =
    ///    hcl::from_str("key = \"value\"").expect("Failed to parse HCL");
    /// let ast = AST::from_hcl_body(&body);
    /// ```
    ///
    /// # Arguments
    ///
    /// * `body` - The HCL body to convert to an AST
    ///
    /// # Returns
    ///
    /// An AST representing the HCL body
    ///
    pub fn from_hcl_body(body: &Body) -> Self {
        let nodes = body
            .blocks()
            .map(|block| Self::create_node(block))
            .collect();
        AST { nodes }
    }

    /// Create a node from a block
    ///
    /// This function is used to create a node from a block.
    fn create_node(block: &Block) -> ASTNode {
        ASTNode {
            name: block.identifier().to_string(),
            labels: block
                .labels()
                .iter()
                .map(|l| Self::block_label_to_string(l))
                .collect(),
            attributes: block
                .body()
                .attributes()
                .into_iter()
                .map(|attr| Self::hcl_attribute_to_attribute(&attr))
                .collect(),
            blocks: block
                .body()
                .blocks()
                .map(|b| Self::create_node(b))
                .collect(),
        }
    }

    /// Convert a block label to a string
    ///
    /// This function is used to convert a block label to a string.
    fn block_label_to_string(label: &BlockLabel) -> String {
        match label {
            BlockLabel::String(s) => s.to_string(),
            BlockLabel::Identifier(i) => i.to_string(),
        }
    }

    /// Convert an HCL attribute to an attribute
    ///
    /// This function is used to convert an HCL attribute to an attribute that can be used in the AST.
    ///
    /// # Usage
    ///
    /// ```
    /// let attr = HclAttribute::new("key",
    ///    Expression::String("value".to_string()));
    /// let attribute = AST::hcl_attribute_to_attribute(&attr);
    /// ```
    ///
    fn hcl_attribute_to_attribute(attr: &HclAttribute) -> Attribute {
        Attribute {
            key: attr.key().to_string(),
            value: Self::expression_to_string(attr.expr()),
        }
    }

    /// Convert an HCL expression to a string
    ///
    /// This function is used to convert the value of an attribute to a string.
    fn expression_to_string(expr: &Expression) -> String {
        match expr {
            Expression::String(s) => s.to_string(),
            Expression::Number(n) => n.to_string(),
            Expression::Bool(b) => b.to_string(),
            _ => format!("{:?}", expr),
        }
    }
}
