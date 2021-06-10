use super::ir_component::IRComponent;
use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub struct TypeIR {
    name: String,
    resolved_type: String,
}

impl IRComponent for TypeIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        type_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_name(&mut *cx, type_object)?;
        self.assign_resolved_type(&mut *cx, type_object)?;
        Ok(())
    }
}

impl TypeIR {
    fn assign_name<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        type_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.create_string(self.name.clone());
        cx.assing_field_to_object(type_object, "name", name)?;
        Ok(())
    }

    fn assign_resolved_type<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        type_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let resolved_type = cx.create_string(self.resolved_type.clone());
        cx.assing_field_to_object(type_object, "resolvesTo", resolved_type)?;
        Ok(())
    }

    pub fn new(name: String, resolved_type: String) -> TypeIR {
        TypeIR {
            name,
            resolved_type,
        }
    }
}
