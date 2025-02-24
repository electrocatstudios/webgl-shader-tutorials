use std::{collections::HashMap, rc::Rc};

use euclid::Vector3D;

use gltf_json::{accessor, mesh::Semantic, validation::Checked};
use base64::prelude::*;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlImageElement, WebGlProgram, WebGlRenderingContext as GL};

use crate::utils;

pub struct Model {
    pub name: String,
    pub gltf: gltf_json::Root,
    pub position: Vector3D<f32, ()>,
    pub rotation: Vector3D<f32, ()>,
    pub scale: Vector3D<f32, ()>,
    pub buffers: HashMap<String, web_sys::WebGlBuffer>,
    pub pos_buffer: Option<web_sys::WebGlBuffer>,
    pub time_location: Option<web_sys::WebGlUniformLocation>,
    pub position_location: Option<u32>,
    pub shader_program: Option<WebGlProgram>,
    pub poly_count: usize
}

const TEXTURE_1: &str = "/assets/forest_scene.png";

impl Model {
    pub fn new(name: String, gltf: gltf_json::Root) -> Model {
        Model {
            name: name,
            gltf: gltf,
            position: Vector3D::new(0.0, 0.0, 0.0),
            rotation: Vector3D::new(0.0, 0.0, 0.0),
            scale: Vector3D::new(1.0, 1.0, 1.0),
            buffers: HashMap::new(),
            pos_buffer: None,
            position_location: None,
            time_location: None,
            shader_program: None,
            poly_count: 0,
        }
    }

    pub fn setup_shader(&mut self, gl: &GL, width: f32, height: f32) { // TODO: Add shader name so each one can have it's own
        let vert_code = include_str!("./fractal.vert");
        let frag_code = include_str!("./fractal.frag");
        
        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, &vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, &frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program: WebGlProgram = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);
        self.position_location = Some(position);

        let canvassize = gl.get_uniform_location(&shader_program, "canvasSize");
        gl.uniform2f(canvassize.as_ref(), width, height);

        self.time_location = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(self.time_location.as_ref() , 1.0);

        self.shader_program = Some(shader_program);
    }

    pub fn setup(&mut self, gl: &GL) {

        // Store count of triangle points (each point is 3 coords)
        // self.tri_count = vertices.len() as i32 / 3;
        // self.gltf.scenes.iter().for_each(|scene| {
            // scene.nodes.iter().for_each(|node| {
        // let node = self.gltf.nodes.get(0).unwrap();
        let mesh = self.gltf.meshes.get(0).unwrap();
        mesh.primitives.iter().for_each(|primitive| {
            
            for (attribute, value) in primitive.attributes.iter() {     
                let (ind, attr_name) = match attribute {
                    Checked::Valid(attr) => {
                        // gloo_console::log!("Attribute: ", attr.to_string(), value.to_string());
                        (value, attr.to_string().clone())
                    },
                    Checked::Invalid => {
                        gloo_console::log!("Invalid attribute: ");
                        continue;
                    }
                };

                let accessor = self.gltf.accessors.get(ind.value()).unwrap();

                let (component_count, byte_size, comp_typ) = utils::get_accessor_details(accessor);

                let buffer = match utils::get_data_from_buffer(primitive, &self.gltf, Semantic::Positions){
                    Ok(buffer) => {
                        gloo_console::log!("Buffer outside func: ", buffer.len());
                        buffer
                    },
                    Err(err) => {
                        gloo_console::log!("Error getting buffer: ", err);
                        continue;
                    }
                };
                gloo_console::log!("Done loading buffer data");
            
                let vertex_buffer = gl.create_buffer().unwrap();
                let mut conv_buffer: Vec<f32> = Vec::new();
                for i in (0..buffer.len()).step_by(4) {
                    let bytes: [u8; 4] = [buffer[i], buffer[i+1], buffer[i+2], buffer[i+3]];
                    let f = f32::from_le_bytes(bytes);
                    conv_buffer.push(f);
                }
                let verts = js_sys::Float32Array::from(conv_buffer.as_slice());
                gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

                // let data = buffer.data();
                // let start = buffer_view.byte_offset() + accessor.byte_offset();
                // let end = start + accessor.count() * accessor.size();
                // let vertices = data[start..end].to_vec();
                // // self.load_vertices(gl, vertices);

                // let vertex_buffer = gl.create_buffer().unwrap();
                // let verts = js_sys::Float32Array::from(vertices.as_slice());
                
                // self.poly_count += verts.length() as usize / 3;

                // gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                // gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
            
            };
        });
        // });
        

        gloo_console::log!("Poly count: {}", self.poly_count);

    }

    pub fn load_textures(&mut self, gl: &GL) {
                // Setup the texture 
        // based on https://snoozetime.github.io/2019/12/19/webgl-texture.html
        let texture = gl.create_texture().unwrap();
        gl.bind_texture(GL::TEXTURE_2D, Some(&texture));

        let image: HtmlImageElement = HtmlImageElement::new().unwrap();
        let imgrc = Rc::new(image.clone());

        {
            let image = imgrc.clone();
            let texture = texture.clone();
            let gl = Rc::new(gl.clone());

            let a = Closure::wrap(Box::new(move || {
                gl.bind_texture(GL::TEXTURE_2D, Some(&texture));
    
                let _ = gl.tex_image_2d_with_u32_and_u32_and_image(
                    GL::TEXTURE_2D,
                    0,
                    GL::RGBA.try_into().unwrap(),
                    GL::RGBA.try_into().unwrap(),
                    GL::UNSIGNED_BYTE,
                    &image,
                );
    
                // different from webgl1 where we need the pic to be power of 2
                gl.generate_mipmap(GL::TEXTURE_2D);
            }) as Box<dyn FnMut()>);

            imgrc.set_onload(Some(a.as_ref().unchecked_ref()));
    
            // Normally we'd store the handle to later get dropped at an appropriate
            // time but for now we want it to be a global handler so we use the
            // forget method to drop it without invalidating the closure. Note that
            // this is leaking memory in Rust, so this should be done judiciously!
            a.forget();
        }
        image.set_src(TEXTURE_1);

    }

    pub fn update(&mut self, time: f32) {
        // 
    }

    
}