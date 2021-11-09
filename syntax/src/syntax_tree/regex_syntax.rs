type PtrExpression = Box<Expression>;

#[derive(Debug, Clone)]
pub enum Expression {
    LITERAL(char),
    STAR(PtrExpression),
    UNION(PtrExpression, PtrExpression),
    CONCAT(PtrExpression, PtrExpression),
    EMPTY
}

#[cfg(test)]
mod tests {
    use crate::syntax_tree::regex_syntax::Expression::{LITERAL, STAR, UNION};

    #[test]
    pub fn test_debug_expression() {
        let u = LITERAL('a');
        let s = STAR(Box::new(u.clone()));
        let exp = UNION(Box::new(u), Box::new(s));
        print!("regex: {:?}", exp);
    }

}
