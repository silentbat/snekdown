#[macro_export]
macro_rules! parse {
    ($str:expr) => {
        SingleStepParser::new($str.to_string(), None).parse()
    };
}
