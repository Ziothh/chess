pub trait StrEnum {
    fn to_str(&self) -> &str;
    fn from_str(value: &str) -> anyhow::Result<Self>;
}
