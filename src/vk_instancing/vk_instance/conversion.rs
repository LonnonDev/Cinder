use mira::vulkan::VkInstance;
use tracing::debug;

use crate::{
    functions::create_instance, vk_instancing::Instance, vk_instancing::SafeInstance,
    vk_instancing::UnsafeInstance,
};

impl<'a> Into<UnsafeInstance<'a>> for SafeInstance<'a> {
    fn into(self) -> UnsafeInstance<'a> {
        #[cfg(feature = "detailed-logging")]
        debug!("Converting `SafeInstance` into `UnsafeInstance`");
        UnsafeInstance {
            create_info: self.create_info,
            allocator: std::ptr::null(),
        }
    }
}

impl<'a> Into<VkInstance> for UnsafeInstance<'a> {
    fn into(self) -> VkInstance {
        create_instance(Some(self.create_info), None).unwrap()
    }
}

impl<'a> Into<VkInstance> for Instance<'a> {
    fn into(self) -> VkInstance {
        create_instance(Some(self.normal.create_info), None).unwrap()
    }
}
