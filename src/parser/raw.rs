use error::TSError;
use parser::utils::{Parsed, Parser as ParserT};

/// The Raw Parser is a simple parser that performs no action on the
/// data and just hands on `raw`
pub struct Parser {}
impl Parser {
    pub fn new(_opts: &str) -> Self {
        Self {}
    }
}
impl ParserT for Parser {
    fn parse(&self, msg: String) -> Result<Parsed, TSError> {
        Ok(Parsed::new(msg))
    }
}
