use bevy::prelude::*;

use crate::main_menu::components::*;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn();
    }
}


pub fn build_main_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let main_menu_entity = commands.spawn(
        (
            NodeBundle {
            style: Style{
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
            },
            MainMenu,
        )
    )
    .with_children(|parent| {
        // Title

        // Play Button
        parent.spawn(
            (
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                    },
                    background_color: Color::GREEN.into(),
                    ..default()
                },
                PlayButton,
            )
        ).with_children(|parent|
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Play".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        },
                    ],
                    alignment: TextAlignment::Center,
                },
                ..default()
                }
            )
        )
        // Quit Button
    }
    )
    .id();

    main_menu_entity
}