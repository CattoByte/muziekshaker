use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

mod camera;
mod model;
mod resources;
mod texture;

use model::{DrawModel, Vertex};

/*
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[rustfmt::skip]
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.9,   -0.9,    0.0],   tex_coords: [0.0,   0.0] },
    Vertex { position: [0.9,    -0.9,    0.0],   tex_coords: [1.0,   0.0] },
    Vertex { position: [0.0,    0.9,   0.0],   tex_coords: [0.5,   1.0] },
];
const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.1,   -0.9,   0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.1,    -0.9,   0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [-0.05,  -0.4,   0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.05,   -0.4,   0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [-0.2,   -0.3,   0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.2,    -0.3,   0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [-0.25,  -0.2,   0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.25,   -0.2,   0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [-0.35,  0.0,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.35,   0.0,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [-0.4,   0.2,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.4,    0.2,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [-0.375, 0.4,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.375,  0.4,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [-0.25,  0.6,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.25,   0.6,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [-0.1,   0.8,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.1,    0.8,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [-0.05,  0.9,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.05,   0.9,    0.0],   tex_coords: [1.0,   1.0] },
    Vertex { position: [0.0,    1.0,    0.0],   tex_coords: [1.0,   1.0] },
];*/

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,
];
/*const INDICES: &[u16] = &[
    0,  1,  2,
    3,  2,  1,
    2,  3,  4,
    5,  4,  3,
    4,  5,  6,
    7,  6,  5,
    6,  7,  8,  //here
    9,  7,  8,
    10,  9,  8,
    11,  10, 9,
    12, 11, 10,
    13, 12, 11,
    14, 13, 12,
    15, 14, 13,
    16, 15, 14,
    17, 16, 15,
    18, 17, 16,
    19, 18, 17,
    19, 20, 18,
];*/

const NUM_INSTANCES_PER_ROW: u32 = 10;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
    0.0,
    NUM_INSTANCES_PER_ROW as f32 * 0.5,
);

struct Instance {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position)
                * cgmath::Matrix4::from(self.rotation))
            .into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

struct State {
    // Basic
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    // Rendering
    render_pipeline: wgpu::RenderPipeline,
    //vertex_buffer: wgpu::Buffer,
    //index_buffer: wgpu::Buffer,
    //num_indices: u32,
    diffuse_texture: texture::Texture,
    diffuse_bind_group: wgpu::BindGroup,
    depth_texture: texture::Texture,
    obj_model: model::Model,

    // Camera
    camera: camera::Camera,
    camera_controller: camera::CameraController,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,

    // Instancing
    instances: Vec<Instance>,
    instance_buffer: wgpu::Buffer,
}

