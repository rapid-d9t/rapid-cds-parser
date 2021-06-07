use super::ir_component::IRComponent;
use super::ir_error::IRError;
use neon::prelude::*;

pub struct ModuleIR {
    services: Vec<Box<dyn IRComponent>>,
    entities: Vec<Box<dyn IRComponent>>,
    types: Vec<Box<dyn IRComponent>>,
}

impl IRComponent for ModuleIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
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
        cx: &mut ComputeContext<'internal, 'outer>,
        module_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let services = JsArray::new(&mut *cx, self.services.len() as u32);
        module_object.set(&mut *cx, "services", services)?;

        for (index, service) in (&self.services).iter().enumerate() {
            let service = service.to_js_object(&mut *cx)?;
            services.set(&mut *cx, index as u32, service)?;
        }

        Ok(())
    }

    fn assign_entities<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        module_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let entities = JsArray::new(&mut *cx, self.entities.len() as u32);
        module_object.set(&mut *cx, "entities", entities)?;

        for (index, entity) in (&self.entities).iter().enumerate() {
            let entity = entity.to_js_object(&mut *cx)?;
            entities.set(&mut *cx, index as u32, entity)?;
        }

        Ok(())
    }

    fn assign_types<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        module_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let types = JsArray::new(&mut *cx, self.types.len() as u32);
        module_object.set(&mut *cx, "types", types)?;

        for (index, ir_type) in (&self.types).iter().enumerate() {
            let type_object = ir_type.to_js_object(&mut *cx)?;
            types.set(&mut *cx, index as u32, type_object)?;
        }

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
