use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};
use web_sys::WebGl2RenderingContext;
use web_sys::{WebGlProgram,WebGlShader};

/*fn load_shader() -> WebGLProgram
{
    let vertex_shader = 
}*/

fn start_app() {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access document");

    let canvas:HtmlCanvasElement = document.get_element_by_id("gl-canvas")
                                    .expect("Unable to find canvas ID:gl-canvas")
                                    .dyn_into::<web_sys::HtmlCanvasElement>()
                                    .expect("Element ID:gl-canvas should be a canvas!");
    
    let context_result = canvas.get_context("webgl2").expect("Unable to get webgl context!");
    
    if let Some(context_raw) = context_result
    {
        let context = context_raw.dyn_into::<WebGl2RenderingContext>().expect("Unable to process context");
        
        context.clear_color(1.0, 0.0, 1.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        
       // context.create_shader(WebGlRenderingContext::VERTEX_SHADER);
        
        //context.attach_shader(program, shader);
    }


    /*
    let text_node = document.create_text_node("Hello, world from Vanilla Rust!");
    body.append_child(text_node.as_ref())
        .expect("Failed to append text");
    */
    
}

fn main() {
    set_panic_hook();
    start_app();
}
