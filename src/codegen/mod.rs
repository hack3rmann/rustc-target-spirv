use rustc_middle::{mir::mono::CodegenUnit, ty::TyCtxt};

#[derive(Clone, Copy)]
pub struct CodegenContext<'s> {
    pub type_context: TyCtxt<'s>,
    pub codegen_unit: CodegenUnit<'s>,
}
