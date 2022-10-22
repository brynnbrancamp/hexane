#![feature(default_free_fn)]
#![feature(exit_status_error)]
#![feature(trait_alias)]
#![feature(let_else)]
#![feature(box_syntax)]

mod context;
mod device;
mod format;
mod image;
mod task;
mod graph;
mod pipeline;
mod swapchain;
mod buffer;

use std::error;
use std::fmt;
use std::result;

pub mod prelude {
    pub use crate::context::{Context, ContextInfo};
    pub use crate::device::{Device, DeviceInfo, DeviceSelector};
    pub use crate::format::Format;
    pub use crate::image::ImageUsage;
    pub use crate::pipeline::{
        Attachment, ComputePipelineInfo, GraphicsPipelineInfo, Pipeline, PipelineCompiler,
        PipelineCompilerInfo, Shader, ShaderType, ShaderCompiler,
    };
    pub use crate::swapchain::{PresentMode, SurfaceFormatSelector, Swapchain, SwapchainInfo};
    pub use crate::buffer::{Buffer};
    pub use crate::graph::{Graph, Node, Executor};
    pub use crate::task::{Task, Access, Usage};
    pub use crate::{Error, Result};
}

#[derive(Debug)]
pub enum Error {
    Creation,
    ShaderCompilerNotFound,
    ShaderCompilationError { message: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;
