use std::rc::Rc;

use web_sys::{window, HtmlCanvasElement, HtmlImageElement, WebGlProgram, WebGlRenderingContext as GL, WebGlUniformLocation};
use yew::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::*;
use wasm_bindgen::{prelude::*, JsCast};
use gloo_console::log;
use gltf_json;
use std::collections::HashMap;

use super::camera::Camera;
use super::model::Model;

pub struct CanvasControl {
    callback: Closure<dyn FnMut()>,
    canvas: Option<HtmlCanvasElement>,
    gl: Option<GL>,
    node_ref: NodeRef,
    last_update: f64,
    shader_program: Option<WebGlProgram>,
    time_location: Option<WebGlUniformLocation>,
    models: Vec::<Model>,
    tri_count: i32,
    u_time: f32,
    height: i32,
    width: i32,
    mouse_x: f32,
    mouse_x_loc: Option<WebGlUniformLocation>,
    mouse_y: f32,
    mouse_y_loc: Option<WebGlUniformLocation>,
    in_render_loop: bool,
    camera: Camera
}

pub enum CanvasControlMsg {
    MouseDown((f64, f64)),
    MouseUp((f64,f64)),
    MouseMove((f64,f64)),
    TouchStart((f64, f64)),
    TouchEnd((f64, f64)),
    TouchMove((f64, f64)),
    ModelReceived(String, String),
    Render,
    Null
}


#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct CanvasControlProps;


impl Component for CanvasControl {
    type Message = CanvasControlMsg;
    type Properties = CanvasControlProps;

    fn create(ctx: &Context<Self>) -> Self {
        let comp_ctx = ctx.link().clone();
        let callback =
            Closure::wrap(Box::new(move || comp_ctx.send_message(CanvasControlMsg::Render)) as Box<dyn FnMut()>);

        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let height = window().unwrap().inner_height().unwrap().as_f64().unwrap();

        let comp_ctx = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            let filename = "assets/cube.gltf".to_string();
            let response = Request::get(&filename.clone())
                .header("Content-Type", "application/json")
                .send()
                .await;
    
            match response {
                Ok(resp) => {
                    // gloo_console::log!("Got response");
                    let filecontent = match resp.text().await {
                        Ok(val) => val,
                        Err(err) => {
                            gloo_console::log!("Error while getting gltf: {:?}", err.to_string());
                            return;
                        }
                    };
                    // gloo_console::log!(val);
                    // gloo_console::log!(resp.json().await.unwrap());
                    
                    // let model = gltf_json::deserialize::from_reader(resp.body().unwrap().get_reader()).unwrap();
                    // if let Ok(data) = 
                    // resp.json::<gltf::GLTF>().await {
                        comp_ctx.send_message(CanvasControlMsg::ModelReceived(filename, filecontent));
                    // }
                }
                Err(err) => {
                    gloo_console::log!("Error while getting gltf: {:?}", err.to_string());
                    // comp_ctx.send_message(Msg::RequestError(err.to_string()));
                }
            }
        });

        CanvasControl{
            callback: callback,
            canvas: None,
            gl: None,
            node_ref: NodeRef::default(),
            last_update: instant::now(),
            shader_program: None,
            time_location: None,
            models: Vec::new(), //HashMap::new(),
            tri_count: 0,
            u_time: 0.0,
            height: height as i32,
            width: width as i32,
            mouse_x: 0.85,
            mouse_x_loc: None,
            mouse_y: 0.85,
            mouse_y_loc: None,
            in_render_loop: false,
            camera: Camera::new()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            CanvasControlMsg::MouseDown(evt) => {
                self.mouse_x = evt.0 as f32 / self.width as f32;
                self.mouse_y = evt.1 as f32 / self.height as f32;
                true
            },
            CanvasControlMsg::MouseUp(_evt) => {
                true
            },
            CanvasControlMsg::MouseMove(_evt) => {
                true
            },
            CanvasControlMsg::TouchStart(evt) => {
                self.mouse_x = evt.0 as f32 / self.width as f32;
                self.mouse_y = evt.1 as f32 / self.height as f32;
                true
            },
            CanvasControlMsg::TouchEnd(_evt) => {
                true
            },
            CanvasControlMsg::TouchMove(evt) => {
                self.mouse_x = evt.0 as f32 / self.width as f32;
                self.mouse_y = evt.1 as f32 / self.height as f32;
                true
            },
            CanvasControlMsg::Render => {
           
                // if !self.in_render_loop{
                //     self.in_render_loop = true;
                    // gloo_console::log!("Render called");
                    self.render();
                // }
                true
            },
            CanvasControlMsg::ModelReceived(name, content) => {
                gloo_console::log!("Model received! ", name.clone());
                // gloo_console::log!("Model content: ", content.clone());
                let gltf: gltf_json::Root = gltf_json::deserialize::from_reader(content.as_bytes()).unwrap();
                // gloo_console::log!(gltf.accessors.len());
                self.models.push(Model::new(name, gltf));
                self.reload();

                ctx.link().send_message(CanvasControlMsg::Render);
                true
            },
            CanvasControlMsg::Null => {
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousedown = ctx.link().callback(move |evt: MouseEvent| {
            CanvasControlMsg::MouseDown((evt.page_x() as f64, evt.page_y() as f64))
        });
        let onmousemove = ctx.link().callback(move |evt: MouseEvent| {
            CanvasControlMsg::MouseMove((evt.page_x() as f64, evt.page_y() as f64))
        });
        let onmouseup = ctx.link().callback(move |evt: MouseEvent| {
            CanvasControlMsg::MouseUp((evt.page_x() as f64, evt.page_y() as f64))
        });
        let ontouchstart = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => CanvasControlMsg::TouchStart((touch.page_x() as f64, touch.page_y() as f64)),
                None => CanvasControlMsg::Null,
            }
        });
        let ontouchend = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => CanvasControlMsg::TouchEnd((touch.page_x() as f64, touch.page_y() as f64)),
                None => CanvasControlMsg::Null,
            }
        });
        let ontouchmove = ctx.link().callback(move |evt: TouchEvent | {
            match evt.touches().get(0) {
                Some(touch) => CanvasControlMsg::TouchMove((touch.page_x() as f64, touch.page_y() as f64)),
                None => CanvasControlMsg::Null,
            }
        });

        html! {
            <div class="game_canvas">
                <canvas id="canvas"
                    style={"margin: 0px; width: 100vw; height: 100vh; left:0px; top:0px;"}
                    onmousedown={onmousedown}
                    onmousemove={onmousemove}
                    onmouseup={onmouseup}
                    ontouchstart={ontouchstart}
                    ontouchend={ontouchend}
                    ontouchmove={ontouchmove}
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

        gl.enable(GL::DEPTH_TEST);
        gl.enable(GL::CULL_FACE);
    

        c.set_width(self.width as u32);
        c.set_height(self.height as u32);

        self.canvas = Some(c);
        self.gl = Some(gl);

        // if first_render {
        //     self.reload();

        //     ctx.link().send_message(CanvasControlMsg::Render);
        // }
    }
}

