mod error;
mod module;
pub mod sections;
mod util;
pub use error::EncodingError;
pub use module::Module;
pub use util::ExternalKind;
pub use util::ResizableLimits;
pub use util::ValType;
