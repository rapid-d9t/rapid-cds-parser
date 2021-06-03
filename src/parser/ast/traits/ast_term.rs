pub trait ASTTerm {
    fn convert_to_json(&self) -> String;
}
