use std::borrow::Cow;

use crate::{
    generator::Generator,
    parser::types::{Enum, EnumSource},
};

pub trait EnumGenerator {
    fn generate(&self, entity: &Enum, gen: &Generator) -> String {
        let name = self.get_name(entity, gen);

        format!(
            "{comment}{macros}\n\
            pub enum {name} {{\n\
                {cases}\n\
            }}\n\
            {subtypes}\n\n",
            comment = self.format_comment(entity, gen),
            macros = self.macros(entity, gen),
            name = name,
            cases = self.cases(entity, gen),
            subtypes = self.subtypes(entity, gen),
        )
    }

    fn cases(&self, entity: &Enum, gen: &Generator) -> String {
        entity
            .cases
            .iter()
            .map(|case| gen.enum_case_gen().generate(case, gen))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn subtypes(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().join_subtypes(entity.subtypes.as_ref(), gen)
    }

    fn get_type_name(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().format_type_name(entity.type_name.as_str(), gen).into()
    }

    fn get_name(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().format_type_name(entity.name.as_str(), gen).into()
    }

    fn macros(&self, entity: &Enum, gen: &Generator) -> Cow<'static, str> {
        if entity.source == EnumSource::Union {
            return "#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]\n#[serde(untagged)]".into();
        }

        let derives = "#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]";
        let _tns = gen.target_ns.borrow();
        derives.into()
    }

    fn format_comment(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().format_comment(entity.comment.as_deref(), 0)
    }
}

pub struct DefaultEnumGen;
impl EnumGenerator for DefaultEnumGen {}
