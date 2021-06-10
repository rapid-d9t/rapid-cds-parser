use super::default_value_ir::DefaultValueIR;
use super::ir_component::IRComponent;
use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub struct FieldIR {
    name: String,
    field_type: String,
    default_value: Option<DefaultValueIR>,
}

impl IRComponent for FieldIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        field_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_name(&mut *cx, field_object)?;
        self.assign_field_type(&mut *cx, field_object)?;
        self.assign_default_value(&mut *cx, field_object)?;
        Ok(())
    }
}

impl FieldIR {
    fn assign_name<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        field_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.create_string(self.name.clone());
        cx.assing_field_to_object(field_object, "name", name)?;
        Ok(())
    }

    fn assign_field_type<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        field_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let field_type = cx.create_string(self.field_type.clone());
        cx.assing_field_to_object(field_object, "type", field_type)?;
        Ok(())
    }

    fn assign_default_value<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        field_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        match &self.default_value {
            Some(value) => {
                let has_default_value = cx.create_bool(true);
                cx.assing_field_to_object(field_object, "hasDefault", has_default_value)?;

                value.assign_to_object(&mut *cx, field_object)?;
            }
            None => {
                let has_default_value = cx.create_bool(false);
                cx.assing_field_to_object(field_object, "hasDefault", has_default_value)?;
            }
        }

        Ok(())
    }

    pub fn new(name: String, field_type: String, default_value: Option<DefaultValueIR>) -> FieldIR {
        FieldIR {
            name,
            field_type,
            default_value,
        }
    }
}
