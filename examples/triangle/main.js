const warp = import('../../wasm/warp_engine');

//import { Matter, Shader, ShaderKind, Program, Context, BufferKind } from "../../wasm/warp_engine";
//import { booted } from "../../wasm/warp_engine_bg";

function main(module) {
  module.set_context(document.getElementById('triangle').getContext('webgl2'));
  module.startup('./triangle/triangle.rhai');

  /*let size = [800, 600];

  let vertices = new Float32Array([-0.5, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0]);

  let indices = new Uint16Array([0, 1, 2]);
  let count = 3;

  let context = document.getElementById("triangle").getContext("webgl2");

  let matter = module.Matter.new(context, vertices, indices);


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



  let matterbind = module.Binding.new(context, shader_program, matter, "a_position")
  // Enable the binding
  matterbind.enable()

  

  // Enable the depth test
  context.enable(module.Flag.DepthTest);

  context.clearColor(0.5, 0.5, 0.5, 1.0);

  // Set the view port
  context.viewport(0, 0, size[0], size[1])


  // This would ideally be in the renderloop
  context.clear(module.BufferBit.Color);
  context.clear(module.BufferBit.Depth);

  context.drawElements(module.Primitives.Triangles, count, module.DataType.U16, 0);*/
}


function load() {
  warp.then(module => main(module))
}

window.onload = load;