// FIXME:
#![allow(unused)]

use std::num::NonZeroU32;

pub type Word = u32;
pub type Id = Word;

/// SPIR-V backend data types.
///
/// ### See
///   - [Types](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#_types)
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum SpirvType<'a> {
    Void,
    Bool,
    Integer {
        width: u32,
        is_signed: bool,
    },
    Float {
        width: u32,
    },
    /// # Note
    ///
    /// `n_components` >= 2
    Vector {
        component_type: Id,
        n_components: u32,
    },
    /// # Note
    ///
    /// `n_columns` >= 2
    Matrix {
        column_type: Id,
        n_columns: u32,
    },
    Image {
        sampled_type: Id,
        dimension: ImageDimension,
        depth: ImageDepth,
        is_arrayed: bool,
        is_multisampled: bool,
        format: ImageFormat,
        access_qualifier: Option<AccessQualifier>,
    },
    Sampler,
    SampledImage {
        image_type: Id,
    },
    Array {
        element_type: Id,
        length: NonZeroU32,
    },
    RuntimeArray {
        element_type: Id,
    },
    Struct {
        member_types: &'a [Id],
    },
    Opaque {
        name: &'a [Word],
    },
    Pointer {
        pointee_type: Id,
        storage_class: StorageClass,
    },
    Function {
        return_type: Id,
    },
}

/// Is whether or not this image is a depth image.
///
/// # Note
///
/// Note that whether or not depth comparisons are actually done is a
/// property of the sampling opcode, not of this type declaration.
///
/// ### See
///  - [SPIR-V Depth](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#OpTypeImage)
#[derive(Clone, Debug, PartialEq, Default, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum ImageDepth {
    /// Indicates not a depth image
    #[default]
    NotDepth = 0,
    /// Indicates a depth image
    Depth = 1,
    /// Means no indication as to whether this is a depth or non-depth image
    Unspecified = 2,
}

/// Indicates whether or not this image is accessed in combination with a sampler.
///
/// ### See
///   - [Sampled](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#OpTypeImage)
#[derive(Clone, Debug, PartialEq, Default, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum Sampled {
    /// Indicates this is only known at run time, not at compile time
    #[default]
    Runtime = 0,
    /// Indicates an image compatible with sampling operations
    SampleCompatible = 1,
    /// Indicates an image compatible with read/write operations (a storage or subpass data image)
    ReadWriteCompatible = 2,
}

/// Dimensionality of an image.
///
/// Some uses require capabilities beyond
/// the enabling capabilities, for example where the type’s Sampled
/// operand is 2, or Arrayed operand is 1. See the capabilities section for more detail.
///
/// ### See
///   - [Dim](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#Dim)
#[derive(Clone, Debug, PartialEq, Default, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum ImageDimension {
    /// SPIR-V Dim 1D (enabled with capabilities: Sampled1D)
    Image1d,
    /// SPIR-V Dim 2D
    #[default]
    Image2d,
    /// SPIR-V Dim 3D
    Image3d,
    /// SPIR-V Dim Cube (enabled with capabilities: Shader)
    Cube,
    /// SPIR-V Dim Rect (enabled with capabilities: SampledRect)
    Rect,
    /// SPIR-V Dim Buffer (enabled with capabilities: SampledBuffer)
    Buffer,
    /// SPIR-V Dim SubpassData (enabled with capabilities: InputAttachment)
    SubpassData,
    /// SPIR-V Dim TileImageDataEXT (enabled with capabilities: TileImageColorReadAccessEXT)
    ///
    /// # Note
    ///
    /// Reserved. (from SPIR-V spec)
    TileImageDataExt,
}

/// Declarative image format.
/// See [Image Format](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#Image_Format)
#[derive(Clone, Debug, PartialEq, Default, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum ImageFormat {
    #[default]
    Unknown = 0,
    Rgba32f = 1,
    Rgba16f = 2,
    R32f = 3,
    Rgba8 = 4,
    Rgba8Snorm = 5,
    Rg32f = 6,
    Rg16f = 7,
    R11fG11fB10f = 8,
    R16f = 9,
    Rgba16 = 10,
    Rgb10A2 = 11,
    Rg16 = 12,
    Rg8 = 13,
    R16 = 14,
    R8 = 15,
    Rgba16Snorm = 16,
    Rg16Snorm = 17,
    Rg8Snorm = 18,
    R16Snorm = 19,
    R8Snorm = 20,
    Rgba32i = 21,
    Rgba16i = 22,
    Rgba8i = 23,
    R32i = 24,
    Rg32i = 25,
    Rg16i = 26,
    Rg8i = 27,
    R16i = 28,
    R8i = 29,
    Rgba32ui = 30,
    Rgba16ui = 31,
    Rgba8ui = 32,
    R32ui = 33,
    Rgb10a2ui = 34,
    Rg32ui = 35,
    Rg16ui = 36,
    Rg8ui = 37,
    R16ui = 38,
    R8ui = 39,
    R64ui = 40,
    R64i = 41,
}

/// Defines the access permissions.
///
/// ### See
///   - [Access Qualifier](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#Access_Qualifier)
#[derive(Clone, Debug, PartialEq, Default, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum AccessQualifier {
    #[default]
    ReadOnly = 0,
    WriteOnly = 1,
    ReadWrite = 2,
}

/// Class of storage for declared variables.
///
/// Intermediate values do not form a storage class, and unless stated otherwise,
/// storage class-based restrictions are not restrictions on intermediate objects and their types.
///
/// ### See
///  - [Storage Class](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#Storage_Class)
///  - [Intermediate values](https://registry.khronos.org/SPIR-V/specs/unified1/SPIRV.html#Intermediate)
#[derive(Clone, Debug, PartialEq, Default, Copy, Eq, PartialOrd, Ord, Hash)]
pub enum StorageClass {
    #[default]
    UniformConstant = 0,
    Input = 1,
    Uniform = 2,
    Output = 3,
    Workgroup = 4,
    CrossWorkgroup = 5,
    Private = 6,
    Function = 7,
    Generic = 8,
    PushConstant = 9,
    AtomicCounter = 10,
    Image = 11,
    StorageBuffer = 12,
}
