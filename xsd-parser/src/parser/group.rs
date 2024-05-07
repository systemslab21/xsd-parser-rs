use std::cell::RefCell;

use roxmltree::Node;

use crate::parser::{
    types::RsEntity,
    xsd_elements::{ElementType, XsdNode},
};

use super::{
    constants::tag,
    node_parser::parse_node,
    types::{Struct, StructField, StructFieldSource},
    utils::{attribute_groups_to_aliases, attributes_to_fields, get_documentation},
};

const AVAILABLE_CONTENT_TYPES: [ElementType; 3] =
    [ElementType::All, ElementType::Choice, ElementType::Sequence];

pub fn parse_group(node: &Node, parent: &Node) -> RsEntity {
    if parent.xsd_type() == ElementType::Schema {
        let name = node.attr_name().expect("Outer groups must have name");

        let mut fields = attributes_to_fields(node);

        let content = node
            .children()
            .filter(|n| n.is_element() && AVAILABLE_CONTENT_TYPES.contains(&n.xsd_type()))
            .last();

        if content.is_none() || content.unwrap().children().filter(|n| n.is_element()).count() == 0
        {
            //No content (or empty), only attributes

            return RsEntity::Struct(Struct {
                fields: RefCell::new(fields),
                attribute_groups: RefCell::new(attribute_groups_to_aliases(node)),
                comment: get_documentation(node),
                subtypes: vec![],
                name: name.to_string(),
            });
        }
        let content_node = content.unwrap();

        let mut res = parse_node(&content_node, node);
        match &mut res {
            RsEntity::Struct(st) => {
                st.fields.borrow_mut().append(&mut fields);
                st.name = name.to_string();
            }
            RsEntity::Enum(en) => {
                en.name = format!("{}Choice", name);
                fields.push(StructField {
                    name: en.name.clone(),
                    type_name: en.name.clone(),
                    source: StructFieldSource::Choice,
                    ..Default::default()
                });
                en.subtypes = vec![RsEntity::Struct(Struct {
                    name: name.to_string(),
                    subtypes: vec![],
                    comment: get_documentation(node),
                    fields: RefCell::new(fields),
                    attribute_groups: RefCell::new(attribute_groups_to_aliases(node)),
                })];
            }
            _ => (),
        };
        res
    } else {
        let base = node.attr_ref().expect("Inner groups have to reference outer groups");
        let field = StructField {
            name: tag::BASE.to_string(),
            type_name: base.to_string(),
            source: StructFieldSource::Base,
            ..Default::default()
        };
        RsEntity::StructField(field)
    }
}
