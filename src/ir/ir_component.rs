use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;
use std::collections::HashMap;

#[derive(std::fmt::Debug)]
pub enum IRComponent {
    Object {
        fields: HashMap<String, Box<IRComponent>>,
    },
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<Box<IRComponent>>),
}

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

    pub fn new_object(fields: HashMap<String, Box<IRComponent>>) -> IRComponent {
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
}
