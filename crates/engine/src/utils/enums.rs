pub trait StrEnum: Sized {
    type Error;
    fn to_str(&self) -> &str;
    fn from_str(value: &str) -> Result<Self, Self::Error>;
}
