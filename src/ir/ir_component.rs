use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;
use std::collections::HashMap;

#[derive(std::fmt::Debug, PartialEq)]
#[cfg(not(tarpaulin_include))]
pub enum IRComponent {
    Object {
        fields: HashMap<String, Box<IRComponent>>,
    },
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Box<IRComponent>>),
}

#[cfg(not(tarpaulin_include))]
impl IRComponent {
    pub fn to_js_value<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
    ) -> Result<neon::handle::Handle<'internal, JsValue>, IRError> {
        match self {
            IRComponent::Object { fields } => {
                let mut object = cx.create_object();

                for (name, value) in fields.iter() {
                    let value = value.to_js_value(&mut *cx)?;
                    cx.assing_field_to_object(&mut object, name.clone(), value)?;
                }

                Ok(object.upcast())
            }
            IRComponent::String(value) => Ok(cx.create_string(value.clone()).upcast()),
            IRComponent::Number(number) => Ok(cx.create_number(*number).upcast()),
            IRComponent::Bool(value) => Ok(cx.create_bool(*value).upcast()),
            IRComponent::Array(array) => {
                let mut js_array = cx.create_array(array.len() as u32);

                let array = array
                    .iter()
                    .map(|entry| entry.to_js_value(&mut *cx))
                    .collect();

                cx.assing_entries_to_array(&mut js_array, array)?;

                Ok(js_array.upcast())
            }
        }
    }

    pub fn new_object_from_vec(fields: Vec<(&str, Box<IRComponent>)>) -> IRComponent {
        let mut object_fields = HashMap::<String, Box<IRComponent>>::new();

        for (name, value) in fields {
            object_fields.insert(name.to_string(), value);
        }

        IRComponent::new_object_from_map(object_fields)
    }

    pub fn new_object_from_map(fields: HashMap<String, Box<IRComponent>>) -> IRComponent {
        IRComponent::Object { fields }
    }

    pub fn new_string(value: String) -> IRComponent {
        IRComponent::String(value)
    }

    pub fn new_number(value: f64) -> IRComponent {
        IRComponent::Number(value)
    }

    pub fn new_bool(value: bool) -> IRComponent {
        IRComponent::Bool(value)
    }

    pub fn new_array(value: Vec<Box<IRComponent>>) -> IRComponent {
        IRComponent::Array(value)
    }

    pub fn mathes(&self, other: &Self) -> bool {
        match self {
            IRComponent::Object { fields } => {
                let self_fields = fields;

                if let IRComponent::Object { fields } = other {
                    self.compare_maps_keys(&self_fields, &fields)
                } else {
                    false
                }
            }
            _ => self == other,
        }
    }

    fn compare_maps_keys(
        &self,
        map1: &HashMap<String, Box<IRComponent>>,
        map2: &HashMap<String, Box<IRComponent>>,
    ) -> bool {
        let mut first_keys: Vec<_> = map1.keys().collect();
        let mut second_keys: Vec<_> = map2.keys().collect();

        first_keys.sort();
        second_keys.sort();

        first_keys == second_keys
    }
}

#[cfg(test)]
mod tests {
    use super::IRComponent;

    #[test]
    fn it_comparable() {
        let fields_1 = vec![
            (
                "name",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
            (
                "name2",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
            (
                "name3",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
        ];
        let component_1 = IRComponent::new_object_from_vec(fields_1);

        let fields_2 = vec![
            (
                "name",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
            (
                "name2",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
            (
                "name3",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
        ];
        let component_2 = IRComponent::new_object_from_vec(fields_2);

        assert_eq!(component_1, component_2);
    }

    #[test]
    fn it_matchable() {
        let fields_1 = vec![
            (
                "name",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
            (
                "name2",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
            (
                "name3",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
        ];
        let component_1 = IRComponent::new_object_from_vec(fields_1);

        let fields_2 = vec![
            (
                "name",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
            (
                "name2",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
            (
                "name3",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
        ];
        let component_2 = IRComponent::new_object_from_vec(fields_2);

        let fields_3 = vec![
            (
                "name",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
            (
                "name3",
                Box::new(IRComponent::new_string("test".to_string())),
            ),
        ];
        let component_3 = IRComponent::new_object_from_vec(fields_3);

        assert!(component_1.mathes(&component_2));
        assert!(!component_2.mathes(&component_3));
    }
}
