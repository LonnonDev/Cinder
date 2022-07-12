//! Safe Wrapper for Vulkan Create Info
//!
//! ```rust
//! use cinder::vk_instancing::{SafeApplicationInfo, SafeCreateInfo};
//!
//! let application_info = SafeApplicationInfo::new_strings("Application Name", "Engine Name", "1.0.0", "1.0.0", "0.1.0.0");
//! let create_info = SafeCreateInfo::new_from(&application_info);
//! ```

use tracing::debug;

use crate::vk_instancing::vk_application_info::r#final::ApplicationInfo;

use super::builder::CreateInfoBuilder;

/// A Safe Wrapper for Create Info
#[derive(Debug, Clone)]
pub struct SafeCreateInfo<'a> {
    pub application_info: &'a ApplicationInfo,
    pub enabled_layer_count: u32,
    pub layer_names: Vec<String>,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: Vec<String>,
}

impl<'a> SafeCreateInfo<'a> {
    /// Creates a new Safe Create Info
    ///
    /// ```rust
    /// use cinder::vk_instancing::{SafeCreateInfo, SafeApplicationInfo};
    ///
    /// let application_info = SafeApplicationInfo::new_strings("Application Name", "Engine Name", "1.0.0", "1.0.0", "0.1.0.0");
    /// let create_info = SafeCreateInfo::auto_new(&application_info);
    /// ```
    pub fn auto_new(application_info: &'a ApplicationInfo) -> Self {
        CreateInfoBuilder::new()
            .with_application_info(application_info)
            .build()
    }
    // TODO: Finish the Doc comment for this function
    /// Creates a new Safe Create Info
    ///
    /// ```rust
    /// use cinder::vk_instancing::{SafeCreateInfo, SafeApplicationInfo};
    ///
    /// let application_info = SafeApplicationInfo::new_strings("Application Name", "Engine Name", "1.0.0", "1.0.0", "0.1.0.0");
    /// let create_info = SafeCreateInfo::new_from(&application_info);
    /// ```
    pub fn new_from(
        application_info: &'a ApplicationInfo,
        enabled_layer_count: u32,
        layer_names: Vec<String>,
        enabled_extension_count: u32,
        enabled_extension_names: Vec<String>,
    ) -> Self {
        #[cfg(feature = "detailed-logging")]
        debug!(
            "Creating `SafeCreateInfo` using `new_from` function with arguments `{:#?}`, `{}`, `{:?}`, `{}`, `{:?}`",
            application_info,
            enabled_layer_count,
            layer_names,
            enabled_extension_count,
            enabled_extension_names
        );
        #[cfg(feature = "logging")]
        debug!("Creating `SafeCreateInfo` using `new_from` function");
        Self {
            application_info,
            enabled_layer_count,
            layer_names,
            enabled_extension_count,
            enabled_extension_names,
        }
    }
}