impl CanvasControl {

    fn canvas_update(&mut self) {
        let now = instant::now();

        if self.last_update >= now {
            return;
        }
        let diff = now - self.last_update;

        let delta = diff as f64 / 1000.0; // Frac of seconds
        self.u_time += delta as f32;

        self.width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as i32;
        self.height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as i32;

        // Do updates using delta
        self.last_update = now;

        for model in self.models.iter_mut() {
            model.update(delta as f32);
        }
    }

    fn reload(&mut self) {
        // Set up shaders and 
        let gl: &GL = match &self.gl {
            Some(gl)=> gl,
            None => {
                log!("ERROR Setting up scene without a proper gl context");
                return;
            }
        };

        self.camera.setup(self.width as f32,self.height as f32);

        let _: &HtmlCanvasElement = match &self.canvas {
            Some(canv) => canv,
            None => return,
        };

        // let vertices: Vec<f32> = vec![
        //     -1.0, -1.0, 0.,
        //     1.0, -1.0, 0.,
        //     1.0, 1.0, 0.,
        //     -1.0, -1.0, 0.,
        //     -1.0, 1.0, 0.,
        //     1.0, 1.0, 0.
        // ];

        for model in self.models.iter_mut() {
            model.setup_shader(gl, self.width as f32, self.height as f32);
            model.load_textures(gl);
            model.setup(gl);
        }
        
        // // Store mouse location
        // self.mouse_x_loc = gl.get_uniform_location(&shader_program, "mouse_x");
        // gl.uniform1f(self.mouse_x_loc.as_ref() , 1.0);
        // self.mouse_y_loc = gl.get_uniform_location(&shader_program, "mouse_y");
        // gl.uniform1f(self.mouse_y_loc.as_ref() , 1.0);


        // self.shader_program = Some(shader_program);
    }

    fn render(&mut self) {
        self.canvas_update();
        
        let c = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        c.set_width(self.width as u32);
        c.set_height(self.height as u32);

        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        gl.viewport(
            0,
            0,
            self.width as i32,
            self.height as i32,
        );

        gl.clear_color(0., 0.5, 0., 1.0);
        gl.clear_depth(1.0);

        // Clear the color buffer bit
        gl.clear(GL::COLOR_BUFFER_BIT);
       
        // Update uniforms in the shaders - for now just the u_time (time since start in secs)
        // gl.uniform1f(self.time_location.as_ref() , self.u_time as f32);

        // Update the current mouse locations
        // gl.uniform1f(self.mouse_x_loc.as_ref() , self.mouse_x);
        // gl.uniform1f(self.mouse_y_loc.as_ref() , self.mouse_y);

        for model in self.models.iter_mut() {
            model.render(gl, self.u_time, &self.camera);
        }

        // gl.draw_arrays(GL::TRIANGLES, 0, self.tri_count);

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}