use super::ir_component::IRComponent;
use super::ir_error::IRError;
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
        cx: &mut ComputeContext<'internal, 'outer>,
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
        cx: &mut ComputeContext<'internal, 'outer>,
        service_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.string(self.name.clone());
        service_object.set(&mut *cx, "name", name)?;
        Ok(())
    }

    fn assign_entities<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        service_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let entities = JsArray::new(&mut *cx, self.entities.len() as u32);
        service_object.set(&mut *cx, "entities", entities)?;

        for (index, entity) in (&self.entities).iter().enumerate() {
            let entity = entity.to_js_object(&mut *cx)?;
            entities.set(&mut *cx, index as u32, entity)?;
        }

        Ok(())
    }

    fn assign_actions<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        service_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let actions = JsArray::new(&mut *cx, self.actions.len() as u32);
        service_object.set(&mut *cx, "actions", actions)?;

        for (index, action) in (&self.actions).iter().enumerate() {
            let action = action.to_js_object(&mut *cx)?;
            actions.set(&mut *cx, index as u32, action)?;
        }

        Ok(())
    }

    fn assign_functions<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        service_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let functions = JsArray::new(&mut *cx, self.functions.len() as u32);
        service_object.set(&mut *cx, "functions", functions)?;

        for (index, function) in (&self.functions).iter().enumerate() {
            let function = function.to_js_object(&mut *cx)?;
            functions.set(&mut *cx, index as u32, function)?;
        }

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
