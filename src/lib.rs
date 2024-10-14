#![feature(rustc_private)]
#![allow(unsafe_code)]

pub mod codegen;

extern crate rustc_ast;
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use codegen::spirv_types::Word;
use rustc_ast::expand::allocator::AllocatorKind;
use rustc_codegen_ssa::{
    back::{
        lto::{LtoModuleCodegen, SerializedModule, ThinModule},
        write::{
            CodegenContext, FatLtoInput, ModuleConfig, OngoingCodegen, TargetMachineFactoryFn,
        },
    },
    traits::{
        CodegenBackend, ExtraBackendMethods, ModuleBufferMethods, ThinBufferMethods,
        WriteBackendMethods,
    },
    CodegenResults, CompiledModule, ModuleCodegen, ModuleKind,
};
use rustc_data_structures::fx::FxIndexMap;
use rustc_errors::{DiagCtxtHandle, FatalError};
use rustc_metadata::EncodedMetadata;
use rustc_middle::{
    dep_graph::{WorkProduct, WorkProductId},
    ty::TyCtxt,
    util::Providers,
};
use rustc_session::{
    config::{OptLevel, OutputFilenames, OutputType, PrintRequest},
    Session,
};
use rustc_span::{ErrorGuaranteed, Symbol};
use std::{
    any::Any,
    fs::File,
    io::{self, Write},
    sync::Arc,
    thread,
};

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
        // TODO:
    }

    fn link(
        &self,
        _sess: &Session,
        _codegen_results: CodegenResults,
        _outputs: &OutputFilenames,
    ) -> Result<(), ErrorGuaranteed> {
        todo!()
    }

    fn provide(&self, providers: &mut Providers) {
        // FIXME: global backend providers
        providers.global_backend_features = |_tcx, ()| vec![];
    }

    fn join_codegen(
        &self,
        ongoing_codegen: Box<dyn Any>,
        session: &Session,
        _outputs: &OutputFilenames,
    ) -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>) {
        let (codegen_results, work_products) = ongoing_codegen
            .downcast::<OngoingCodegen<Self>>()
            .expect("Expected OngoingCodegen, found Box<Any>")
            .join(session);

        (codegen_results, work_products)
    }

    fn codegen_crate(
        &self,
        type_context: TyCtxt<'_>,
        metadata: EncodedMetadata,
        need_metadata_module: bool,
    ) -> Box<dyn Any> {
        Box::new(rustc_codegen_ssa::base::codegen_crate(
            Self,
            type_context,
            type_context
                .sess
                .opts
                .cg
                .target_cpu
                .clone()
                .unwrap_or_else(|| type_context.sess.target.cpu.to_string()),
            metadata,
            need_metadata_module,
        ))
    }

    fn locale_resource(&self) -> &'static str {
        rustc_errors::DEFAULT_LOCALE_RESOURCE
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
        eprintln!("TODO: print statistics");
    }

    fn print_pass_timings(&self) {
        eprintln!("TODO: print pass timings");
    }

    unsafe fn codegen(
        codegen_context: &CodegenContext<Self>,
        _dcx: DiagCtxtHandle<'_>,
        module: ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
    ) -> Result<CompiledModule, FatalError> {
        let object_path = codegen_context
            .output_filenames
            .temp_path(OutputType::Object, Some(&module.name));

        let spirv_module = spirv_tools::binary::from_binary(&module.module_llvm);

        File::create(&object_path)
            .unwrap()
            .write_all(spirv_module)
            .unwrap();

        Ok(CompiledModule {
            name: module.name,
            kind: module.kind,
            object: Some(object_path),
            dwarf_object: None,
            bytecode: None,
            assembly: None,
            llvm_ir: None,
        })
    }

    unsafe fn optimize(
        _cgcx: &CodegenContext<Self>,
        _dcx: DiagCtxtHandle<'_>,
        _module: &ModuleCodegen<Self::Module>,
        _config: &ModuleConfig,
    ) -> Result<(), FatalError> {
        // FIXME: optimize
        Ok(())
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
        _codegen_context: &CodegenContext<Self>,
        thin_module: ThinModule<Self>,
    ) -> Result<ModuleCodegen<Self::Module>, FatalError> {
        // FIXME: optimize

        let module = ModuleCodegen {
            module_llvm: spirv_tools::binary::to_binary(thin_module.data())
                .unwrap()
                .to_vec(),
            name: thin_module.name().to_string(),
            kind: ModuleKind::Regular,
        };

        Ok(module)
    }

    fn serialize_module(module: ModuleCodegen<Self::Module>) -> (String, Self::ModuleBuffer) {
        (
            module.name,
            SpirvModuleBuffer {
                data: module.module_llvm,
            },
        )
    }
}

impl ExtraBackendMethods for SpirvCodegenBackend {
    fn codegen_allocator(
        &self,
        _type_context: TyCtxt<'_>,
        _module_name: &str,
        _kind: AllocatorKind,
        _alloc_error_handler_kind: AllocatorKind,
    ) -> Self::Module {
        // TODO: codegen allocator
        vec![]
    }

    fn compile_codegen_unit(
        &self,
        type_context: TyCtxt<'_>,
        codegen_unit_name: Symbol,
    ) -> (ModuleCodegen<Self::Module>, u64) {
        let codegen_unit = type_context.codegen_unit(codegen_unit_name);
        eprintln!("CODEGEN_UNIT_NAME={}", codegen_unit.name());

        todo!("compile rust to spirv")
    }

    fn target_machine_factory(
        &self,
        _sess: &Session,
        _opt_level: OptLevel,
        _target_features: &[String],
    ) -> TargetMachineFactoryFn<Self> {
        Arc::new(|_| Ok(()))
    }
}

#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(SpirvCodegenBackend)
}
