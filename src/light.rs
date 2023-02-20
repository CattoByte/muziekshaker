#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    pub position: [f32; 3],
    pub _padding: u32, // Uniform spacing has to be in powers of two, so padding is necessary.
    pub colour: [f32; 3],
    pub _padding2: u32,
}
