pub mod slack;

pub trait PrettyPrint {
    fn to_string_pretty(&self) -> String;
}
