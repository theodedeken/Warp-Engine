const warp = import('../../wasm/warp_engine');

//import { Matter, Shader, ShaderKind, Program, Context, BufferKind } from "../../wasm/warp_engine";
//import { booted } from "../../wasm/warp_engine_bg";

function main(module) {
    let size = [800, 600];
    //let config = AppConfig::new ("Test", size);
    //let mut app = App::new (config);

    let vertices = new Float32Array([-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0]);

    let indices = new Uint16Array([0, 1, 2]);
    let count = 3;

    //let context = Context.new("canvas");
    let context = document.getElementById("triangle").getContext("webgl2");
    let matter = module.Matter.new(context, vertices, indices);

    /*================ Shaders ====================*/

    // Vertex shader source code
    let vert_code = `#version 300 es
 
    // an attribute is an input (in) to a vertex shader.
    // It will receive data from a buffer
    in vec4 a_position;
     
    // all shaders have a main function
    void main() {
     
      // gl_Position is a special variable a vertex shader
      // is responsible for setting
      gl_Position = a_position;
    }
    `;

    let vert_shader = module.Shader.new(context, vert_code, module.ShaderKind.Vertex);

    //fragment shader source code
    let frag_code = `#version 300 es
 
    // fragment shaders don't have a default precision so we need
    // to pick one. mediump is a good default. It means "medium precision"
    precision mediump float;
     
    // we need to declare an output for the fragment shader
    out vec4 outColor;
     
    void main() {
      // Just set the output to a constant redish-purple
      outColor = vec4(1, 0, 0.5, 1);
    }
    `;
    // Create fragment shader object
    let frag_shader = module.Shader.new(context, frag_code, module.ShaderKind.Fragment);

    let shader_program = module.Program.new(context, vert_shader, frag_shader);

    // Use the combined shader program object
    shader_program.enable();
    //gl.use_program(& shader_program);

    /*======= Associating shaders to buffer objects =======*/
    //matter.bind()

    let matterbind = module.Binding.new(context, shader_program, matter, "a_position")

    matterbind.enable()
    //program.bind("coordinates", matter)
    // Bind vertex buffer object
    //gl.bind_buffer(BufferKind:: Array, & vertex_buffer);

    // Bind index buffer object
    //gl.bind_buffer(BufferKind:: ElementArray, & index_buffer);

    //let coord = shader_program.get_attrib_location("coordinates");
    // Get the attribute location
    //let coord = gl.get_attrib_location(& shader_program, "coordinates".into())
    //.unwrap();

    // Point an attribute to the currently bound VBO
    //context.vertex_attrib_pointer(coord, AttributeSize.Three, DataType.Float, false, 0, 0);

    // Enable the attribute
    //context.enable_vertex_attrib_array(coord);

    /*=========Drawing the triangle===========*/

    // Clear the canvas
    context.clearColor(0.5, 0.5, 0.5, 0.9);

    // Enable the depth test
    context.enable(module.Flag.DepthTest);

    // Clear the color buffer bit
    context.clear(module.BufferBit.Color);
    context.clear(module.BufferBit.Depth);

    // Set the view port
    context.viewport(0, 0, size[0], size[1])
    setTimeout(() => {
        context.clear(module.BufferBit.Color);
        context.clear(module.BufferBit.Depth);
        context.clearColor(1.0, 1.0, 1.0, 1.0);
        context.drawElements(module.Primitives.Triangles, count, module.DataType.U16, 0);
    }, 50)
}


function load() {
    warp.then(module => main(module))
}

window.onload = load;