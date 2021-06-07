use super::ir_component::IRComponent;
use super::ir_error::IRError;
use neon::prelude::*;

pub struct ActionIR {
    name: String,
    arguments: Vec<Box<dyn IRComponent>>,
    output: Option<Box<dyn IRComponent>>,
}

impl IRComponent for ActionIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
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
        cx: &mut ComputeContext<'internal, 'outer>,
        action_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let name = cx.string(self.name.clone());
        action_object.set(&mut *cx, "name", name)?;
        Ok(())
    }

    fn assign_arguments<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        action_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let arguments = JsArray::new(&mut *cx, self.arguments.len() as u32);
        action_object.set(&mut *cx, "arguments", arguments)?;

        for (index, argument) in (&self.arguments).iter().enumerate() {
            let argument = argument.to_js_object(&mut *cx)?;
            arguments.set(&mut *cx, index as u32, argument)?;
        }

        Ok(())
    }

    fn assign_output<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        action_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        match &self.output {
            Some(output) => {
                let has_output = cx.boolean(true);
                action_object.set(&mut *cx, "hasOutput", has_output)?;

                let output =
                    cx.compute_scoped(|mut cx| Ok(output.to_js_object(&mut cx).unwrap()))?;
                action_object.set(&mut *cx, "output", output)?;
            }
            None => {
                let has_output = cx.boolean(false);
                action_object.set(&mut *cx, "hasOutput", has_output)?;
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
