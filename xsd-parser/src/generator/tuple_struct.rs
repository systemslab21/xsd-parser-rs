use std::borrow::Cow;

use crate::{generator::Generator, parser::types::TupleStruct};

pub trait TupleStructGenerator {
    fn generate(&self, entity: &TupleStruct, gen: &Generator) -> String {
        format!(
            "{comment}{macros}pub struct {name} (pub {typename});\n{subtypes}\n\n",
            comment = self.format_comment(entity, gen),
            name = self.get_name(entity, gen),
            macros = self.macros(entity, gen),
            typename = self.get_type_name(entity, gen),
            subtypes = self.subtypes(entity, gen),
        )
    }

    fn subtypes(&self, entity: &TupleStruct, gen: &Generator) -> String {
        gen.base().join_subtypes(entity.subtypes.as_ref(), gen)
    }

    fn get_type_name(&self, entity: &TupleStruct, gen: &Generator) -> String {
        gen.base()
            .modify_type(
                gen.base().format_type_name(entity.type_name.as_str(), gen).as_ref(),
                &entity.type_modifiers,
            )
            .into()
    }

    fn get_name(&self, entity: &TupleStruct, gen: &Generator) -> String {
        gen.base().format_type_name(entity.name.as_str(), gen).into()
    }

    fn macros(&self, _entity: &TupleStruct, _gen: &Generator) -> Cow<'static, str> {
        "#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]\n#[serde(transparent)]\n"
            .into()
    }

    fn format_comment(&self, entity: &TupleStruct, gen: &Generator) -> String {
        gen.base().format_comment(entity.comment.as_deref(), 0)
    }
}

pub struct DefaultTupleStructGen;
impl TupleStructGenerator for DefaultTupleStructGen {}
