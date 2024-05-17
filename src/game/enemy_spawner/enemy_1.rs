#[derive(Component)]
pub struct Enemy1 {
    hp: i8,
}

impl Enemy1 {
    pub fn spawn(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
    ) {
        let window = window_query.single().unwrap();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                texture: asset_server.load("assets/enemy.png"),
                ..default()
            },
            Enemy1 {
                hp: 1,
            }
        ));
    }
}