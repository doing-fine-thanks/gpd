use godot::prelude::*;

mod gpd;

struct GPDext;

#[gdextension]
unsafe impl ExtensionLibrary for GPDext {}
