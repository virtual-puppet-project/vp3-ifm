use gdnative::prelude::*;
mod ifm;
mod ifm_data;


fn init(handle: InitHandle) {
    handle.add_class::<ifm::Ifacialmocap>();
}

godot_init!(init);