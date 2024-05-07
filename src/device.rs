use std::sync::Arc;

use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::instance::Instance;

const REQUIRED_EXTENSIONS: DeviceExtensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::empty()
};

pub fn is_device_suitable(device: &Arc<PhysicalDevice>) -> bool {
    device.supported_extensions().contains(&REQUIRED_EXTENSIONS)
}

fn rate_device(device: &Arc<PhysicalDevice>) -> u32 {
    let feats = device.supported_features();
    if !feats.geometry_shader || !is_device_suitable(device) {
        return 0;
    }

    let props = device.properties();
    let mut score = 0u32;
    if props.device_type == PhysicalDeviceType::DiscreteGpu {
        score += 1000;
    }
    score += props.max_image_dimension2_d;
    println!(
        "Device \x1b[32m{}\x1b[0m scored \x1b[32m{}\x1b[0m",
        props.device_name, score
    );
    score
}

pub fn select_physical(instance: Arc<Instance>) -> Arc<PhysicalDevice> {
    instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .max_by_key(rate_device)
        .expect("no device found")
}

pub fn create_device(physical_device: Arc<PhysicalDevice>) -> (Arc<Device>, Arc<Queue>) {
    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_queue_family_index, queue_family_properties)| {
            queue_family_properties
                .queue_flags
                .contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
        .expect("failed to create device");

    let queue = queues.next().unwrap();
    (device, queue)
}

