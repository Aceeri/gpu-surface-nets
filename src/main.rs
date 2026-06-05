
//! My learning repo for GPU programming, compute surface nets :)

use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins::default())
        .add_plugins(GpuSurfaceNets::default());

    app.run();
}
