pub mod spirv_types;

use rustc_ast::{InlineAsmOptions, InlineAsmTemplatePiece};
use rustc_codegen_ssa::traits::{
    AsmBuilderMethods, AsmCodegenMethods, BackendTypes, GlobalAsmOperandRef, InlineAsmOperandRef,
};
use rustc_middle::{
    mir::mono::CodegenUnit,
    ty::{Instance, TyCtxt},
};
use rustc_span::Span;

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

impl BackendTypes for CodegenContext<'_> {
    type Value = ();
    type Metadata = ();
    type Function = ();
    type BasicBlock = ();
    type Type = ();
    type Funclet = ();
    type DIScope = ();
    type DILocation = ();
    type DIVariable = ();
}

impl<'tcx> AsmBuilderMethods<'tcx> for CodegenContext<'tcx> {
    fn codegen_inline_asm(
        &mut self,
        _template: &[InlineAsmTemplatePiece],
        _operands: &[InlineAsmOperandRef<'tcx, Self>],
        _options: InlineAsmOptions,
        _line_spans: &[Span],
        _instance: Instance<'_>,
        _dest: Option<Self::BasicBlock>,
        _catch_funclet: Option<(Self::BasicBlock, Option<&Self::Funclet>)>,
    ) {
        todo!("codegen_inline_asm")
    }
}

impl<'tcx> AsmCodegenMethods<'tcx> for CodegenContext<'tcx> {
    fn codegen_global_asm(
        &self,
        _template: &[InlineAsmTemplatePiece],
        _operands: &[GlobalAsmOperandRef<'tcx>],
        _options: InlineAsmOptions,
        _line_spans: &[Span],
    ) {
        todo!("codegen_global_asm")
    }
}
