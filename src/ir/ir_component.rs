use super::ir_error::IRError;
use super::js_context::JsContext;
use neon::prelude::*;

pub trait IRComponent {
    fn to_js_object<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
    ) -> Result<neon::handle::Handle<'internal, JsObject>, IRError> {
        let mut object = cx.create_object();

        self.assign_object_properties(cx, &mut object)?;

        Ok(object)
    }

    fn assign_object_properties<'internal, 'outer>(
        &self,
        cx: &mut JsContext<'internal, 'outer>,
        object: &mut neon::handle::Handle<'internal, JsObject>,
    ) -> Result<(), IRError>;
}
