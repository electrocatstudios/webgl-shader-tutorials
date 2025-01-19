use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL};
use yew::prelude::*;

use wasm_bindgen::{prelude::*, JsCast};

pub struct CanvasControl {
    callback: Closure<dyn FnMut()>,
    canvas: Option<HtmlCanvasElement>,
    gl: Option<GL>,
    node_ref: NodeRef,
    last_update: f64,
    u_time: f32,
    height: i32,
    width: i32,
}

pub enum CanvasControlMsg {
    Render
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

        // Get window size and use this later for sizing the canvas to full screen
        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();
        let height = window().unwrap().inner_height().unwrap().as_f64().unwrap();

        CanvasControl{
            callback: callback,
            canvas: None,
            gl: None,
            node_ref: NodeRef::default(),
            last_update: instant::now(),
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
        // Update internally stored time metrics
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
        // Double check we have a canvas - if not then return, something went wrong
        let _: &HtmlCanvasElement = match &self.canvas {
            Some(canv) => canv,
            None => return,
        };

        // Do set up here - preparing your GL Scene
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

        // Clear the color buffer bit
        gl.clear(GL::COLOR_BUFFER_BIT);
       
        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}