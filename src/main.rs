mod ui;

use bevy::prelude::*;
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use bevy_render::mesh::Mesh;
use bevy_terrain::terrain_material::add_terrain_material;
use bevy_terrain::{
    gizmo::add_axis_gizmo, terrain::terrain_example, terrain_material::TerrainMaterial,
};
use bevy_terrain::{
    terrain_common::{Terrain, TerrainImageLoadOptions, TerrainMeshResource},
    terrain_rtin::{rtin_load_terrain, RtinParams},
};
use ui::{button_system, setup_ui, show_ui_system, update_terrain_system, ButtonMaterials};

use bevy::render::{
    pipeline::{PipelineDescriptor, RenderPipeline},
    render_graph::RenderGraph,
};

fn main() {
    terrain_example();

    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FlyCameraPlugin)
        .add_asset::<TerrainMaterial>()
        .init_resource::<ButtonMaterials>()
        .init_resource::<TerrainMeshResource>()
        .init_resource::<RtinParams>()
        .add_startup_system(setup.system())
        .add_system(button_system.system())
        .add_system(update_terrain_system.system())
        .add_system(show_ui_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
    mut rtin_params: ResMut<RtinParams>,
    mut terrain_mesh_res: ResMut<TerrainMeshResource>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let image_filename = "terrain.png";

    rtin_params.error_threshold = 0.2;
    rtin_params.load_options = TerrainImageLoadOptions {
        max_image_height: 20f32,
        pixel_side_length: 1f32,
    };

    let (terrain_shaded_mesh, terrain_wireframe_mesh) =
        rtin_load_terrain(image_filename, &rtin_params);

    let terrain_shaded_mesh_handle = meshes.add(terrain_shaded_mesh);
    let terrain_wireframe_mesh_handle = meshes.add(terrain_wireframe_mesh);

    terrain_mesh_res.shaded = terrain_shaded_mesh_handle;
    terrain_mesh_res.wireframe = terrain_wireframe_mesh_handle;

    let pipeline_handle = add_terrain_material(pipelines, shaders, render_graph);

    commands
        .spawn_bundle(MeshBundle {
            mesh: terrain_mesh_res.shaded.clone(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(Terrain {});
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 4.0, 0.0)),
        ..Default::default()
    });
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 20.0, 0.0))
                .looking_at(Vec3::default(), Vec3::Y),
            ..Default::default()
        })
        .insert(FlyCamera {
            pitch: 180.0,
            ..Default::default()
        });

    // add_axis_gizmo(commands, meshes, materials,
    //     Transform::from_translation(Vec3::new(0f32, 0f32, 0f32)));

    setup_ui(
        &mut commands,
        asset_server,
        color_materials,
        button_materials,
        rtin_params,
    );
}
