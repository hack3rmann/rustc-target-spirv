#![feature(rustc_private)]
#![allow(unsafe_code)]

extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use rustc_codegen_ssa::traits::CodegenBackend;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct SpirvCodegenBackend;

impl CodegenBackend for SpirvCodegenBackend {
    fn init(&self, _sess: &rustc_session::Session) {
        todo!()
    }

    fn link(
        &self,
        _sess: &rustc_session::Session,
        _codegen_results: rustc_codegen_ssa::CodegenResults,
        _outputs: &rustc_session::config::OutputFilenames,
    ) -> Result<(), rustc_span::ErrorGuaranteed> {
        todo!()
    }

    fn print(
        &self,
        _req: &rustc_session::config::PrintRequest,
        _out: &mut String,
        _sess: &rustc_session::Session,
    ) {
        todo!()
    }

    fn provide(&self, _providers: &mut rustc_middle::util::Providers) {
        todo!()
    }

    fn print_passes(&self) {
        todo!()
    }

    fn join_codegen(
        &self,
        _ongoing_codegen: Box<dyn std::any::Any>,
        _sess: &rustc_session::Session,
        _outputs: &rustc_session::config::OutputFilenames,
    ) -> (
        rustc_codegen_ssa::CodegenResults,
        rustc_data_structures::fx::FxIndexMap<
            rustc_middle::dep_graph::WorkProductId,
            rustc_middle::dep_graph::WorkProduct,
        >,
    ) {
        todo!()
    }

    fn print_version(&self) {
        todo!()
    }

    fn codegen_crate(
        &self,
        _tcx: rustc_middle::ty::TyCtxt<'_>,
        _metadata: rustc_metadata::EncodedMetadata,
        _need_metadata_module: bool,
    ) -> Box<dyn std::any::Any> {
        todo!()
    }

    fn locale_resource(&self) -> &'static str {
        todo!()
    }

    fn target_features(
        &self,
        _sess: &rustc_session::Session,
        _allow_unstable: bool,
    ) -> Vec<rustc_span::Symbol> {
        todo!()
    }

    fn metadata_loader(&self) -> Box<rustc_metadata::creader::MetadataLoaderDyn> {
        todo!()
    }

    fn supports_parallel(&self) -> bool {
        todo!()
    }
}

#[no_mangle]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(SpirvCodegenBackend)
}
