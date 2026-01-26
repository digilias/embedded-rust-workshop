use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Point3, Rad, Vector3, Deg};
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::util::DeviceExt;
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
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
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
struct UniformData {
    view_proj: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Shape {
    Cube,
    Pyramid,
    Torus,
    Cylinder,
}

#[derive(Debug, Clone)]
pub struct ShapeInstance {
    pub shape: Shape,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: f32,
}

struct ShapeGeometry {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    edge_indices: Vec<u16>,
}

const MAX_INSTANCES: usize = 64;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    wireframe_pipeline: wgpu::RenderPipeline,
    shape_buffers: HashMap<Shape, (wgpu::Buffer, wgpu::Buffer, u32, wgpu::Buffer, u32)>,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    uniform_alignment: u32,
    depth_texture: wgpu::TextureView,
    instances: Vec<ShapeInstance>,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window).expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to get adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Calculate aligned uniform size (must be multiple of 256 for dynamic uniform buffers)
        let uniform_size = std::mem::size_of::<UniformData>() as u32;
        let uniform_alignment = 256u32; // wgpu requires 256-byte alignment for dynamic offsets
        let aligned_uniform_size = (uniform_size + uniform_alignment - 1) & !(uniform_alignment - 1);

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: (aligned_uniform_size as usize * MAX_INSTANCES) as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: wgpu::BufferSize::new(uniform_size as u64),
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &uniform_buffer,
                    offset: 0,
                    size: wgpu::BufferSize::new(uniform_size as u64),
                }),
            }],
            label: Some("uniform_bind_group"),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None, // No blending - fully opaque
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
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

        // Create wireframe pipeline for outlines
        let wireframe_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Wireframe Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_wireframe",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None, // No blending - fully opaque wireframe
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
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

        let depth_texture = Self::create_depth_texture(&device, &config);

        // Create shape geometries
        let mut shape_buffers = HashMap::new();

        // Add all shapes
        for shape_type in &[Shape::Cube, Shape::Pyramid, Shape::Torus, Shape::Cylinder] {
            let geometry = Self::create_shape_geometry(shape_type);

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", shape_type)),
                contents: bytemuck::cast_slice(&geometry.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", shape_type)),
                contents: bytemuck::cast_slice(&geometry.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            let edge_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Edge Buffer", shape_type)),
                contents: bytemuck::cast_slice(&geometry.edge_indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            shape_buffers.insert(
                shape_type.clone(),
                (vertex_buffer, index_buffer, geometry.indices.len() as u32, edge_buffer, geometry.edge_indices.len() as u32),
            );
        }

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            wireframe_pipeline,
            shape_buffers,
            uniform_buffer,
            uniform_bind_group,
            uniform_alignment: aligned_uniform_size,
            depth_texture,
            instances: Vec::new(),
        }
    }

    fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        depth_texture.create_view(&wgpu::TextureViewDescriptor::default())
    }

    fn create_shape_geometry(shape: &Shape) -> ShapeGeometry {
        match shape {
            Shape::Cube => Self::create_cube(),
            Shape::Pyramid => Self::create_pyramid(),
            Shape::Torus => Self::create_torus(16, 8, 0.5, 0.2),
            Shape::Cylinder => Self::create_cylinder(16),
        }
    }

    fn create_cube() -> ShapeGeometry {
        let vertices = vec![
            // Front face - red
            Vertex { position: [-0.5, -0.5, 0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [0.5, 0.5, 0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, 0.5, 0.5], color: [1.0, 0.0, 0.0] },
            // Back face - green
            Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, 0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-0.5, 0.5, -0.5], color: [0.0, 1.0, 0.0] },
            // Top face - blue
            Vertex { position: [-0.5, 0.5, 0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, 0.5, 0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, 0.5, -0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [-0.5, 0.5, -0.5], color: [0.0, 0.0, 1.0] },
            // Bottom face - yellow
            Vertex { position: [-0.5, -0.5, 0.5], color: [1.0, 1.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.5], color: [1.0, 1.0, 0.0] },
            Vertex { position: [0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
            Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0] },
            // Right face - magenta
            Vertex { position: [0.5, -0.5, 0.5], color: [1.0, 0.0, 1.0] },
            Vertex { position: [0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0] },
            Vertex { position: [0.5, 0.5, -0.5], color: [1.0, 0.0, 1.0] },
            Vertex { position: [0.5, 0.5, 0.5], color: [1.0, 0.0, 1.0] },
            // Left face - cyan
            Vertex { position: [-0.5, -0.5, 0.5], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-0.5, 0.5, -0.5], color: [0.0, 1.0, 1.0] },
            Vertex { position: [-0.5, 0.5, 0.5], color: [0.0, 1.0, 1.0] },
        ];

        let indices = vec![
            0, 1, 2, 2, 3, 0,       // front
            4, 6, 5, 6, 4, 7,       // back
            8, 9, 10, 10, 11, 8,    // top
            12, 14, 13, 14, 12, 15, // bottom
            16, 17, 18, 18, 19, 16, // right
            20, 22, 21, 22, 20, 23, // left
        ];

        // Define edges of the cube (12 edges total)
        let edge_indices = vec![
            // Front face edges
            0, 1,  1, 2,  2, 3,  3, 0,
            // Back face edges
            4, 5,  5, 6,  6, 7,  7, 4,
            // Connecting edges from front to back
            0, 4,  1, 5,  2, 6,  3, 7,
        ];

        ShapeGeometry { vertices, indices, edge_indices }
    }

    fn create_sphere(segments: u32, rings: u32) -> ShapeGeometry {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for ring in 0..=rings {
            let theta = ring as f32 * std::f32::consts::PI / rings as f32;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for segment in 0..=segments {
                let phi = segment as f32 * 2.0 * std::f32::consts::PI / segments as f32;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;

                // Use position-based coloring
                vertices.push(Vertex {
                    position: [x * 0.5, y * 0.5, z * 0.5],
                    color: [(x + 1.0) * 0.5, (y + 1.0) * 0.5, (z + 1.0) * 0.5],
                });
            }
        }

        for ring in 0..rings {
            for segment in 0..segments {
                let current = ring * (segments + 1) + segment;
                let next = current + segments + 1;

                indices.push(current as u16);
                indices.push(next as u16);
                indices.push(current as u16 + 1);

                indices.push(current as u16 + 1);
                indices.push(next as u16);
                indices.push(next as u16 + 1);
            }
        }

        // Create edge indices for sphere wireframe
        let mut edge_indices = Vec::new();
        for ring in 0..rings {
            for segment in 0..segments {
                let current = ring * (segments + 1) + segment;
                let next_ring = current + segments + 1;
                let next_segment = current + 1;

                // Vertical edge
                edge_indices.push(current as u16);
                edge_indices.push(next_ring as u16);

                // Horizontal edge
                edge_indices.push(current as u16);
                edge_indices.push(next_segment as u16);
            }
        }

        ShapeGeometry { vertices, indices, edge_indices }
    }

    fn create_pyramid() -> ShapeGeometry {
        let vertices = vec![
            // Base - green
            Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, -0.5, -0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [0.5, -0.5, 0.5], color: [0.0, 1.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.5], color: [0.0, 1.0, 0.0] },
            // Apex - white
            Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 1.0, 1.0] },
            // Side vertices with different colors
            Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 0.0] },
            Vertex { position: [0.5, -0.5, -0.5], color: [0.0, 0.0, 1.0] },
            Vertex { position: [0.5, -0.5, 0.5], color: [1.0, 1.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.5], color: [1.0, 0.0, 1.0] },
        ];

        let indices = vec![
            // Base
            0, 2, 1,
            2, 0, 3,
            // Sides
            5, 6, 4,  // front
            6, 7, 4,  // right
            7, 8, 4,  // back
            8, 5, 4,  // left
        ];

        // Define edges of the pyramid (8 edges total)
        let edge_indices = vec![
            // Base edges
            0, 1,  1, 2,  2, 3,  3, 0,
            // Edges from base to apex
            0, 4,  1, 4,  2, 4,  3, 4,
        ];

        ShapeGeometry { vertices, indices, edge_indices }
    }

    fn create_torus(major_segments: u32, minor_segments: u32, major_radius: f32, minor_radius: f32) -> ShapeGeometry {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for i in 0..=major_segments {
            let theta = i as f32 * 2.0 * std::f32::consts::PI / major_segments as f32;
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            for j in 0..=minor_segments {
                let phi = j as f32 * 2.0 * std::f32::consts::PI / minor_segments as f32;
                let cos_phi = phi.cos();
                let sin_phi = phi.sin();

                let x = (major_radius + minor_radius * cos_phi) * cos_theta;
                let y = minor_radius * sin_phi;
                let z = (major_radius + minor_radius * cos_phi) * sin_theta;

                vertices.push(Vertex {
                    position: [x, y, z],
                    color: [(cos_theta + 1.0) * 0.5, (sin_theta + 1.0) * 0.5, (cos_phi + 1.0) * 0.5],
                });
            }
        }

        for i in 0..major_segments {
            for j in 0..minor_segments {
                let current = i * (minor_segments + 1) + j;
                let next = current + minor_segments + 1;

                indices.push(current as u16);
                indices.push(next as u16);
                indices.push(current as u16 + 1);

                indices.push(current as u16 + 1);
                indices.push(next as u16);
                indices.push(next as u16 + 1);
            }
        }

        // Create edge indices for the torus wireframe
        let mut edge_indices = Vec::new();
        for i in 0..major_segments {
            for j in 0..minor_segments {
                let current = i * (minor_segments + 1) + j;
                let next_major = ((i + 1) % major_segments) * (minor_segments + 1) + j;
                let next_minor = current + 1;

                // Edge along major circle
                edge_indices.push(current as u16);
                edge_indices.push(next_major as u16);

                // Edge along minor circle
                edge_indices.push(current as u16);
                edge_indices.push(next_minor as u16);
            }
        }

        ShapeGeometry { vertices, indices, edge_indices }
    }

    fn create_cylinder(segments: u32) -> ShapeGeometry {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Create cylinder body
        for i in 0..=segments {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / segments as f32;
            let x = angle.cos() * 0.5;
            let z = angle.sin() * 0.5;

            // Top circle
            vertices.push(Vertex {
                position: [x, 0.5, z],
                color: [0.0, 1.0, 1.0],
            });
            // Bottom circle
            vertices.push(Vertex {
                position: [x, -0.5, z],
                color: [1.0, 0.0, 1.0],
            });
        }

        // Create cylinder sides
        for i in 0..segments {
            let idx = i * 2;
            indices.push(idx as u16);
            indices.push(idx as u16 + 1);
            indices.push(idx as u16 + 2);

            indices.push(idx as u16 + 1);
            indices.push(idx as u16 + 3);
            indices.push(idx as u16 + 2);
        }

        // Add center vertices for caps
        let top_center = vertices.len() as u16;
        vertices.push(Vertex {
            position: [0.0, 0.5, 0.0],
            color: [0.0, 1.0, 1.0],
        });
        
        let bottom_center = vertices.len() as u16;
        vertices.push(Vertex {
            position: [0.0, -0.5, 0.0],
            color: [1.0, 0.0, 1.0],
        });

        // Create top and bottom caps
        for i in 0..segments {
            // Top cap
            indices.push(top_center);
            indices.push((i * 2) as u16);
            indices.push(((i + 1) * 2) as u16);

            // Bottom cap
            indices.push(bottom_center);
            indices.push(((i + 1) * 2 + 1) as u16);
            indices.push((i * 2 + 1) as u16);
        }

        // Create edge indices for the cylinder wireframe
        let mut edge_indices = Vec::new();
        for i in 0..segments {
            let top = (i * 2) as u16;
            let bottom = (i * 2 + 1) as u16;
            let next_top = (((i + 1) % segments) * 2) as u16;
            let next_bottom = (((i + 1) % segments) * 2 + 1) as u16;

            // Top rim circle
            edge_indices.push(top);
            edge_indices.push(next_top);

            // Bottom rim circle
            edge_indices.push(bottom);
            edge_indices.push(next_bottom);

            // Vertical edge
            edge_indices.push(top);
            edge_indices.push(bottom);

            // Spokes from center to rim
            edge_indices.push(top_center);
            edge_indices.push(top);

            edge_indices.push(bottom_center);
            edge_indices.push(bottom);
        }

        ShapeGeometry { vertices, indices, edge_indices }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.depth_texture = Self::create_depth_texture(&self.device, &self.config);
        }
    }

    pub fn update(&mut self, instances: Vec<ShapeInstance>) {
        // Store instances for rendering
        self.instances = instances;
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Set up view projection matrix
        let aspect = self.config.width as f32 / self.config.height as f32;
        let camera_view = Matrix4::look_at_rh(
            Point3::new(0.0, 0.0, 30.0),
            Point3::new(0.0, 0.0, 0.0),
            Vector3::unit_y(),
        );
        let proj = cgmath::perspective(Deg(45.0), aspect, 0.1, 100.0);
        let view_proj = proj * camera_view;

        // Write all uniform data BEFORE creating the render pass
        for (index, instance) in self.instances.iter().enumerate() {
            let translation = Matrix4::from_translation(instance.position);
            let rotation = Matrix4::from_angle_x(Rad(instance.rotation.x))
                * Matrix4::from_angle_y(Rad(instance.rotation.y))
                * Matrix4::from_angle_z(Rad(instance.rotation.z));
            let scale = Matrix4::from_scale(instance.scale);
            let model = translation * rotation * scale;

            let uniform_data = UniformData {
                view_proj: view_proj.into(),
                model: model.into(),
            };

            let offset = (index as u64) * (self.uniform_alignment as u64);
            self.queue.write_buffer(
                &self.uniform_buffer,
                offset,
                bytemuck::cast_slice(&[uniform_data]),
            );
        }

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.2,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // First pass: render filled shapes
            render_pass.set_pipeline(&self.render_pipeline);

            for (index, instance) in self.instances.iter().enumerate() {
                let dynamic_offset = (index as u32) * self.uniform_alignment;
                render_pass.set_bind_group(0, &self.uniform_bind_group, &[dynamic_offset]);

                // Draw the appropriate shape
                if let Some((vertex_buffer, index_buffer, index_count, _, _)) =
                    self.shape_buffers.get(&instance.shape) {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..*index_count, 0, 0..1);
                }
            }

            // Second pass: render wireframe outlines
            render_pass.set_pipeline(&self.wireframe_pipeline);

            for (index, instance) in self.instances.iter().enumerate() {
                let dynamic_offset = (index as u32) * self.uniform_alignment;
                render_pass.set_bind_group(0, &self.uniform_bind_group, &[dynamic_offset]);

                // Draw wireframe using edge indices
                if let Some((vertex_buffer, _, _, edge_buffer, edge_count)) =
                    self.shape_buffers.get(&instance.shape) {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_index_buffer(edge_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    render_pass.draw_indexed(0..*edge_count, 0, 0..1);
                }
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
