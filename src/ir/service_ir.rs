use super::ir_component::IRComponent;
use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub struct ServiceIR {
    name: String,
    entities: Vec<Box<dyn IRComponent>>,
    actions: Vec<Box<dyn IRComponent>>,
    functions: Vec<Box<dyn IRComponent>>,
}

impl IRComponent for ServiceIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        service_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_name(&mut *cx, service_object)?;
        self.assign_entities(&mut *cx, service_object)?;
        self.assign_actions(&mut *cx, service_object)?;
        self.assign_functions(&mut *cx, service_object)?;
        Ok(())
    }
}

impl ServiceIR {
    fn assign_name<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        service_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.create_string(self.name.clone());
        cx.assing_field_to_object(service_object, "name", name)?;
        Ok(())
    }

    fn assign_entities<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        service_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let entities: Vec<_> = self
            .entities
            .iter()
            .map(|entity| entity.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(service_object, "entities", entities)?;

        Ok(())
    }

    fn assign_actions<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        service_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let actions: Vec<_> = self
            .actions
            .iter()
            .map(|action| action.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(service_object, "actions", actions)?;

        Ok(())
    }

    fn assign_functions<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        service_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let functions: Vec<_> = self
            .functions
            .iter()
            .map(|function| function.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(service_object, "functions", functions)?;

        Ok(())
    }

    pub fn new(
        name: String,
        entities: Vec<Box<dyn IRComponent>>,
        actions: Vec<Box<dyn IRComponent>>,
        functions: Vec<Box<dyn IRComponent>>,
    ) -> ServiceIR {
        ServiceIR {
            name,
            entities,
            actions,
            functions,
        }
    }
}
