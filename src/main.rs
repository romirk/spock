// #![allow(unused)]

use std::sync::Arc;
use std::time::{Duration, Instant};

use vulkano::{sync, VulkanLibrary};
use vulkano::buffer::{Buffer, BufferCreateInfo, BufferUsage};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferExecFuture, CommandBufferUsage,
};
use vulkano::command_buffer::allocator::{
    StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
};
use vulkano::device::{Device, Queue};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::pipeline::{Pipeline, PipelineBindPoint};
use vulkano::sync::future::{FenceSignalFuture, NowFuture};
use vulkano::sync::GpuFuture;

mod device;
mod pipeline;

fn time_future(future: FenceSignalFuture<CommandBufferExecFuture<NowFuture>>) -> Duration {
    let t = Instant::now();
    future.wait(None).unwrap();
    t.elapsed()
}

fn vulkan_init() -> (Arc<Device>, Arc<Queue>) {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    // let physical_device = device::select_physical(instance);

    // println!(
    //     "Selected physical device: \x1b[32m{}\x1b[0m",
    //     physical_device.properties().device_name
    // );

    // let (device, queue) = device::create_device(physical_device);
    //
    // println!(
    //     "Created logical device with API version \x1b[32m{}\x1b[0m",
    //     device.api_version()
    // );
    // (device, queue)
    device::create_device(device::select_physical(instance))
}

fn main() {
    let (device, queue) = vulkan_init();

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let data_iter = 0..65536u32;
    let data_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data_iter,
    )
    .expect("failed to create buffer");

    let (compute_pipeline, descriptor_set_layout_index, descriptor_set) =
        pipeline::create_pipeline(&device, &data_buffer);

    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        &command_buffer_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let work_group_counts = [1024, 1, 1];

    command_buffer_builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .unwrap()
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            descriptor_set_layout_index as u32,
            descriptor_set,
        )
        .unwrap()
        .dispatch(work_group_counts)
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
