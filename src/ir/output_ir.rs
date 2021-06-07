use super::ir_component::IRComponent;
use super::ir_error::IRError;
use neon::prelude::*;

pub struct OutputIR {
    output_type: String,
    is_arrayed: bool,
}

impl IRComponent for OutputIR {
    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        output_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        self.assign_output_type(&mut *cx, output_object)?;
        self.assign_is_arrayed(&mut *cx, output_object)?;
        Ok(())
    }
}

impl OutputIR {
    fn assign_output_type<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        output_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let output_type = cx.string(self.output_type.clone());
        output_object.set(&mut *cx, "type", output_type)?;
        Ok(())
    }

    fn assign_is_arrayed<'internal, 'outer>(
        &self,
        cx: &mut ComputeContext<'internal, 'outer>,
        output_object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError> {
        let is_arrayed = cx.boolean(self.is_arrayed);
        output_object.set(&mut *cx, "isArrayed", is_arrayed)?;
        Ok(())
    }

    pub fn new(output_type: String, is_arrayed: bool) -> OutputIR {
        OutputIR {
            output_type,
            is_arrayed,
        }
    }
}
