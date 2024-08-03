use crate::hcl::ast::{ASTNode, AST};
use std::io::Cursor;
use xml::writer::{EmitterConfig, EventWriter, XmlEvent};

pub fn ast_to_drawio(ast: &AST) -> Result<String, xml::writer::Error> {
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(Cursor::new(Vec::new()));

    writer.write(XmlEvent::start_element("mxfile").attr("host", "app.diagrams.net"))?;
    writer.write(XmlEvent::start_element("diagram"))?;
    writer.write(XmlEvent::start_element("mxGraphModel"))?;
    writer.write(XmlEvent::start_element("root"))?;

    writer.write(XmlEvent::start_element("mxCell").attr("id", "0"))?;
    writer.write(XmlEvent::end_element())?;
    writer.write(
        XmlEvent::start_element("mxCell")
            .attr("id", "1")
            .attr("parent", "0"),
    )?;
    writer.write(XmlEvent::end_element())?;

    let mut id = 2;
    for node in &ast.nodes {
        draw_node(&mut writer, node, &mut id, 1)?;
    }

    writer.write(XmlEvent::end_element())?; // root
    writer.write(XmlEvent::end_element())?; // mxGraphModel
    writer.write(XmlEvent::end_element())?; // diagram
    writer.write(XmlEvent::end_element())?; // mxfile

    let result = writer.into_inner().into_inner();
    Ok(String::from_utf8(result).unwrap())
}

/// Draw a node in the Draw.io XML
///
/// This function is used to draw a node in the Draw.io XML.
///
/// # Arguments
///
/// * `writer` - The XML writer to write to
/// * `node` - The AST node to draw
/// * `id` - The ID of the node
/// * `parent` - The ID of the parent node
///
/// # Returns
///
/// An XML writer with the node drawn
///
/// # Example
///
/// ```
/// let mut writer = EmitterConfig::new()
///    .perform_indent(true)
///   .create_writer(Cursor::new(Vec::new()));
/// let node = ASTNode {
///   name: "node".to_string(),
///  labels: vec!["label1".to_string(), "label2".to_string()],
/// attributes: vec![Attribute { key: "key".to_string(), value: "value".to_string() }],
/// blocks: vec![],
/// };
/// draw_node(&mut writer, &node, &mut 2, 1);
/// ```
///
fn draw_node(
    writer: &mut EventWriter<Cursor<Vec<u8>>>,
    node: &ASTNode,
    id: &mut i32,
    parent: i32,
) -> Result<(), xml::writer::Error> {
    let node_id = *id;
    *id += 1;

    let x = 100 + (node_id - 2) * 200;
    let y = (parent - 1) * 100;

    writer.write(
        XmlEvent::start_element("mxCell")
            .attr("id", &node_id.to_string())
            .attr(
                "value",
                &format!("{}\n{}", node.name, node.labels.join(", ")),
            )
            .attr("style", "rounded=1;whiteSpace=wrap;html=1;")
            .attr("vertex", "1")
            .attr("parent", &parent.to_string()),
    )?;

    writer.write(
        XmlEvent::start_element("mxGeometry")
            .attr("x", &x.to_string())
            .attr("y", &y.to_string())
            .attr("width", "120")
            .attr("height", "60")
            .attr("as", "geometry"),
    )?;
    writer.write(XmlEvent::end_element())?; // mxGeometry
    writer.write(XmlEvent::end_element())?; // mxCell

    for (i, attr) in node.attributes.iter().enumerate() {
        let attr_id = *id;
        *id += 1;
        writer.write(XmlEvent::start_element("mxCell")
            .attr("id", &attr_id.to_string())
            .attr("value", &format!("{}: {}", attr.key, attr.value))
            .attr("style", "text;html=1;strokeColor=none;fillColor=none;align=left;verticalAlign=middle;whiteSpace=wrap;rounded=0;")
            .attr("vertex", "1")
            .attr("parent", &node_id.to_string()))?;

        writer.write(
            XmlEvent::start_element("mxGeometry")
                .attr("x", "0")
                .attr("y", &(80 + i * 20).to_string())
                .attr("width", "120")
                .attr("height", "20")
                .attr("as", "geometry"),
        )?;
        writer.write(XmlEvent::end_element())?; // mxGeometry
        writer.write(XmlEvent::end_element())?; // mxCell
    }

    for child in &node.blocks {
        draw_node(writer, child, id, node_id)?;
    }

    Ok(())
}
