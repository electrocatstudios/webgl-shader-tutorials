use std::{collections::HashMap, rc::Rc};

use euclid::{Transform3D, Vector3D};

use gltf_json::mesh::Semantic;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlImageElement, WebGlProgram, WebGlRenderingContext as GL};

use crate::camera::Camera;
use crate::utils;

pub struct Model {
    pub _name: String,
    pub gltf: gltf_json::Root,
    pub matrix: Transform3D<f32, (), ()>,
    pub _position: Vector3D<f32, ()>,
    pub _rotation: Vector3D<f32, ()>,
    pub _scale: Vector3D<f32, ()>,
    pub _buffers: HashMap<String, web_sys::WebGlBuffer>,
    pub _pos_buffer: Option<web_sys::WebGlBuffer>,
    pub time_location: Option<web_sys::WebGlUniformLocation>,
    pub _position_location: Option<u32>,
    pub shader_program: Option<WebGlProgram>,
    pub poly_count: usize,
    // pub vao: Option<WebGlVertexArrayObject>,
}

const TEXTURE_1: &str = "/assets/forest_scene.png";

impl Model {
    pub fn new(name: String, gltf: gltf_json::Root) -> Model {
        Model {
            _name: name,
            gltf: gltf,
            matrix: Transform3D::identity(),
            _position: Vector3D::new(0.0, 0.0, 0.0),
            _rotation: Vector3D::new(0.0, 0.0, 0.0),
            _scale: Vector3D::new(1.0, 1.0, 1.0),
            _buffers: HashMap::new(),
            _pos_buffer: None,
            _position_location: None,
            time_location: None,
            shader_program: None,
            poly_count: 0,
        }
    }

    pub fn setup_shader(&mut self, gl: &GL, width: f32, height: f32) { // TODO: Add shader name so each one can have it's own
        let vert_code = include_str!("./simple.vert");
        let frag_code = include_str!("./simple.frag");
        
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
        let canvassize = gl.get_uniform_location(&shader_program, "u_screensize");
        gl.uniform2f(canvassize.as_ref(), width, height);

        self.time_location = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(self.time_location.as_ref() , 1.0);

        self.shader_program = Some(shader_program);
    }

    pub fn setup(&mut self, gl: &GL) {

        // TODO: loop through all meshes and perform this - rather than first one
        let mesh = self.gltf.meshes.get(0).unwrap();
        mesh.primitives.iter().for_each(|primitive| {
            /* Position Buffer */
            match utils::get_data_from_buffer(primitive, &self.gltf, Semantic::Positions){
                Ok(buffer) => {
                    let conv_buffer = utils::get_f32_buffer_from_u8(buffer.buffer);
                    self.poly_count = buffer.triangle_count as usize;
                    let vertex_buffer = gl.create_buffer().unwrap();
                    let verts = js_sys::Float32Array::from(conv_buffer.as_slice());
                    
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
                    
                    match &self.shader_program {
                        Some(sp) => {
                            let att = gl.get_attrib_location(sp, "a_position");
                            gl.vertex_attrib_pointer_with_i32(att as u32, 3, GL::FLOAT, false, 0, 0);
                            gl.enable_vertex_attrib_array(att as u32);
                        },
                        None => {
                            gloo_console::log!("No shader program found while setting a_position");
                        } 
                    }
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);
                },
                Err(err) => {
                    gloo_console::log!("Error getting position buffer: ", err);
                    return; // Can't continue witout positions
                }
            };                
            /* End of Position Buffer */

            /* Normal Buffer */
            match utils::get_data_from_buffer(primitive, &self.gltf, Semantic::Normals){
                Ok(buffer) => {
                    let conv_buffer = utils::get_f32_buffer_from_u8(buffer.buffer);
                    let vertex_buffer = gl.create_buffer().unwrap();                
                    let verts = js_sys::Float32Array::from(conv_buffer.as_slice());
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
                    match &self.shader_program {
                        Some(sp) => {
                            let att = gl.get_attrib_location(sp, "a_normal");
                            gl.vertex_attrib_pointer_with_i32(att as u32, 3, GL::FLOAT, false, 0, 0);
                            gl.enable_vertex_attrib_array(att as u32);
                        },
                        None => {
                            gloo_console::log!("No shader program found while setting a_position");
                        } 
                    }
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);
                },
                Err(err) => {
                    gloo_console::log!("Error getting normal buffer: ", err);
                }
            };
            /* End Normal Buffer */ 
            
            /* Tex_coord Buffer */
            // TODO: Support multiple tex coords
            match utils::get_data_from_buffer(primitive, &self.gltf, Semantic::TexCoords(0)){
                Ok(buffer) => {
                    let conv_buffer = utils::get_f32_buffer_from_u8(buffer.buffer);
                    let vertex_buffer = gl.create_buffer().unwrap();                
                    let verts = js_sys::Float32Array::from(conv_buffer.as_slice());
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
                    
                    match &self.shader_program {
                        Some(sp) => {
                            let att = gl.get_attrib_location(sp, "a_texcoord");
                            gl.vertex_attrib_pointer_with_i32(att as u32, 2, GL::FLOAT, false, 0, 0);
                            gl.enable_vertex_attrib_array(att as u32);
                        },
                        None => {} 
                    }
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);
                },
                Err(err) => {
                    gloo_console::log!("Error getting tex coord buffer: ", err);
                }
            };
            /* End Tex_coord Buffer */ 

