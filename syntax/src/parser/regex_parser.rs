use crate::syntax_tree::regex_syntax::Expression;

trait Parser {
    fn parse(s: &str) -> Expression;
}