use base64::prelude::*;
use gloo_console;
use gltf_json::{accessor::{self, ComponentType, GenericComponentType}, mesh::Semantic, validation::Checked, Accessor};
use gltf_json::mesh::*;

pub fn get_accessor_details(accessor: &Accessor) -> (i32, i32, ComponentType) {
    // Calculate the number of bytes per component and the number of components
    // gloo_console::log!("Accessor: ", accessor.type_.to_string());
    let ty = match accessor.type_ {
        Checked::Valid(val) => val,
        Checked::Invalid => {
            gloo_console::log!("Invalid accessor type");   
            return (0, 0, ComponentType::I8);
        }                            
    };
    let component_count = get_component_count_for_accessor_type(ty);

    let comp_ty = match accessor.component_type {
        Checked::Valid(val) => val,
        Checked::Invalid => {
            gloo_console::log!("Invalid component type");
            return (0, 0, ComponentType::I8);
        }                            
    };

    let byte_size = get_byte_size_for_component_type(comp_ty.0);
    
    (component_count, byte_size, comp_ty.0) 
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

pub fn get_data_from_buffer(primitive: &Primitive, gltf: &gltf_json::Root, semantic: Semantic) -> Result<Vec::<u8>, String>{

    let semantic: Checked<Semantic> = Checked::<Semantic>::Valid(semantic);
    let att = primitive.attributes.get(&semantic).unwrap();
    let accessor = gltf.accessors.get(att.value()).unwrap();
    let buffer_view = gltf.buffer_views.get(accessor.buffer_view.unwrap().value()).unwrap();
    let buffer = gltf.buffers.get(buffer_view.buffer.value()).unwrap();
    let buf_len: u64 = buffer.byte_length.0;
    gloo_console::log!("Buffer length: ", buf_len, buffer.uri.clone());
    let (comp_count, comp_size, _) = get_accessor_details(accessor);
    let chunk_size = (comp_count * comp_size) as usize;
    
    // Check if the buffer data is embedded (base64) or external
    match &buffer.uri {
        Some(uri) => {
            if uri.starts_with("data:application/octet-stream;base64,") {
                // Handle embedded base64 data
                let b64_data = uri.split(',').nth(1).unwrap().as_bytes();
                let decoded = BASE64_STANDARD.decode(b64_data).unwrap();
                
                // TODO: Collect the data into buffer
                gloo_console::log!("Decoded: ", decoded.len());
                let mut ret = Vec::<u8>::new();
                for i in 0..accessor.count.0 as usize {
                    let offset = i * buffer_view.byte_stride.unwrap().0 + accessor.byte_offset.unwrap().0 as usize;
                    for j in 0..chunk_size {
                        ret.push(decoded[offset + (i * chunk_size + j)]);
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