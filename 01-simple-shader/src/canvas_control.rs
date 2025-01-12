use web_sys::{WebGlRenderingContext as GL, WebGlProgram, HtmlCanvasElement, window};
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
    height: i32,
    width: i32,
}

pub enum CanvasControlMsg {
    MouseDown((f64, f64)),
    MouseUp((f64,f64)),
    MouseMove((f64,f64)),
    TouchStart((f64, f64)),
    TouchEnd((f64, f64)),
    TouchMove((f64, f64)),
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

        CanvasControl{
            callback: callback,
            canvas: None,
            gl: None,
            node_ref: NodeRef::default(),
            last_update: instant::now(),
            shader_program: None,
            height: height as i32,
            width: width as i32,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            CanvasControlMsg::MouseDown(evt) => {
                true
            },
            CanvasControlMsg::MouseUp(_evt) => {
                true
            },
            CanvasControlMsg::MouseMove(_evt) => {
                // log!("Event here => ", self.mousehandler.offset_x, self.mousehandler.offset_y);
                true
            },
            CanvasControlMsg::TouchStart(evt) => {
                // log!("Event here TouchStart => ", evt.0, evt.1);
                true
            },
            CanvasControlMsg::TouchEnd(_evt) => {
                // log!("Event here TouchEnd => ", evt.0, evt.1);
                true
            },
            CanvasControlMsg::TouchMove(_evt) => {
                // log!("Event here TouchMove => ", evt.0, evt.1);
                true
            },
            CanvasControlMsg::Render => {
                // log!("Render");
                self.render();
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
        let c = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let gl: GL = c
            .get_context("webgl")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        c.set_width(self.width as u32);
        c.set_height(self.height as u32);

        // self.width = c.client_width();
        // self.height = c.client_height();

        self.canvas = Some(c);
        self.gl = Some(gl);

        if first_render {
            // The callback to request animation frame is passed a time value which can be used for
            // rendering motion independent of the framerate which may vary.
            // let render_frame = self.link.callback(CanvasControlMsg::Render);
            // let handle = RenderService::request_animation_frame(render_frame);
            // window()
            //     .unwrap()
            //     .request_animation_frame(self.callback.as_ref().unchecked_ref())
            //     .unwrap();
            self.reload();

            ctx.link().send_message(CanvasControlMsg::Render);
            // A reference to the handle must be stored, otherwise it is dropped and the render won't
            // occur.
            // self.callback = Some(Box::new(handle));
        }
    }
}

impl CanvasControl {

    fn canvas_update(&mut self) {
        let now = instant::now();

        if self.last_update >= now {
            return;
        }
        let diff = now - self.last_update;

        let delta = diff as f64 / 1000.0;
        // Do updates using delta

        self.last_update += diff;
    }

  

    fn reload(&mut self) {

        let gl = match &self.gl {
            Some(gl)=> gl,
            None => {
                log!("ERROR Setting up scene without a proper gl context");
                return;
            }
        };

        let vert_code = include_str!("./basic.vert");
        let frag_code = include_str!("./basic.frag");

        let canvas: &HtmlCanvasElement = match &self.canvas {
            Some(canv) => canv,
            None => return,
        };

        let vertices: Vec<f32> = vec![
            -1.0, -1.0, 0.,
            1.0, -1.0, 0.,
            1.0, 1.0, 0.
        ];

        let vertex_buffer = self.gl.clone().unwrap().create_buffer().unwrap();
        let verts = js_sys::Float32Array::from(vertices.as_slice());

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &verts, GL::STATIC_DRAW);

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

        let canvassize = gl.get_uniform_location(&shader_program, "canvasSize");
        gl.uniform2f(canvassize.as_ref(), self.width as f32, self.height as f32);

        self.shader_program = Some(shader_program);
    }

    fn render(&mut self) {
        self.canvas_update();
        
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        // Attach the time as a uniform for the GL context.
        let time = gl.get_uniform_location(&self.shader_program.clone().unwrap(), "u_time");
        gl.uniform1f(time.as_ref(), self.last_update as f32);

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
       
        gl.draw_arrays(GL::TRIANGLES, 0, 3);

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}