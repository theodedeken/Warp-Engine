use rhai::{Engine, RegisterFn};

pub struct RhaiEngine {
    engine: Engine,
}

impl RhaiEngine {
    pub fn new() -> RhaiEngine {
        RhaiEngine {
            engine: Engine::new(),
        }
    }
}

/*
    let context = document.getElementById("triangle").getContext("webgl2");
    let matter = module.Matter.new(context, vertices, indices);
    let vert_shader = module.Shader.new(context, vert_code, module.ShaderKind.Vertex);
    let frag_shader = module.Shader.new(context, frag_code, module.ShaderKind.Fragment);
    let shader_program = module.Program.new(context, vert_shader, frag_shader);
    shader_program.enable();
    let matterbind = module.Binding.new(context, shader_program, matter, "a_position")
    matterbind.enable()
    context.enable(module.Flag.DepthTest);
    context.clearColor(0.5, 0.5, 0.5, 1.0);
    context.viewport(0, 0, size[0], size[1])
    context.clear(module.BufferBit.Color);
    context.clear(module.BufferBit.Depth);
    context.drawElements(module.Primitives.Triangles, count, module.DataType.U16, 0);
*/

// TODO do this in a more maintainable way
use graphics::webgl::matter::Matter;

impl RhaiEngine {
    pub fn init(&mut self) {
        // Matter methods
        self.engine.register_type::<Matter>();
    }
}
