use base64::prelude::*;
use gloo_console;
use gltf_json::{accessor::{self, ComponentType}, mesh::Semantic, validation::Checked, Accessor};
use gltf_json::mesh::*;

pub fn get_accessor_details(accessor: &Accessor) -> (i32, i32, u64) {
    // Calculate the number of bytes per component and the number of components
    let ty = match accessor.type_ {
        Checked::Valid(val) => val,
        Checked::Invalid => {
            gloo_console::log!("Invalid accessor type");   
            return (0, 0, 0);
        }                            
    };
    let component_count = get_component_count_for_accessor_type(ty);

    let comp_ty = match accessor.component_type {
        Checked::Valid(val) => {
            val
        },
        Checked::Invalid => {
            gloo_console::log!("Invalid component type");
            return (0, 0, 0);
        }                            
    };

    let byte_size = get_byte_size_for_component_type(comp_ty.0);
    let triangle_count =  accessor.count.0 as u64;

    (component_count, byte_size, triangle_count) 
}

fn get_component_count_for_accessor_type(ty: accessor::Type) -> i32 {
    match ty {
        accessor::Type::Scalar => 1,
        accessor::Type::Vec2 => 2,
        accessor::Type::Vec3 => 3,
        accessor::Type::Vec4 => 4,
        accessor::Type::Mat2 => 4,
        accessor::Type::Mat3 => 9,
        accessor::Type::Mat4 => 16
    }
}

fn get_byte_size_for_component_type(comp_ty: ComponentType) -> i32 {
    match comp_ty {
        ComponentType::I8 => 1,
        ComponentType::U8 => 1,
        ComponentType::I16 => 2,
        ComponentType::U16 => 2,
        ComponentType::U32 => 4,
        ComponentType::F32 => 4
    }
}

pub struct BufferObject {
    pub triangle_count: u64,
    pub buffer: Vec<u8>,
    pub _byte_stride: u64,
    pub _byte_offset: u64,
}

impl BufferObject {
    pub fn new(byte_stride: u64, byte_offset: u64, triangle_count: u64) -> Self {
        BufferObject {
            triangle_count: triangle_count,
            buffer: Vec::<u8>::new(),
            _byte_stride: byte_stride,
            _byte_offset: byte_offset,
        }
    }
}

pub fn get_data_from_buffer(primitive: &Primitive, gltf: &gltf_json::Root, semantic_in: Semantic) -> Result<BufferObject, String>{

    let semantic: Checked<Semantic> = Checked::<Semantic>::Valid(semantic_in.clone());
    let att = primitive.attributes.get(&semantic).unwrap();
    let accessor = gltf.accessors.get(att.value()).unwrap();
    let buffer_view = gltf.buffer_views.get(accessor.buffer_view.unwrap().value()).unwrap();
    let buffer = gltf.buffers.get(buffer_view.buffer.value()).unwrap();
    let (comp_count, comp_size, triangle_count) = get_accessor_details(accessor);
    let chunk_size = (comp_count * comp_size) as usize;
    let mut ret = BufferObject::new(chunk_size as u64, 0, triangle_count);

    // Check if the buffer data is embedded (base64) or external
    match &buffer.uri {
        Some(uri) => {
            if uri.starts_with("data:application/octet-stream;base64,") {
                // Handle embedded base64 data
                let b64_data = uri.split(',').nth(1).unwrap().as_bytes();
                let decoded = BASE64_STANDARD.decode(b64_data).unwrap();
                
                ret.buffer = Vec::<u8>::new();
                for i in 0..accessor.count.0 as usize {
                    let offset = (i * buffer_view.byte_stride.unwrap().0) + accessor.byte_offset.unwrap().0 as usize;
                    for j in 0..chunk_size {
                        ret.buffer.push(decoded[offset + j]);
                    }
                }
                Ok(ret)

            } else {
                // Handle external file URI
                // You'll need to load this file separately using your asset loading system
                let buffer_path = format!("assets/{}", uri);
                gloo_console::log!("Buffer path: ", buffer_path.clone());
                // Load buffer_path...
                Err(buffer_path)
            }
        },
        None => {
            gloo_console::log!("No buffer URI found");
            // Handle GLB-stored buffer data
            // For GLB files, you need to extract the buffer data from the binary chunk
            Err("".to_string())
        }
    }
}

pub fn get_f32_buffer_from_u8(buffer: Vec<u8>) -> Vec<f32> {
    let mut ret = Vec::<f32>::new();
    for i in (0..buffer.len()).step_by(4) {
        // Convert each block of 4 to an f32        
        let bytes: [u8; 4] = [buffer[i], buffer[i+1], buffer[i+2], buffer[i+3]];
        let f = f32::from_le_bytes(bytes);
        ret.push(f);
    }

    ret
}