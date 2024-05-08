#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "Once")]
    pub once: i32,

    #[yaserde(prefix = "tns", rename = "Optional")]
    pub optional: Option<i32>,

    #[yaserde(prefix = "tns", rename = "OnceSpecify")]
    pub once_specify: i32,

    #[yaserde(prefix = "tns", rename = "TwiceOrMore")]
    pub twice_or_more: Vec<i32>,
}

// pub type Foo = FooType;