            /* Color Buffer */
            // TODO: Support multiple Color buffers
            match utils::get_data_from_buffer(primitive, &self.gltf, Semantic::Colors(0)){
                Ok(buffer) => {
                    let conv_buffer = utils::get_f32_buffer_from_u8(buffer.buffer);

                    let vertex_buffer = gl.create_buffer().unwrap();                
                    let verts = js_sys::Float32Array::from(conv_buffer.as_slice());
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);
                    match &self.shader_program {
                        Some(sp) => {
                            let att = gl.get_attrib_location(sp, "a_color");
                            gl.vertex_attrib_pointer_with_i32(att as u32, 3, GL::FLOAT, false, 0, 0);
                            gl.enable_vertex_attrib_array(att as u32);
                        },
                        None => {}
                    }
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);
                },
                Err(err) => {
                    gloo_console::log!("Error getting tex coord buffer: ", err);
                }
            };
            /* End Color Buffer */ 

            // Other buffers                
        });
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
        self.matrix = self.matrix.then_rotate(0.0, 1.0, 0.0, euclid::Angle { radians: time.sin() * std::f32::consts::PI });
    }

    pub fn render(&mut self, gl: &GL, time: f32, camera: &Camera) { //projection: Transform3D<f32, (), ()>, view: Transform3D<f32, (), ()>
        gl.use_program(self.shader_program.as_ref());
        
        // Update uniforms
        let canvassize = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_screensize");
        gl.uniform2f(canvassize.as_ref(), camera.width, camera.height);

        let proj_loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_projection");
        let vals: [f32; 16] = camera.projection.to_array();
        gl.uniform_matrix4fv_with_f32_array(proj_loc.as_ref() , false, &vals);

        let view_loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_view");
        let vals: [f32; 16] = camera.view.to_array();
        gl.uniform_matrix4fv_with_f32_array(view_loc.as_ref() , false, &vals);

        let model_loc = gl.get_uniform_location(&self.shader_program.as_mut().unwrap(), "u_model");
        let vals: [f32; 16] = self.matrix.to_array();
        gl.uniform_matrix4fv_with_f32_array(model_loc.as_ref() , false, &vals);

        gl.uniform1f(self.time_location.as_ref() , time);

        gl.draw_arrays(GL::TRIANGLES, 0, self.poly_count as i32);
    }
    
}