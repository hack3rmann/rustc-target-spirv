#![feature(rustc_private)]
#![allow(unsafe_code)]

extern crate rustc_ast;
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use rustc_ast::expand::allocator::AllocatorKind;
use rustc_codegen_ssa::{
    back::{
        lto::{LtoModuleCodegen, SerializedModule, ThinModule},
        write::{CodegenContext, FatLtoInput, ModuleConfig, TargetMachineFactoryFn},
    },
    traits::{
        CodegenBackend, ExtraBackendMethods, ModuleBufferMethods, ThinBufferMethods,
        WriteBackendMethods,
    },
    CodegenResults, CompiledModule, ModuleCodegen,
};
use rustc_data_structures::fx::FxIndexMap;
use rustc_errors::{DiagCtxtHandle, FatalError};
use rustc_metadata::{creader::MetadataLoaderDyn, EncodedMetadata};
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::TyCtxt,
    util::Providers,
};
use rustc_session::{
    config::{OptLevel, OutputFilenames, PrintRequest},
    Session,
};
use rustc_span::{ErrorGuaranteed, Symbol};
use std::{any::Any, io, thread};

pub type Word = u32;

#[derive(Clone, Debug, PartialEq, Default)]
struct SpirvModuleBuffer {
    data: Vec<Word>,
}

impl ModuleBufferMethods for SpirvModuleBuffer {
    fn data(&self) -> &[u8] {
        spirv_tools::binary::from_binary(&self.data)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
struct SpirvThinBuffer {
    data: Vec<Word>,
}

impl ThinBufferMethods for SpirvThinBuffer {
    fn data(&self) -> &[u8] {
        spirv_tools::binary::from_binary(&self.data)
    }

    fn thin_link_data(&self) -> &[u8] {
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct SpirvCodegenBackend;

impl CodegenBackend for SpirvCodegenBackend {
    fn init(&self, _sess: &Session) {
        todo!()
    }

    fn link(
        &self,
        _sess: &Session,
        _codegen_results: CodegenResults,
        _outputs: &OutputFilenames,
    ) -> Result<(), ErrorGuaranteed> {
        todo!()
    }

    fn print(&self, _req: &PrintRequest, _out: &mut String, _sess: &Session) {
        todo!()
    }

    fn provide(&self, _providers: &mut Providers) {
        todo!()
    }

    fn print_passes(&self) {
        todo!()
    }

    fn join_codegen(
        &self,
        _ongoing_codegen: Box<dyn Any>,
        _sess: &Session,
        _outputs: &OutputFilenames,
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>) {
        todo!()
    }

    fn print_version(&self) {
        todo!()
    }

    fn codegen_crate(
        &self,
        _tcx: TyCtxt<'_>,
        _metadata: EncodedMetadata,
        _need_metadata_module: bool,
    ) -> Box<dyn Any> {
        todo!()
    }

    fn locale_resource(&self) -> &'static str {
        todo!()
    }

    fn target_features(&self, _sess: &Session, _allow_unstable: bool) -> Vec<Symbol> {
        todo!()
    }

    fn metadata_loader(&self) -> Box<MetadataLoaderDyn> {
        todo!()
    }

    fn supports_parallel(&self) -> bool {
        todo!()
    }
}

impl WriteBackendMethods for SpirvCodegenBackend {
    type Module = Vec<Word>;
    type ThinData = ();
    type ThinBuffer = SpirvThinBuffer;
    type ModuleBuffer = SpirvModuleBuffer;
    type TargetMachine = ();
    type TargetMachineError = String;

    fn run_thin_lto(
        _cgcx: &CodegenContext<Self>,
        _modules: Vec<(String, Self::ThinBuffer)>,
        _cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
    ) -> Result<(Vec<LtoModuleCodegen<Self>>, Vec<WorkProduct>), FatalError> {
        todo!()
    }

    fn run_link(
        _cgcx: &CodegenContext<Self>,
        _dcx: DiagCtxtHandle<'_>,
        _modules: Vec<ModuleCodegen<Self::Module>>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        todo!()
    }

    fn print_statistics(&self) {
        todo!()
    }

    fn print_pass_timings(&self) {
        todo!()
    }

    unsafe fn codegen(
        _cgcx: &CodegenContext<Self>,
        _dcx: DiagCtxtHandle<'_>,
        _module: ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
    ) -> Result<CompiledModule, FatalError> {
        todo!()
    }

    unsafe fn optimize(
        _cgcx: &CodegenContext<Self>,
        _dcx: DiagCtxtHandle<'_>,
        _module: &ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
    ) -> Result<(), FatalError> {
        todo!()
    }

    fn run_fat_lto(
        _cgcx: &CodegenContext<Self>,
        _modules: Vec<FatLtoInput<Self>>,
        _cached_modules: Vec<(SerializedModule<Self::ModuleBuffer>, WorkProduct)>,
    ) -> Result<LtoModuleCodegen<Self>, FatalError> {
        todo!()
    }

    fn optimize_fat(
        _cgcx: &CodegenContext<Self>,
        _llmod: &mut ModuleCodegen<Self::Module>,
    ) -> Result<(), FatalError> {
        todo!()
    }

    fn prepare_thin(
        module: ModuleCodegen<Self::Module>,
        _want_summary: bool,
    ) -> (String, Self::ThinBuffer) {
        (
            module.name,
            SpirvThinBuffer {
                data: module.module_llvm,
            },
        )
    }

    unsafe fn optimize_thin(
        _cgcx: &CodegenContext<Self>,
        _thin: ThinModule<Self>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        todo!()
    }

    fn serialize_module(_module: ModuleCodegen<Self::Module>) -> (String, Self::ModuleBuffer) {
        todo!()
    }
}

impl ExtraBackendMethods for SpirvCodegenBackend {
    fn codegen_allocator(
        &self,
        _tcx: TyCtxt<'_>,
        _module_name: &str,
        _kind: AllocatorKind,
        _alloc_error_handler_kind: AllocatorKind,
    ) -> Self::Module {
        todo!()
    }

    fn compile_codegen_unit(
        &self,
        _tcx: TyCtxt<'_>,
        _cgu_name: Symbol,
    ) -> (ModuleCodegen<Self::Module>, u64) {
        todo!()
    }

    fn target_machine_factory(
        &self,
        _sess: &Session,
        _opt_level: OptLevel,
        _target_features: &[String],
    ) -> TargetMachineFactoryFn<Self> {
        todo!()
    }

    fn spawn_named_thread<F, T>(
        _time_trace: bool,
        _name: String,
        _f: F,
    ) -> io::Result<thread::JoinHandle<T>>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        todo!()
    }
}

#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(SpirvCodegenBackend)
}
