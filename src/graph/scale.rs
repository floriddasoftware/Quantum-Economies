#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum Scale {
    Minimal,
    Local,
    Recursive,
    Global,
}