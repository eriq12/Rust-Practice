// using compute-shader-101 from Google
// source repo: https://github.com/googlefonts/compute-shader-101

// util for measuring time for how long a compute takes
use std::time::Instant;

use wgpu::util::DeviceExt;

use bytemuck;

async fn run() {
    // create instance that should run on Vulkan, Metal, DX12, or WebGPU Browser
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });
    // get adapter to contain device and queue with default options
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    // get features to add TIMESTAMP_QUERY to later when requesting device
    let features = adapter.features();
    // get device to bind buffers to and submit queries
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: features
                    & wgpu::Features::TIMESTAMP_QUERY,
                required_limits: Default::default(),
            },
            None
        ).await.unwrap();
    let query_set = if features.contains(wgpu::Features::TIMESTAMP_QUERY) {
        Some(device.create_query_set(&wgpu::QuerySetDescriptor {
            count: 2,
            ty: wgpu::QueryType::Timestamp,
            label: None,
        }))
    } else {
        None
    };
    // compile shaders
    let start_instant = Instant::now();
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
    println!("Shader compliation {:?}", start_instant.elapsed());

    // input and output_buffers
    let input_f = &[1.0f32; 32];
    let input: &[u8] = bytemuck::bytes_of(input_f);
    //buffer that holds inputf as bytes
    let input_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: input, // input converted from input_f into the buffer
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    // holds results from input_buf, no need for init as it will be written over
    let output_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: input.len() as u64, // makes sure the lengths of the buffers match
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let output_staging_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: input.len() as u64,
        usage:  wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    // buffer for query results
    let query_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 16,
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::QUERY_RESOLVE, // need to add to write query answers to
        mapped_at_creation: false,
    });
    // buffer to hold commands(query)?
    let query_staging_buf = device.create_buffer(&wgpu::BufferDescriptor{
        label: None,
        size: 16,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    // define layout of bind group
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty:wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty:wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None
            },
        ],
    });

    // define layout of compute_pipeline
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    // create the compute pipeline
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point: "main",
    });

    // create bind group
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry{
                binding: 0,
                resource: input_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry{
                binding: 1,
                resource: output_buf.as_entire_binding(),
            },
        ],
    });

    let mut encoder = device.create_command_encoder(&Default::default());
    // write timestamp for before
    if let Some(query_set) = &query_set {
        encoder.write_timestamp(query_set, 0);
    }
    // start compute
    {
        let mut cpass = encoder.begin_compute_pass(&Default::default());
        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.dispatch_workgroups(input_f.len() as u32, 1, 1);
    }
    // write timestamp for after
    if let Some(query_set) = &query_set {
        encoder.write_timestamp(query_set, 1);
    }
    encoder.copy_buffer_to_buffer(&output_buf, 0, &output_staging_buf, 0, input.len() as u64);
    // write results of timestamps? to destination query buffer
    if let Some(query_set) = &query_set {
        encoder.resolve_query_set(query_set, 0..2, &query_buf, 0);
    }
    encoder.copy_buffer_to_buffer(&query_buf, 0, &query_staging_buf, 0, 16);
    // submit all commands to queue to begin
    queue.submit(Some(encoder.finish()));

    let buf_slice = output_staging_buf.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    // (from the repo) assumes both buffers will be available at the same time
    // to be more careful, wait for both notifications to be sent first
    buf_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    let query_slice = query_staging_buf.slice(..);
    let _query_future = query_slice.map_async(wgpu::MapMode::Read, |_| ());
    println!("pre-poll {:?}",  std::time::Instant::now());
    // to finish async
    device.poll(wgpu::Maintain::Wait);
    println!("post-poll {:?}",  std::time::Instant::now());
    // on receive Ok, get the data and print
    if let Some(Ok(_)) = receiver.receive().await {
        // apparently the deref and ref makes it from BufferView to &[u8]?
        let data_raw = &*buf_slice.get_mapped_range();
        let data: &[f32] = bytemuck::cast_slice(data_raw);
        println!("data: {:?}", &*data);
    }
    // print the time elapsed
    if features.contains(wgpu::Features::TIMESTAMP_QUERY) {
        let ts_period = queue.get_timestamp_period();
        let ts_data_raw = &*query_slice.get_mapped_range();
        let ts_data: &[u64] = bytemuck::cast_slice(ts_data_raw);
        println!(
            "compute shader elapsed: {:?}ms",
            (ts_data[1] - ts_data[0]) as f64 * ts_period as f64 * 1e-6
        );
    }
}

fn main() {
    pollster::block_on(run());
}
