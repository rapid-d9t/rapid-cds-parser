use crate::ir::ir_component::IRComponent;

pub trait ASTTerm {
    fn generate_ir(&self) -> Box<dyn IRComponent>;
}
