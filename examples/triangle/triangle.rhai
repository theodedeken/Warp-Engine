let size = [800, 600];

let vertices = [-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];

let indices = [0, 1, 2];
let count = 3;

let context = create_context("triangle");

let matter = create_matter(context, vertices, indices);

/*================ Shaders ====================*/

// Vertex shader source code
let vert_code = "#version 300 es \n in vec4 a_position; \n void main() { \n gl_Position = a_position; }";

let vert_shader = create_shader(context, vert_code, module.ShaderKind.Vertex);

//fragment shader source code
let frag_code = "#version 300 es \n precision mediump float; \n out vec4 outColor; \n void main() {    outColor = vec4(1, 0, 0.5, 1);}";
// Create fragment shader object
let frag_shader = create_shader(context, frag_code, module.ShaderKind.Fragment);

let shader_program = create_program(context, vert_shader, frag_shader);

// Use the combined shader program object
shader_program.enable();

/*======= Associating shaders to buffer objects =======*/

let matterbind = create_binding(context, shader_program, matter, "a_position")
// Enable the binding
matterbind.enable()

/*=========Drawing the triangle===========*/

// Enable the depth test
context.enable(module.Flag.DepthTest);

context.clearColor(0.5, 0.5, 0.5, 1.0);

// Set the view port
context.viewport(0, 0, size[0], size[1])


// This would ideally be in the renderloop
context.clear(module.BufferBit.Color);
context.clear(module.BufferBit.Depth);

context.drawElements(module.Primitives.Triangles, count, module.DataType.U16, 0);