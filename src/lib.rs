use std::ffi::CStr;
use std::str::Utf8Error;

use obs_wrapper::prelude::*;
use obs_wrapper::{obs_register_module, obs_string};

extern "C" {
    pub fn text_lookup_getstr(
        lookup_string: *const ::std::os::raw::c_char,
        translated_string: *mut *const ::std::os::raw::c_char,
    ) -> bool;
}

pub fn get_module_string(
    lookup_string: ObsString,
) -> Option<Result<ObsString, Utf8Error>> {
    let mut localized: *const std::os::raw::c_char = lookup_string.as_ptr();

    if unsafe {
        text_lookup_getstr(
            lookup_string.as_ptr(),
            &mut localized as *mut *const std::os::raw::c_char,
        )
    } {
        Some(unsafe {
            CStr::from_ptr(localized)
                .to_str()
                .map(|s| ObsString::from_nul_terminted_str(s))
        })
    } else {
        None
    }
}

struct HymnLanguageModule {
    context: ModuleContext,
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
        get_module_string(obs_string!("Description")).unwrap().expect("bad ini: description")
    }

    fn name() -> ObsString {
        get_module_string(obs_string!("Name")).unwrap().expect("bad ini: name")
    }

    fn author() -> ObsString {
        obs_string!("vertago1@gmail.com")
    }
}

obs_register_module!(HymnLanguageModule);
