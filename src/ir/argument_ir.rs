use super::ir_component::IRComponent;
use super::ir_error::IRError;
use neon::prelude::*;

pub struct ArgumentIR {
    name: String,
    argument_type: String,
    is_arrayed: bool,
}

impl IRComponent for ArgumentIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        argument_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_name(&mut *cx, argument_object)?;
        self.assign_argument_type(&mut *cx, argument_object)?;
        self.assign_is_arrayed(&mut *cx, argument_object)?;
        Ok(())
    }
}

impl ArgumentIR {
    fn assign_name<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        argument_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.string(self.name.clone());
        argument_object.set(&mut *cx, "name", name)?;
        Ok(())
    }

    fn assign_argument_type<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        argument_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let argument_type = cx.string(self.argument_type.clone());
        argument_object.set(&mut *cx, "type", argument_type)?;
        Ok(())
    }

    fn assign_is_arrayed<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        argument_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let is_arrayed = cx.boolean(self.is_arrayed);
        argument_object.set(&mut *cx, "isArrayed", is_arrayed)?;
        Ok(())
    }

    pub fn new(name: String, argument_type: String, is_arrayed: bool) -> ArgumentIR {
        ArgumentIR {
            name,
            argument_type,
            is_arrayed,
        }
    }
}
