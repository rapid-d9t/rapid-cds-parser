use super::ir_component::IRComponent;
use super::ir_error::IRError;
use neon::prelude::*;

pub struct EntityIR {
    name: String,
    aspects: Vec<String>,
    fields: Vec<Box<dyn IRComponent>>,
    is_projection_on: Option<String>,
}

impl IRComponent for EntityIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        entity_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_name(&mut *cx, entity_object)?;
        self.assign_is_projection_on(&mut *cx, entity_object)?;
        self.assign_aspects(&mut *cx, entity_object)?;
        self.assign_fields(&mut *cx, entity_object)?;
        Ok(())
    }
}

impl EntityIR {
    fn assign_name<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        entity_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.string(self.name.clone());
        entity_object.set(&mut *cx, "name", name)?;
        Ok(())
    }

    fn assign_is_projection_on<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        entity_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        match &self.is_projection_on {
            Some(name) => {
                let is_projection = cx.boolean(true);
                entity_object.set(&mut *cx, "isProjection", is_projection)?;

                let is_projection_on = cx.string(name);
                entity_object.set(&mut *cx, "isProjectionOn", is_projection_on)?;
            }
            None => {
                let is_projection = cx.boolean(false);
                entity_object.set(&mut *cx, "isProjection", is_projection)?;
            }
        }

        Ok(())
    }

    fn assign_aspects<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        entity_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let aspects = JsArray::new(&mut *cx, self.aspects.len() as u32);
        entity_object.set(&mut *cx, "aspects", aspects)?;

        for (index, aspect_name) in (&self.aspects).iter().enumerate() {
            let aspect_name = cx.string(aspect_name);
            aspects.set(&mut *cx, index as u32, aspect_name)?;
        }

        Ok(())
    }

    fn assign_fields<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        entity_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let fields = JsArray::new(&mut *cx, self.fields.len() as u32);
        entity_object.set(&mut *cx, "fields", fields)?;

        for (index, field) in (&self.fields).iter().enumerate() {
            let field = field.to_js_object(&mut *cx)?;
            fields.set(&mut *cx, index as u32, field)?;
        }

        Ok(())
    }

    pub fn new_concrete(
        name: String,
        aspects: Vec<String>,
        fields: Vec<Box<dyn IRComponent>>,
    ) -> EntityIR {
        EntityIR {
            name,
            aspects,
            fields,
            is_projection_on: None,
        }
    }

    pub fn new_projection(
        name: String,
        fields: Vec<Box<dyn IRComponent>>,
        projection_on: String,
    ) -> EntityIR {
        EntityIR {
            name,
            aspects: Vec::new(),
            fields,
            is_projection_on: Some(projection_on),
        }
    }
}
