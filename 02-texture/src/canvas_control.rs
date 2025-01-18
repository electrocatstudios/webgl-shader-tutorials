use std::rc::Rc;

use web_sys::{window, HtmlCanvasElement,HtmlImageElement, WebGlProgram, WebGlRenderingContext as GL, WebGlUniformLocation};
use yew::prelude::*;

use wasm_bindgen::{prelude::*, JsCast};
use gloo_console::log;

pub struct CanvasControl {
    callback: Closure<dyn FnMut()>,
    canvas: Option<HtmlCanvasElement>,
    gl: Option<GL>,
    node_ref: NodeRef,
    last_update: f64,
    shader_program: Option<WebGlProgram>,
    time_location: Option<WebGlUniformLocation>,
    tri_count: i32,
    u_time: f32,
    height: i32,
    width: i32,
}

pub enum CanvasControlMsg {
    Render
}


#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct CanvasControlProps;

const TEXTURE_1: &str = "assets/noise.png";

impl Component for CanvasControl {
    type Message = CanvasControlMsg;
    type Properties = CanvasControlProps;

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(CanvasControlMsg::Render)) as Box<dyn FnMut()>);

        // Get window size and use this later for sizing the canvas to full screen
        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let height = window().unwrap().inner_height().unwrap().as_f64().unwrap();

        CanvasControl{
            callback: callback,
            canvas: None,
            gl: None,
            node_ref: NodeRef::default(),
            last_update: instant::now(),
            shader_program: None,
            time_location: None,
            tri_count: 0,
            u_time: 0.0,
            height: height as i32,
            width: width as i32,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            CanvasControlMsg::Render => {
                self.render();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="game_canvas">
                <canvas id="canvas"
                    style={"margin: 0px; width: 100vw; height: 100vh; left:0px; top:0px;"}
                    ref={self.node_ref.clone()}
                    tabindex = "1"
                ></canvas>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        // Grab context and other setup
        let c = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: GL = c
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        // Fill the screen
        c.set_width(self.width as u32);
        c.set_height(self.height as u32);

        // Store references to the canvas and GL context
        self.canvas = Some(c);
        self.gl = Some(gl);

        if first_render {
            // Load the scene - as it's the first time rendering
            self.reload();
            // Send message to internal message pump to start the render loop
            ctx.link().send_message(CanvasControlMsg::Render);
        }
    }
}

impl CanvasControl {

    fn canvas_update(&mut self) {
        let now = instant::now();

        if self.last_update >= now {
            // Somehow ended up time traveling - ignore
            return;
        }

        let diff = now - self.last_update; // Time since last frame in ms

        let delta = diff as f64 / 1000.0; // Convert to seconds
        self.u_time += delta as f32; // Update the u_time Uniform
    
        self.last_update = now; // Make sure we use the "now" from before so we don't miss time
    }

    fn reload(&mut self) {
        // Set up shaders and uniform locations
        let gl = match &self.gl {
            Some(gl)=> gl,
            None => {
                log!("ERROR Setting up scene without a proper gl context");
                return;
            }
        };

        // Double check we have a canvas - if not then return, something went wrong
        let _: &HtmlCanvasElement = match &self.canvas {
            Some(canv) => canv,
            None => return,
        };

        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 0.,
            1.0, -1.0, 0.,
            1.0, 1.0, 0.,
            -1.0, -1.0, 0.,
            -1.0, 1.0, 0.,
            1.0, 1.0, 0.
        ];

        // Store count of triangle points (each point is 3 coords)
        self.tri_count = vertices.len() as i32 / 3;

        let vertex_buffer = self.gl.clone().unwrap().create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

        // Set up the shaders - and compile them
        let vert_code = include_str!("./texture.vert");
        let frag_code = include_str!("./texture.frag");

        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, &vert_code);
        gl.compile_shader(&vert_shader);

        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, &frag_code);
        gl.compile_shader(&frag_shader);

        // Create the shader program and attach our now compiled shaders
        let shader_program: WebGlProgram = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        // Attach the position vector as an attribute for the GL context.
        let position = gl.get_attrib_location(&shader_program, "a_position") as u32;
        gl.vertex_attrib_pointer_with_i32(position, 3, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position);

        // Let the shader know it's resolution
        let canvassize = gl.get_uniform_location(&shader_program, "canvasSize");
        gl.uniform2f(canvassize.as_ref(), self.width as f32, self.height as f32);

        // Get and store the location of the time variable - which the program will tell to the
        // GPU so it can make calculations based on this time.
        self.time_location = gl.get_uniform_location(&shader_program, "u_time");
        gl.uniform1f(self.time_location.as_ref() , 1.0); //self.last_update as f32

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

        self.shader_program = Some(shader_program);
    }

    fn render(&mut self) {
        // Update internal state before rendering
        self.canvas_update();
        
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        gl.viewport(
            0,
            0,
            self.width as i32,
            self.height as i32,
        );

        gl.clear_color(0., 0.7, 0., 1.0);
        gl.clear_depth(1.0);

        // Enable the depth test
        gl.enable(GL::DEPTH_TEST);

        // Clear the color buffer bit
        gl.clear(GL::COLOR_BUFFER_BIT);
       
        // Update uniforms in the shaders - for now just the u_time (time since start in secs)
        gl.uniform1f(self.time_location.as_ref() , self.u_time as f32);

        gl.draw_arrays(GL::TRIANGLES, 0, self.tri_count);

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}