#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Id(pub String);

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(attribute, prefix = "tns", rename = "id")]
    pub id: Option<Id>,
}
