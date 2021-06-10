use super::ir_component::IRComponent;
use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub struct ModuleIR {
    services: Vec<Box<dyn IRComponent>>,
    entities: Vec<Box<dyn IRComponent>>,
    types: Vec<Box<dyn IRComponent>>,
}

impl IRComponent for ModuleIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        module_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_services(&mut *cx, module_object)?;
        self.assign_entities(&mut *cx, module_object)?;
        self.assign_types(&mut *cx, module_object)?;
        Ok(())
    }
}

impl ModuleIR {
    fn assign_services<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        module_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let services: Vec<_> = self
            .services
            .iter()
            .map(|service| service.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(module_object, "services", services)?;

        Ok(())
    }

    fn assign_entities<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        module_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let entities: Vec<_> = self
            .entities
            .iter()
            .map(|entity| entity.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(module_object, "entities", entities)?;

        Ok(())
    }

    fn assign_types<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        module_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let types: Vec<_> = self
            .types
            .iter()
            .map(|ir_type| ir_type.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(module_object, "types", types)?;

        Ok(())
    }

    pub fn new(
        services: Vec<Box<dyn IRComponent>>,
        entities: Vec<Box<dyn IRComponent>>,
        types: Vec<Box<dyn IRComponent>>,
    ) -> ModuleIR {
        ModuleIR {
            services,
            entities,
            types,
        }
    }
}
