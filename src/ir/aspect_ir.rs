use super::ir_component::IRComponent;
use super::ir_error::IRError;
use neon::prelude::*;

pub struct AspectIR {
    name: String,
    fields: Vec<Box<dyn IRComponent>>,
}

impl IRComponent for AspectIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
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
        cx: &mut ComputeContext<'internal, 'outer>,
        aspect_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.string(self.name.clone());
        aspect_object.set(&mut *cx, "name", name)?;
        Ok(())
    }

    fn assign_fields<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        aspect_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let fields = JsArray::new(&mut *cx, self.fields.len() as u32);
        aspect_object.set(&mut *cx, "fields", fields)?;

        for (index, field) in (&self.fields).iter().enumerate() {
            let field = field.to_js_object(&mut *cx)?;
            fields.set(&mut *cx, index as u32, field)?;
        }

        Ok(())
    }

    pub fn new(name: String, fields: Vec<Box<dyn IRComponent>>) -> AspectIR {
        AspectIR { name, fields }
    }
}
