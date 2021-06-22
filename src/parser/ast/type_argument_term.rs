use crate::ir::ir_component::IRComponent;
use crate::parser::ast::traits::ast_term::ASTTerm;

pub struct TypeArgumentTerm {
    value: u64,
}

impl ASTTerm for TypeArgumentTerm {
    fn generate_ir(&self) -> Box<IRComponent> {
        Box::new(IRComponent::new_number(self.value as f64))
    }
}

impl TypeArgumentTerm {
    pub fn new_boxed(value: u64) -> Box<TypeArgumentTerm> {
        Box::new(TypeArgumentTerm::new(value))
    }

    pub fn new(value: u64) -> TypeArgumentTerm {
        TypeArgumentTerm { value }
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::ir_component::IRComponent;
    use crate::parser::ast::traits::ast_term::ASTTerm;
    use crate::parser::ast::type_argument_term::TypeArgumentTerm;

    #[test]
    fn it_generates_ir() {
        let type_argument_term = TypeArgumentTerm::new_boxed(1234);

        let generated_ir = type_argument_term.generate_ir();

        let correct_ir = IRComponent::new_number(1234.);

        assert_eq!(generated_ir, Box::new(correct_ir))
    }
}
