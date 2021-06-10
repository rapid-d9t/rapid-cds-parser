use super::ir_component::IRComponent;
use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub struct ArgumentIR {
    name: String,
    argument_type: String,
    is_arrayed: bool,
}

impl IRComponent for ArgumentIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
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
        cx: &mut JsContext<'internal, 'outer>,
        argument_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.create_string(self.name.clone());
        cx.assing_field_to_object(argument_object, "name", name)?;
        Ok(())
    }

    fn assign_argument_type<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        argument_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let argument_type = cx.create_string(self.argument_type.clone());
        cx.assing_field_to_object(argument_object, "type", argument_type)?;
        Ok(())
    }

    fn assign_is_arrayed<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        argument_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let is_arrayed = cx.create_bool(self.is_arrayed);
        cx.assing_field_to_object(argument_object, "type", is_arrayed)?;
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
