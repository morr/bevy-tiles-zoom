use crate::*;

expose_submodules!(components, systems);

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickTileEvent>()
            .add_event::<HoverTileEvent>()
            .add_systems(Startup, spawn_map)
            .add_systems(Update, highlight_hovered_tile);
    }
}
