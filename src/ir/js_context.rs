use super::ir_error::IRError;
use neon::prelude::*;

pub struct JsContext<'internal, 'outer> {
    cx: ComputeContext<'internal, 'outer>,
}

impl<'internal, 'outer> JsContext<'internal, 'outer> {
    pub fn new(cx: ComputeContext<'internal, 'outer>) -> JsContext<'internal, 'outer> {
        JsContext { cx }
    }

    pub fn assing_field_to_object<ValueType: neon::types::Value>(
        &mut self,
        object: &mut neon::handle::Handle<'internal, JsObject>,
        field_name: String,
        field: neon::handle::Handle<ValueType>,
    ) -> NeonResult<bool> {
        object.set(&mut self.cx, field_name.as_str(), field)
    }

    pub fn create_object(&mut self) -> neon::handle::Handle<'internal, JsObject> {
        JsObject::new(&mut self.cx)
    }

    pub fn create_array(&mut self, length: u32) -> neon::handle::Handle<'internal, JsArray> {
        JsArray::new(&mut self.cx, length)
    }

    pub fn assing_entries_to_array(
        &mut self,
        array: &mut neon::handle::Handle<'internal, JsArray>,
        values: Vec<Result<neon::handle::Handle<JsValue>, IRError>>,
    ) -> NeonResult<bool> {
        for (index, value) in (&values).iter().enumerate() {
            match value {
                Ok(value) => {
                    array.set(&mut self.cx, index as u32, *value)?;
                }
                Err(error) => return self.throw_error(format!("{}", error)),
            }
        }

        Ok(true)
    }

    pub fn create_string(&mut self, value: String) -> neon::handle::Handle<'internal, JsString> {
        self.cx.string(value)
    }

    pub fn create_number(&mut self, value: f64) -> neon::handle::Handle<'internal, JsNumber> {
        self.cx.number(value)
    }

    pub fn create_bool(&mut self, value: bool) -> neon::handle::Handle<'internal, JsBoolean> {
        self.cx.boolean(value)
    }

    pub fn throw_error<T>(&mut self, message: String) -> NeonResult<T> {
        self.cx.throw_error(message)
    }
}
