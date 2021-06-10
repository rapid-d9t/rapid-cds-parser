use super::ir_component::IRComponent;
use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub struct ActionIR {
    name: String,
    arguments: Vec<Box<dyn IRComponent>>,
    output: Option<Box<dyn IRComponent>>,
}

impl IRComponent for ActionIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        action_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_name(&mut *cx, action_object)?;
        self.assign_arguments(&mut *cx, action_object)?;
        self.assign_output(&mut *cx, action_object)?;
        Ok(())
    }
}

impl ActionIR {
    fn assign_name<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        action_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.create_string(self.name.clone());
        cx.assing_field_to_object(action_object, "name", name)?;
        Ok(())
    }

    fn assign_arguments<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        action_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let arguments: Vec<_> = self
            .arguments
            .iter()
            .map(|argument| argument.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(action_object, "arguments", arguments)?;

        Ok(())
    }

    fn assign_output<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        action_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        match &self.output {
            Some(output) => {
                let has_output = cx.create_bool(true);
                cx.assing_field_to_object(action_object, "hasOutput", has_output)?;

                let output = output.to_js_object(&mut *cx)?;
                cx.assing_field_to_object(action_object, "output", output)?;
            }
            None => {
                let has_output = cx.create_bool(false);
                cx.assing_field_to_object(action_object, "hasOutput", has_output)?;
            }
        }

        Ok(())
    }

    pub fn new(name: String, arguments: Vec<Box<dyn IRComponent>>) -> ActionIR {
        ActionIR {
            name,
            arguments,
            output: None,
        }
    }

    pub fn new_with_output(
        name: String,
        arguments: Vec<Box<dyn IRComponent>>,
        output: Box<dyn IRComponent>,
    ) -> ActionIR {
        ActionIR {
            name,
            arguments,
            output: Some(output),
        }
    }
}
