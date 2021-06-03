use super::name_term::NameTerm;
use super::traits::ast_term::ASTTerm;
use super::traits::module_usable_term::ModuleUsableTerm;
use super::traits::service_usable_term::ServiceUsableTerm;

pub struct ServiceTerm {
    name: NameTerm,
    applied_aspects: Vec<NameTerm>,
    definitions: Vec<Box<dyn ServiceUsableTerm>>,
}

impl ServiceTerm {
    pub fn new(
        name: NameTerm,
        applied_aspects: Vec<NameTerm>,
        definitions: Vec<Box<dyn ServiceUsableTerm>>,
    ) -> ServiceTerm {
        ServiceTerm {
            name,
            applied_aspects,
            definitions,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.get_value()
    }

    fn build_definitions_json(&self) -> String {
        let mut definitions_json = String::new();

        for definition in &self.definitions {
            definitions_json.push_str("\t");
            definitions_json.push_str(&definition.convert_to_json());
            definitions_json.push_str(",\n");
        }

        definitions_json
    }
}

impl ModuleUsableTerm for ServiceTerm {}

impl ASTTerm for ServiceTerm {
    fn convert_to_json(&self) -> String {
        format!(
            "
{{
    type: \"service\",
    name: \"{}\",
    declarations: [
    {}
    ]
}}",
            self.get_name(),
            self.build_definitions_json(),
        )
    }
}
