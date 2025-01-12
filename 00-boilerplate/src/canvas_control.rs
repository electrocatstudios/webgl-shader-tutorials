use web_sys::{WebGlRenderingContext as GL, HtmlCanvasElement, window};

use yew::prelude::*;

use wasm_bindgen::{prelude::*, JsCast};
// use gloo_console::log;

pub struct CanvasControl {
    callback: Closure<dyn FnMut()>,
    canvas: Option<HtmlCanvasElement>,
    gl: Option<GL>,
    node_ref: NodeRef,
    last_update: f64,
    _height: f64,
    _width: f64,
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
            _height: height,
            _width: width,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool{
        match msg {
            CanvasControlMsg::MouseDown(_evt) => {
                true
            },
            CanvasControlMsg::MouseUp(_evt) => {
                true
            },
            CanvasControlMsg::MouseMove(_evt) => {
                // log!("Event here => ", self.mousehandler.offset_x, self.mousehandler.offset_y);
                true
            },
            CanvasControlMsg::TouchStart(_evt) => {
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
                    style={"margin: 0px; width: 100vw; height: 100vh, left:0px; top:0px;"}
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

        let _delta = diff as f64 / 1000.0;
        // Do updates using delta

        self.last_update += diff;
    }

    fn render(&mut self) {
        self.canvas_update();
        
        let _gl = self.gl.as_ref().expect("GL Context not initialized!");

        let canvas: &HtmlCanvasElement = match &self.canvas {
            Some(canv) => canv,
            None => return,
        };

        let width = canvas.client_width() as f64;
        let height = canvas.client_height() as f64;

        // Make sure the we reset the draw surface to prevent stretching
        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        let gl: GL = canvas
                        .get_context("webgl")
                        .expect("Error unwrapping the webgl context")
                        .unwrap()
                        .dyn_into::<GL>()
                        .unwrap(); 

        gl.viewport(
            0,
            0,
            canvas.width().try_into().unwrap(),
            canvas.height().try_into().unwrap(),
        );

        window()
            .unwrap()
            .request_animation_frame(self.callback.as_ref().unchecked_ref())
            .unwrap();
    }
}