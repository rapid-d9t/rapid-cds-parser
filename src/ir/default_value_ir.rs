use super::ir_error::IRError;
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
        cx: &mut ComputeContext<'internal, 'outer>,
        object: &mut JsObject,
    ) -> Result<(), IRError> {
        match self {
            DefaultValueIR::Integer(value) => {
                let value = cx.number(*value as f64);
                object.set(&mut *cx, "default", value)?;
                Ok(())
            }
            DefaultValueIR::Float(value) => {
                let value = cx.number(*value);
                object.set(&mut *cx, "default", value)?;
                Ok(())
            }
            DefaultValueIR::String(value) => {
                let value = cx.string(value.clone());
                object.set(&mut *cx, "default", value)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
