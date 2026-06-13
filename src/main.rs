//! My learning repo for GPU programming, compute surface nets :)

use bevy::prelude::*;
use gpu_surface_nets::simulation::*;
use gpu_surface_nets::surface_nets::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(GpuSurfaceNets::default())
        .add_plugins(SimulationPlugin::default());

    app.run();
}
