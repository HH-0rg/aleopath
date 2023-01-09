use alloc::string::String;

pub(crate) trait Assembly {
    fn assembly(&self) -> String;
}