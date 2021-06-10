use neon::prelude::*;

pub struct JsContext<'internal, 'outer> {
    cx: ComputeContext<'internal, 'outer>,
}

impl<'internal, 'outer> JsContext<'internal, 'outer> {
    pub fn new(cx: ComputeContext<'internal, 'outer>) -> JsContext<'internal, 'outer> {
        JsContext { cx }
    }

    pub fn assing_failable_array_field_to_object<
        ValueType: neon::types::Value,
        ErrorType: std::convert::From<neon::result::Throw> + Clone,
    >(
        &mut self,
        object: &mut neon::handle::Handle<'internal, JsObject>,
        field_name: &str,
        values: Vec<Result<neon::handle::Handle<ValueType>, ErrorType>>,
    ) -> Result<(), ErrorType> {
        let mut array = self.create_array(values.len() as u32);
        self.assing_field_to_object(object, field_name, array)?;

        for (index, value) in (&values).iter().enumerate() {
            match value {
                Ok(value) => {
                    self.assing_entry_to_array(&mut array, index as u32, *value)?;
                }
                Err(error) => return Err(error.clone()),
            }
        }

        Ok(())
    }

    pub fn assing_array_field_to_object<ValueType: neon::types::Value>(
        &mut self,
        object: &mut neon::handle::Handle<'internal, JsObject>,
        field_name: &str,
        values: Vec<neon::handle::Handle<ValueType>>,
    ) -> NeonResult<()> {
        let mut array = self.create_array(values.len() as u32);
        self.assing_field_to_object(object, field_name, array)?;

        for (index, value) in (&values).iter().enumerate() {
            self.assing_entry_to_array(&mut array, index as u32, *value)?;
        }

        Ok(())
    }

    pub fn assing_field_to_object<ValueType: neon::types::Value>(
        &mut self,
        object: &mut neon::handle::Handle<'internal, JsObject>,
        field_name: &str,
        field: neon::handle::Handle<ValueType>,
    ) -> NeonResult<bool> {
        object.set(&mut self.cx, field_name, field)
    }

    pub fn assing_entry_to_array<ValueType: neon::types::Value>(
        &mut self,
        array: &mut neon::handle::Handle<'internal, JsArray>,
        entry_pos: u32,
        entry: neon::handle::Handle<ValueType>,
    ) -> NeonResult<bool> {
        array.set(&mut self.cx, entry_pos, entry)
    }

    pub fn create_object(&mut self) -> neon::handle::Handle<'internal, JsObject> {
        JsObject::new(&mut self.cx)
    }

    pub fn create_array(&mut self, size: u32) -> neon::handle::Handle<'internal, JsArray> {
        JsArray::new(&mut self.cx, size)
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
