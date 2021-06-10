use super::ir_component::IRComponent;
use super::ir_error::IRError;
use super::js_context::JsContext;
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
        cx: &mut JsContext<'internal, 'outer>,
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
        cx: &mut JsContext<'internal, 'outer>,
        entity_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.create_string(self.name.clone());
        cx.assing_field_to_object(entity_object, "name", name)?;
        Ok(())
    }

    fn assign_is_projection_on<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        entity_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        match &self.is_projection_on {
            Some(name) => {
                let is_projection = cx.create_bool(true);
                cx.assing_field_to_object(entity_object, "isProjection", is_projection)?;

                let is_projection_on = cx.create_string(name.clone());
                cx.assing_field_to_object(entity_object, "isProjectionOn", is_projection_on)?;
            }
            None => {
                let is_projection = cx.create_bool(false);
                cx.assing_field_to_object(entity_object, "isProjection", is_projection)?;
            }
        }

        Ok(())
    }

    fn assign_aspects<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        entity_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let aspects: Vec<_> = self
            .aspects
            .iter()
            .map(|aspect| cx.create_string(aspect.clone()))
            .collect();

        cx.assing_array_field_to_object(entity_object, "aspects", aspects)?;

        Ok(())
    }

    fn assign_fields<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        entity_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let fields: Vec<_> = self
            .fields
            .iter()
            .map(|field| field.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(entity_object, "fields", fields)?;

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
