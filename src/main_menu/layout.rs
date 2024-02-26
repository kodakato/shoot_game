use bevy::prelude::*;

use crate::main_menu::components::*;

pub fn spawn_main_menu(mut commands: Commands) {
    build_main_menu(&mut commands);
}

pub fn despawn_main_menu(
    mut commands: Commands,
    query: Query<(Entity, &Children), With<MainMenu>>,
) {
    for (entity, children) in query.iter() {
        for &child in children.iter() {
            commands.entity(child).despawn_recursive();
        }
        commands.entity(entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                background_color: Color::RED.into(),
                ..default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(400.0),
                        height: Val::Px(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        //Text
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "Space Shooter".to_string(),
                                    style: TextStyle {
                                        font_size: 60.0,
                                        color: Color::WHITE,
                                        ..default()
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        },
                    );
                });

            // Play Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::GREEN.into(),
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Play".to_string(),
                                style: TextStyle {
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            }],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            // Quit Button
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::GREEN.into(),
                        ..default()
                    },
                    QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "Quit".to_string(),
                                style: TextStyle {
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
                            }],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });
        })
        .id();

    main_menu_entity
}