impl State {
    //Part of wgpu's initialization requires async code.
    async fn new(window: &Window, render_mode: wgpu::PolygonMode) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all()); //VK + MTL + DX12 (and WebGPU, but that one won't be used)
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::POLYGON_MODE_LINE
                        | wgpu::Features::POLYGON_MODE_POINT,
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, //Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        let leaf_bytes = include_bytes!("../res/leaf.webp");
        let leaf_texture =
            texture::Texture::from_bytes(&device, &queue, leaf_bytes, "leaf.webp").unwrap();
        let autumn_leaf_bytes = include_bytes!("../res/autumn-leaf.webp");
        let autumn_leaf_texture =
            texture::Texture::from_bytes(&device, &queue, autumn_leaf_bytes, "leaf-autumn.webp")
                .unwrap();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let leaf_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&leaf_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&leaf_texture.sampler),
                },
            ],
            label: Some("leaf_bind_group"),
        });

        let autumn_leaf_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&autumn_leaf_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&autumn_leaf_texture.sampler),
                },
            ],
            label: Some("autumn_leaf_bind_group"),
        });

        let camera = camera::Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 90.0,
            znear: 0.1,
            zfar: 100.0,
        };
        let camera_controller = camera::CameraController::new(0.05);

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        }); // maybe replace the shader module with 'wgpu::include_wgsl!("shader.wgsl")'?

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[model::ModelVertex::desc(), InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // Fed triangles.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // A counter-clockwise face means that the triangle is facing forward.
                cull_mode: Some(wgpu::Face::Back), // The rest go into here.
                //polygon_mode: (|&mode| {if (mode == 0 as u8) {wgpu::PolygonMode::Fill} else {wgpu::PolygonMode::Fill}})(render_debug), // Anything else requires 'Features::NON_FILL_POLYGON'
                polygon_mode: render_mode,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        /*
                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(VERTICES),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(INDICES),
                    usage: wgpu::BufferUsages::INDEX,
                });
                let num_indices = INDICES.len() as u32;
        */
        const SPACE_BETWEEN: f32 = 3.0;
        let instances = (0..NUM_INSTANCES_PER_ROW)
            .flat_map(|z| {
                (0..NUM_INSTANCES_PER_ROW).map(move |x| {
                    use cgmath::prelude::*;
                    let x = SPACE_BETWEEN * (x as f32 - NUM_INSTANCES_PER_ROW as f32 / 3.0);
                    let z = SPACE_BETWEEN * (z as f32 - NUM_INSTANCES_PER_ROW as f32 / 3.0);

                    let position = cgmath::Vector3 { x, y: 0.0, z };
                    let rotation = if position.is_zero() {
                        cgmath::Quaternion::from_axis_angle(
                            cgmath::Vector3::unit_z(),
                            cgmath::Deg(0.0),
                        )
                    } else {
                        cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
                    };

                    Instance { position, rotation }
                })
            })
            .collect::<Vec<_>>();

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let obj_model =
            resources::load_model("cube.obj", &device, &queue, &texture_bind_group_layout).unwrap();

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            //vertex_buffer,
            //index_buffer,
            //num_indices,
            diffuse_texture: leaf_texture,
            diffuse_bind_group: leaf_bind_group,
            depth_texture,
            obj_model,
            camera,
            camera_controller,
            camera_uniform,
            camera_buffer,
            camera_bind_group,
            instances,
            instance_buffer,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
            self.depth_texture =
                texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        /*match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(VirtualKeyCode::Space),
                        ..
                    },
                ..
            } => {
                self.space_pressed = *state == ElementState::Pressed;
                println!("{}", self.space_pressed);
                true
            }
            _ => false,
        }*/
        self.camera_controller.process_events(event)
    }

    fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            //Needed to release mutable borrow
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            /*let texture_bind_group = if self.space_pressed {
                &self.autumn_leaf_bind_group
            } else {
                &self.leaf_bind_group
            };*/

            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_pipeline(&self.render_pipeline);

            let mesh = &self.obj_model.meshes[0];
            let material = &self.obj_model.materials[mesh.material];

            render_pass.draw_mesh_instanced(
                mesh,
                material,
                0..self.instances.len() as u32,
                &self.camera_bind_group,
            );

            /*render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // I know for a fact listening to ‘ゴ　チ　ャ　ゴ　チ　ャ　う　る　せ　ー　！　！　！’ can't be a good idea.
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);*/
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut render_mode = wgpu::PolygonMode::Fill;
    let mut state = State::new(&window, render_mode).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if (window_id == window.id()) => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::F1),
                                ..
                            },
                        ..
                    } => {
                        if render_mode == wgpu::PolygonMode::Fill {
                            render_mode = wgpu::PolygonMode::Line;
                        } else if render_mode == wgpu::PolygonMode::Line {
                            render_mode = wgpu::PolygonMode::Point;
                        } else {
                            render_mode = wgpu::PolygonMode::Fill;
                        }
                        pollster::block_on(async {
                            //todo!(); //This is absolutely deplorable, move this into the state itself.
                            //It literally brings up a wgpu error but somehow keeps running.
                            state = State::new(&window, render_mode).await;
                        });
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {
                    state.resize(state.size);
                }
                Err(wgpu::SurfaceError::Lost) => {
                    state.resize(state.size);
                }
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
}

/*
 * What might be the issue:
 *  resources.rs    No.
 *  model.rs        Doubt it.
 *  lib.rs          ???
 *  texture.rs      No?
 *  shader.wgsl     No.
 *
 *  It's probably the fact that I'm not using the vertex layout from the model and am instead
 *  relying on the pre-existing layout (which is unaware of normal textures).
 *  It's probably just treating normal textures as more model data.
 */
