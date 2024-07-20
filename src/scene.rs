use bevy::prelude::*;
use bevy_ratatui_render::RatatuiRenderContext;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup_scene_system)
        .add_systems(Update, rotate_cube_system);
}

#[derive(Component)]
struct Cube;

fn setup_scene_system(
    mut commands: Commands,
    ratatui_render: Res<RatatuiRenderContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., -5.).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            target: ratatui_render.target("main").unwrap_or_default(),
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0., 5., 0.),
        point_light: PointLight {
            intensity: 100_000.,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Cube,
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1., 1., 1.)),
            material: materials.add(StandardMaterial {
                ior: 1.52,
                thickness: 0.2,
                specular_transmission: 0.9,
                base_color: Color::hsl(187., 0.4, 0.7),
                ..default()
            }),
            ..default()
        },
    ));
}

fn rotate_cube_system(mut cube: Query<&mut Transform, With<Cube>>, time: Res<Time>) {
    let mut transform = cube.single_mut();
    transform.rotate_y(time.delta_seconds());
}
