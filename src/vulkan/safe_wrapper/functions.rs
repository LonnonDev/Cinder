use std::mem::zeroed;
use std::ptr;

use mira::vulkan::{VkInstanceCreateInfo, VkAllocationCallbacks, VkInstance};

use crate::result::CinderResult;

use crate::vulkan::r#unsafe::unsafe_functions::vkCreateInstance;

pub(crate) fn create_instance(
    create_info: Option<VkInstanceCreateInfo>, 
    allocator: Option<VkAllocationCallbacks>, 
) -> CinderResult<VkInstance> {
    unsafe {
        let instance = zeroed();
        let result = vkCreateInstance(
            match create_info {
                Some(create_info) => &create_info as *const _,
                None => ptr::null(),
            },
            match allocator {
                Some(allocator) => &allocator as *const _,
                None => ptr::null(),
            },
            instance,
        );
        CinderResult::new(result, instance)
    }
}