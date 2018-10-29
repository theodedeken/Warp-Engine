use rhai::{Engine, RegisterFn};

pub struct RhaiEngine {
    pub engine: Engine,
}

impl RhaiEngine {
    pub fn new() -> RhaiEngine {
        RhaiEngine {
            engine: Engine::new(),
        }
    }
}

// TODO do this in a more maintainable way
use graphics::webgl::{
    binding::Binding, context::Context, matter::Matter, program::Program, shader::Shader,
};
use webgl_rs::glenum::Flag;
use webgl_rs::rendering_context::WebGL2RenderingContext;

impl RhaiEngine {
    pub fn init(&mut self) {
        // Enums
        self.engine.register_type::<Flag>();

        // Context methods
        self.engine.register_type::<Context>();
        self.engine.register_fn("create_context", Context::new);
        self.engine.register_fn("enable", Context::enable);
        self.engine.register_fn("clear_color", Context::clear_color);
        self.engine.register_fn("viewport", Context::viewport);
        self.engine.register_fn("clear", Context::clear);
        self.engine
            .register_fn("draw_elements", Context::draw_elements);

        // Matter methods
        self.engine.register_type::<Matter>();
        self.engine.register_fn("create_matter", Matter::new);

        // Context methods
        self.engine.register_type::<WebGL2RenderingContext>();

        // Shader methods
        self.engine.register_type::<Shader>();
        self.engine.register_fn("create_shader", Shader::new);

        // Program methods
        self.engine.register_type::<Program>();
        self.engine.register_fn("create_program", Program::new);
        self.engine.register_fn("enable", Program::enable);

        // Binding methods
        self.engine.register_type::<Binding>();
        self.engine.register_fn("create_binding", Binding::new);
        self.engine.register_fn("enable", Binding::enable);

        // Get context
        self.engine.register_fn("get_context", crate::get_context);
    }
}
