use console_error_panic_hook::set_once as set_panic_hook;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};
use web_sys::WebGl2RenderingContext;
use web_sys::{WebGlProgram,WebGlShader};

const BASIC_VERTEX_SHADER_SRC:&str = "
attribute vec4 aVertexPosition;
uniform mat4 uModelViewMatrix;
uniform mat4 uProjectionMatrix;
void main() {
    gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
}
";

const BASIC_FRAGMENT_SHADER_SRC:&str = "
void main() {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
}
";

fn load_shader(context:&WebGl2RenderingContext, shader_type:u32, source: &str) 
-> Result<WebGlShader, String>
{
    let has_shader = context.create_shader(shader_type);
    
    if let Some(shader) = has_shader
    {
        context.shader_source(&shader, source);
        context.compile_shader(&shader);
        if context.get_shader_parameter(
            &shader,
            WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            return Ok(shader)
        }
    }
    Err("Unable to compile shader".to_string()) //get_shader_info_log
}

fn create_program(context:&WebGl2RenderingContext, vertex_src: &str, fragment_src: &str) 
-> Result<WebGlProgram, String>
{
    let vertex_shader = load_shader(
                                                    &context,
                                                    WebGl2RenderingContext::VERTEX_SHADER,
                                                    vertex_src
    )?;
    let fragment_shader = load_shader(
                                                        &context,
                                                        WebGl2RenderingContext::FRAGMENT_SHADER,
                                                        fragment_src
                                                        )?;
    let has_program = context.create_program();
    if let Some(program) = has_program
    {
        context.attach_shader(&program, &vertex_shader);
        context.attach_shader(&program, &fragment_shader);
    }
    
    Err("WIP".to_string())
    
}

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
        
                
        let try_program 
        = create_program(&context, BASIC_VERTEX_SHADER_SRC, BASIC_FRAGMENT_SHADER_SRC);
        
        if let Ok(program) = try_program
        {
            context.use_program(Some(&program));
            
        }
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
