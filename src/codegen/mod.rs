pub mod spirv_types;

use rustc_middle::{
    mir::mono::CodegenUnit,
    ty::{self, Ty, TyCtxt},
};

pub struct CodegenContext<'tcx> {
    pub type_context: TyCtxt<'tcx>,
    pub codegen_unit: &'tcx CodegenUnit<'tcx>,
}

impl<'tcx> CodegenContext<'tcx> {
    pub const fn new(type_context: TyCtxt<'tcx>, codegen_unit: &'tcx CodegenUnit<'tcx>) -> Self {
        Self {
            type_context,
            codegen_unit,
        }
    }
}
