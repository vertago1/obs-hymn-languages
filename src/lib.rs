use obs_wrapper::prelude::*;
use obs_wrapper::{obs_register_module, obs_string};

struct HymnLanguageModule {
    context: ModuleContext
}

impl Module for HymnLanguageModule {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        true
    }

    fn description() -> ObsString {
        obs_string!("TODO")
    }

    fn name() -> ObsString {
        obs_string!("TODO")
    }

    fn author() -> ObsString {
        obs_string!("TODO")
    }
}

obs_register_module!(HymnLanguageModule);