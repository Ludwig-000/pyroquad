use std::panic;



pub fn setup_engine(){


    // basically, every time there is a runtime error on the python side, the engine will panic. Since said panic 
    // is unnecessary for the user, and only confusing to read, we filter that one out, while keeping all other panics.
	let old_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        let msg = info.to_string();
        if msg.contains("THREAD_ID.unwrap() == CURRENT_THREAD_ID") {
            // do nothing
        } else {
            // Call the original hook for everything else
            old_hook(info);
        }
    }));


    // compiles shaders
	crate::engine::SHADERS::shaderLoader::shader_load();
	
}