use super::ir_component::IRComponent;
use super::ir_error::IRError;
use neon::prelude::*;

pub struct FunctionIR {
    name: String,
    arguments: Vec<Box<dyn IRComponent>>,
    output: Box<dyn IRComponent>,
}

impl IRComponent for FunctionIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
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
        cx: &mut ComputeContext<'internal, 'outer>,
        function_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.string(self.name.clone());
        function_object.set(&mut *cx, "name", name)?;
        Ok(())
    }

    fn assign_arguments<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        function_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let arguments = JsArray::new(&mut *cx, self.arguments.len() as u32);
        function_object.set(&mut *cx, "arguments", arguments)?;

        for (index, argument) in (&self.arguments).iter().enumerate() {
            let argument = argument.to_js_object(&mut *cx)?;
            arguments.set(&mut *cx, index as u32, argument)?;
        }

        Ok(())
    }

    fn assign_output<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        function_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let output = self.output.to_js_object(&mut *cx)?;
        function_object.set(&mut *cx, "output", output)?;
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
