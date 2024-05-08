#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct BarType {
    #[yaserde(attribute, rename = "a")]
    pub a: Option<String>,
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "Bar")]
    pub bar: foo_type::BarType,
}

pub mod foo_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
    pub struct BarType {
        #[yaserde(attribute, rename = "b")]
        pub b: Option<String>,

        #[yaserde(attribute, rename = "a")]
        pub a: Option<String>,
    }
}
