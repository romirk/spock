// #![allow(unused)]

use std::sync::Arc;
use std::time::{Duration, Instant};

use vulkano::{sync, VulkanLibrary};
use vulkano::command_buffer::CommandBufferExecFuture;
use vulkano::format::Format;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::Pipeline;
use vulkano::sync::future::{FenceSignalFuture, NowFuture};
use vulkano::sync::GpuFuture;

mod device;
mod pipeline;

fn time_future(future: FenceSignalFuture<CommandBufferExecFuture<NowFuture>>) -> Duration {
    let t = Instant::now();
    future.wait(None).unwrap();
    let d = t.elapsed();
    d
}

fn main() {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    let physical_device = device::select_physical(instance);

    println!(
        "Selected physical device: \x1b[32m{}\x1b[0m",
        physical_device.properties().device_name
    );

    let (device, queue) = device::create_device(physical_device);

    println!(
        "Created logical device with API version \x1b[32m{}\x1b[0m",
        device.api_version()
    );

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
        .unwrap();

    let command_buffer = command_buffer_builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    let d = time_future(future);

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("compute OK ({} us)", d.as_micros());
}

