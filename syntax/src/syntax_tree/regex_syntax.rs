type PtrExpr = Box<Expression>;

#[derive(Debug, Clone)]
pub enum Expression {
    CONST(char),
    STAR(PtrExpr),
    UNION(PtrExpr, PtrExpr),
    CONCAT(PtrExpr, PtrExpr),
    EMPTY
}

#[cfg(test)]
mod tests {
    use crate::syntax_tree::regex_syntax::Expression::{CONST, STAR, UNION};

    #[test]
    pub fn test_debug_expression() {
        let u = CONST('a');
        let s = STAR(Box::new(u.clone()));
        let exp = UNION(Box::new(u), Box::new(s));
        print!("regex: {:?}", exp);
    }

}
