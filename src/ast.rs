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
    pub fn from_hcl_body(body: &Body) -> Self {
        let nodes = body
            .blocks()
            .map(|block| Self::create_node(block))
            .collect();
        AST { nodes }
    }

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

    fn block_label_to_string(label: &BlockLabel) -> String {
        match label {
            BlockLabel::String(s) => s.to_string(),
            BlockLabel::Identifier(i) => i.to_string(),
        }
    }

    fn hcl_attribute_to_attribute(attr: &HclAttribute) -> Attribute {
        Attribute {
            key: attr.key().to_string(),
            value: Self::expression_to_string(attr.expr()),
        }
    }

    fn expression_to_string(expr: &Expression) -> String {
        match expr {
            Expression::String(s) => s.to_string(),
            Expression::Number(n) => n.to_string(),
            Expression::Bool(b) => b.to_string(),
            _ => format!("{:?}", expr), // その他の式タイプの場合はデバッグ出力を使用
        }
    }
}
