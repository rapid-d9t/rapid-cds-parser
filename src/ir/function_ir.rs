use super::ir_component::IRComponent;
use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub struct FunctionIR {
    name: String,
    arguments: Vec<Box<dyn IRComponent>>,
    output: Box<dyn IRComponent>,
}

impl IRComponent for FunctionIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        function_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_name(&mut *cx, function_object)?;
        self.assign_arguments(&mut *cx, function_object)?;
        self.assign_output(&mut *cx, function_object)?;
        Ok(())
    }
}

impl FunctionIR {
    fn assign_name<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        function_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.create_string(self.name.clone());
        cx.assing_field_to_object(function_object, "name", name)?;
        Ok(())
    }

    fn assign_arguments<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        function_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let arguments: Vec<_> = self
            .arguments
            .iter()
            .map(|argument| argument.to_js_object(&mut *cx))
            .collect();

        cx.assing_failable_array_field_to_object(function_object, "arguments", arguments)?;

        Ok(())
    }

    fn assign_output<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        function_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let output = self.output.to_js_object(&mut *cx)?;
        cx.assing_field_to_object(function_object, "output", output)?;
        Ok(())
    }

    pub fn new(
        name: String,
        arguments: Vec<Box<dyn IRComponent>>,
        output: Box<dyn IRComponent>,
    ) -> FunctionIR {
        FunctionIR {
            name,
            arguments,
            output,
        }
    }
}
