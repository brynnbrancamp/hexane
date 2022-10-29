#![feature(default_free_fn)]
#![feature(exit_status_error)]
#![feature(trait_alias)]
#![feature(let_else)]
#![feature(box_syntax)]
#![feature(unboxed_closures)]
#![feature(fn_traits)]

mod buffer;
mod commands;
mod context;
mod device;
mod format;
mod image;
mod pipeline;
mod semaphore;
mod swapchain;
mod task;

use std::error;
use std::fmt;
use std::result;

pub mod prelude {
    pub use crate::buffer::{Buffer, BufferInfo, Memory};
    pub use crate::commands::{
        Attachment, BufferCopy, BufferWrite, Clear, Commands, DrawIndexed, LoadOp, RenderArea,
        RenderPass,
    };
    pub use crate::context::{Context, ContextInfo};
    pub use crate::device::{Device, DeviceInfo, DeviceSelector};
    pub use crate::format::Format;
    pub use crate::image::{Image, ImageUsage};
    pub use crate::pipeline::{
        Color, ComputePipelineInfo, GraphicsPipelineInfo, Pipeline, PipelineCompiler,
        PipelineCompilerInfo, Shader, ShaderCompiler, ShaderType,
    };
    pub use crate::semaphore::{BinarySemaphoreInfo, Semaphore, TimelineSemaphoreInfo};
    pub use crate::swapchain::{PresentMode, SurfaceFormatSelector, Swapchain, SwapchainInfo};
    pub use crate::task::{non_optimizer, BufferAccess, Executor, ImageAccess, Resource, Task};
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
