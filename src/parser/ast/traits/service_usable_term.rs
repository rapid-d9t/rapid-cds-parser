use super::ast_term::ASTTerm;
use super::service_term_type::ServiceTermType;

pub trait ServiceUsableTerm: ASTTerm {
    fn get_name(&self) -> String;

    fn get_type(&self) -> ServiceTermType;
}
