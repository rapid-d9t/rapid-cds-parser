use super::ir_component::IRComponent;
use super::ir_error::IRError;
use neon::prelude::*;

pub struct TypeIR {
    name: String,
    resolved_type: String,
}

impl IRComponent for TypeIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
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
        cx: &mut ComputeContext<'internal, 'outer>,
        type_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.string(self.name.clone());
        type_object.set(&mut *cx, "name", name)?;
        Ok(())
    }

    fn assign_resolved_type<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        type_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let resolved_type = cx.string(self.resolved_type.clone());
        type_object.set(&mut *cx, "resolvesTo", resolved_type)?;
        Ok(())
    }

    pub fn new(name: String, resolved_type: String) -> TypeIR {
        TypeIR {
            name,
            resolved_type,
        }
    }
}
