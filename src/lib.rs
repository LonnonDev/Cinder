pub mod triangle;
pub mod match_error_codes;

pub mod vulkan {
    pub mod safe {
        pub mod application_info;
        pub mod instance;
        pub mod physical_device;
        pub mod instance_create_info;
        pub mod functions;
        pub mod device_queue_create_info;
        pub mod queue_family_indices;
    }
    pub mod r#unsafe {
        pub mod unsafe_functions;
    }
}