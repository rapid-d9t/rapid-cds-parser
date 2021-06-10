use super::ir_component::IRComponent;
use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub struct AspectIR {
    name: String,
    fields: Vec<Box<dyn IRComponent>>,
}

impl IRComponent for AspectIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        aspect_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_name(&mut *cx, aspect_object)?;
        self.assign_fields(&mut *cx, aspect_object)?;
        Ok(())
    }
}

impl AspectIR {
    fn assign_name<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        aspect_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.create_string(self.name.clone());
        cx.assing_field_to_object(aspect_object, "name", name)?;
        Ok(())
    }

    fn assign_fields<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        aspect_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let fields: Vec<_> = self
            .fields
            .iter()
            .map(|field| field.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(aspect_object, "fields", fields)?;

        Ok(())
    }

    pub fn new(name: String, fields: Vec<Box<dyn IRComponent>>) -> AspectIR {
        AspectIR { name, fields }
    }
}
