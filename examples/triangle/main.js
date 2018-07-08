import { Matter, Shader, ShaderKind, Program, Context, BufferKind } from "../../wasm/warp_engine";
import { booted } from "../../wasm/warp_engine_bg";

function main() {
    let size = [800, 600];
    //let config = AppConfig::new ("Test", size);
    //let mut app = App::new (config);

    let vertices = Float32Array([-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0]);

    let indices = Uint16Array([0, 1, 2]);
    let count = 3;

    let context = Context.new("canvas");
    let matter = Matter.new(context, vertices, indices);

    /*================ Shaders ====================*/

    // Vertex shader source code
    let vert_code = "attribute vec3 coordinates;void main(void){gl_Position = vec4(coordinates, 1.0);}";

    let vert_shader = Shader.new(context, vert_code, ShaderKind.Vertex);

    //fragment shader source code
    let frag_code = "void main(void){gl_FragColor = vec4(1, 0.5, 0.0, 1);}";
    // Create fragment shader object
    let frag_shader = Shader.new(context, frag_code, ShaderKind.Fragment);

    let shader_program = Program.new(context, vert_shader, frag_shader);

    // Use the combined shader program object
    shader_program.use();
    //gl.use_program(& shader_program);

    /*======= Associating shaders to buffer objects =======*/
    //matter.bind()
    let binding = Binding.new(context, program, matter, "coordinates");
    matter.bind()
    binding.enable()
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
    context.clear_color(0.5, 0.5, 0.5, 0.9);

    // Enable the depth test
    context.enable(Flag.DepthTest);

    // Clear the color buffer bit
    context.clear(BufferBit.Color);
    context.clear(BufferBit.Depth);

    // Set the view port
    context.viewport(0, 0, size[0], size[1])

    while (true) {
        context.clear(BufferBit.Color);
        context.clear(BufferBit.Depth);
        context.clear_color(1.0, 1.0, 1.0, 1.0);
        context.draw_elements(Primitives.Triangles, count, DataType.U16, 0);
    }

}
booted.then(main);