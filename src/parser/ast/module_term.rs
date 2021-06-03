use super::traits::ast_term::ASTTerm;
use super::traits::module_usable_term::ModuleUsableTerm;

pub struct ModuleTerm {
    name: String,
    definitions: Vec<Box<dyn ModuleUsableTerm>>,
}

impl ModuleTerm {
    pub fn new(definitions: Vec<Box<dyn ModuleUsableTerm>>) -> ModuleTerm {
        ModuleTerm {
            name: "".to_string(),
            definitions,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, new_name: &String) {
        self.name = new_name.clone();
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

impl ASTTerm for ModuleTerm {
    fn convert_to_json(&self) -> String {
        format!(
            "
{{
    type: \"module\",
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
