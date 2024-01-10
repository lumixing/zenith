use crate::token::{Literal, Token};

trait Node {
    fn token_literal(&self) -> &Option<Literal>;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

struct Program {
    statements: Vec<Box<dyn Statement>>
}

impl Node for Program {
    fn token_literal(&self) -> &Option<Literal> {
        self.statements[0].token_literal()
    }
}

struct Identifier {
    token: Token,
    value: String
}

impl Node for Identifier {
    fn token_literal(&self) -> &Option<Literal> {
        &self.token.literal
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}

struct VarStatement {
    token: Token,
    name: Identifier,
    value: Box<dyn Expression>
}

impl Node for VarStatement {
    fn token_literal(&self) -> &Option<Literal> {
        &self.token.literal
    }
}

impl Statement for VarStatement {
    fn statement_node(&self) {
        todo!()
    }
}

