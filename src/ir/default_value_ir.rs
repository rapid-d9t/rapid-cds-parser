use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub enum DefaultValueIR {
    Null,
    Integer(i64),
    Float(f64),
    String(String),
}

impl DefaultValueIR {
    pub fn assign_to_object<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        object: &mut Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        match self {
            DefaultValueIR::Integer(value) => {
                let value = cx.create_number(*value as f64);
                cx.assing_field_to_object(object, "default", value)?;
                Ok(())
            }
            DefaultValueIR::Float(value) => {
                let value = cx.create_number(*value);
                cx.assing_field_to_object(object, "default", value)?;
                Ok(())
            }
            DefaultValueIR::String(value) => {
                let value = cx.create_string(value.clone());
                cx.assing_field_to_object(object, "default", value)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
