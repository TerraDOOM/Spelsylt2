use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::picking::focus::HoverMap;

use crate::prelude::*;
use crate::xcom::*;

pub fn spawn_geo_hud(commands: &mut Commands, context: &XcomState) {
    commands
        .spawn((
            XcomObject,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                right: Val::Vw(0.0),
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ZIndex(2),
        ))
        .with_children(|parent| {
            parent //The clock button
                .spawn((
                    Node {
                        width: Val::Px(256.0),
                        height: Val::Px(256.0),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ImageNode::new(context.assets.button_normal_big.clone()),
                    Clock,
                ))
                .with_child((
                    Text::new("1985\nApr 5th\n10:49"),
                    TextFont {
                        font: context.assets.font.clone(),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.9)),
                ));

            let mut make_geo_button =
                |name, id| make_button(parent, name, id, context, Val::Px(256.0), Val::Px(64.0));

            make_geo_button("Research", ButtonPath::ScienceMenu);
            make_geo_button("Production", ButtonPath::ProductionMenu);
            make_geo_button("Save", ButtonPath::MainMenu);
            make_geo_button("Load", ButtonPath::MainMenu);
        });
}

fn make_button(
    parent: &mut ChildBuilder,
    text: &str,
    link_id: ButtonPath,
    context: &XcomState,
    width: Val,
    height: Val,
) {
    parent
        .spawn((
            Button,
            ButtonLink(link_id),
            Node {
                width,
                height,
                ..default_button_node()
            },
            ImageNode::new(context.assets.button_normal.clone()),
        ))
        .insert(PickingBehavior {
            should_block_lower: false,
            ..default()
        })
        .with_child((
            Text::new(text.to_string()),
            TextFont {
                font: context.assets.font.clone(),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.9)),
            PickingBehavior {
                should_block_lower: false,
                ..default()
            },
        ));
}

fn make_science_button(parent: &mut ChildBuilder, research: &Research, context: &XcomState) {
    parent
        .spawn((
            Button,
            ButtonLink(ButtonPath::StartResearch),
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                ..default_button_node()
            },
            ScienceSelect(research.id),
            ImageNode::new(context.assets.button_normal.clone()),
        ))
        .insert(PickingBehavior {
            should_block_lower: false,
            ..default()
        })
        .with_child((
            Text::new(research.name.clone()),
            TextFont {
                font: context.assets.font.clone(),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.9)),
            PickingBehavior {
                should_block_lower: false,
                ..default()
            },
        ));
}

pub fn spawn_science_hud(commands: &mut Commands, context: &XcomState) {
    commands.spawn_hud(
        context,
        ScienceScreen, //The fade backdrop. Will also be a button out
        |parent| {
            //Top 30% of the screen for found research and icons
            parent
                .spawn(Node {
                    width: Val::Percent(30.0),
                    height: Val::Percent(100.0),
                    top: Val::Vh(0.0),
                    flex_direction: FlexDirection::Column,
                    ..default_button_node()
                })
                .with_children(|research_icon| {
                    for unlocked_technology in &context.finished_research {
                        let icon = context.assets.icons[&unlocked_technology.id].clone();
                        make_icon(research_icon, icon, context);
                    }
                });

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    height: Val::Percent(25.),
                    width: Val::Percent(75.),
                    top: Val::Percent(8.),
                    ..default()
                })
                .with_child((
                    Text::new("Currently researching X"),
                    CurrentResearch,
                    TextFont {
                        font: context.assets.font.clone(),
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.7, 0.7, 0.9)),
                ));

            parent
                .spawn(Node {
                    top: Val::Percent(30.0),
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Stretch,
                    height: Val::Percent(60.),
                    right: Val::Percent(5.),
                    width: Val::Percent(60.0),
                    overflow: Overflow::scroll_y(),
                    ..default()
                })
                .with_children(|option_box| {
                    option_box
                        .spawn(Node {
                            //Scientist text
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::Stretch,
                            ..default()
                        })
                        .with_child((
                            Text::new("Scientist"),
                            ScientistDisplay,
                            TextFont {
                                font: context.assets.font.clone(),
                                font_size: 33.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.7, 0.7, 0.9)),
                        ));

                    for potential_research in &context.possible_research {
                        let mut fine = true;
                        for prereqisite in potential_research.prerequisites.clone() {
                            if !(context
                                .finished_research
                                .iter()
                                .find(|n| n.id == prereqisite)
                                .is_some())
                            {
                                fine = false;
                            }
                        }
                        if fine {
                            make_science_button(option_box, potential_research, context);
                        }
                    }
                    /*make_science_button("Hover Magic1", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic2", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic3", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic4", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic5", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic6", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic7", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic8", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic9", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic7", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic8", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic9", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic7", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic8", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic9", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic10", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic11", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic12", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic13", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic14", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic15", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic16", ButtonPath::ScienceMenu);
                    make_science_button("Ace Frame", ButtonPath::MainMenu);*/
                    make_button(
                        option_box,
                        "Exit",
                        ButtonPath::MainMenu,
                        context,
                        Val::Percent(100.),
                        Val::Px(128.),
                    );
                });
        },
        false,
    );
}

pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (mouse_wheel_event.x * 21.0, mouse_wheel_event.y * 21.0),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}

pub fn spawn_manufacturing_hud(commands: &mut Commands, context: &XcomState) {
    commands.spawn_hud(
        context,
        ProdScreen,
        |parent| {
            //Top 30% of the screen for found research and icons
            parent.spawn((
                (Node {
                    width: Val::Percent(50.0),
                    height: Val::Percent(30.0),
                    top: Val::Vh(5.0),
                    flex_direction: FlexDirection::Row,
                    ..default_button_node()
                }),
                Text::new("Producing: "),
                TextFont {
                    font: context.assets.font.clone(),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            parent
                .spawn(Node {
                    width: Val::Percent(50.0),
                    height: Val::Percent(70.0),
                    bottom: Val::Vh(0.0),
                    flex_direction: FlexDirection::Column,
                    ..default_button_node()
                })
                .with_children(|production_area| {
                    for unlocked_technology in &context.finished_research {
                        let icon = context.assets.icons[&unlocked_technology.id].clone();
                        make_icon(production_area, icon, context);
                    }
                });
        },
        false,
    );
}

pub fn spawn_notice_hud(commands: &mut Commands, context: &XcomState) {
    commands.spawn_hud(
        context,
        NoticeScreen,
        |parent| {
            parent.spawn((
                TitleNode,
                Node {
                    width: Val::Percent(80.0),
                    left: Val::Percent(10.),
                    height: Val::Percent(20.0),
                    top: Val::Vh(5.0),
                    flex_direction: FlexDirection::Row,
                    ..default_button_node()
                },
                Text::new("Notice Title"),
                TextFont {
                    font: context.assets.font.clone(),
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            parent.spawn((
                YappNode,
                Node {
                    width: Val::Percent(80.0),
                    left: Val::Percent(10.),
                    height: Val::Percent(60.0),
                    flex_direction: FlexDirection::Column,
                    ..default_button_node()
                },
                Text::new("Notice text that keeps on yapping. Like damn bro keep on talking that Lorem Ipsum"),
                TextFont {
                    font: context.assets.font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
            make_button(
                parent,
                "Exit",
                ButtonPath::MainMenu,
                context,
                Val::Percent(100.),
                Val::Px(128.),
            );
        },
        true,
    );
}

trait UiExt {
    fn spawn_hud<T: Component, F>(&mut self, ctx: &XcomState, marker: T, builder: F, row: bool)
    where
        F: for<'r> FnOnce(&mut ChildBuilder<'r>);
}

impl UiExt for Commands<'_, '_> {
    fn spawn_hud<T: Component, F>(&mut self, ctx: &XcomState, marker: T, builder: F, row: bool)
    where
        F: for<'r> FnOnce(&mut ChildBuilder<'r>),
    {
        self.spawn((
            marker,
            XcomObject,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                right: Val::Vw(0.0),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            ZIndex(3),
        ))
        .with_children(|backdrop| {
            backdrop
                .spawn((
                    Node {
                        width: Val::Vw(80.0),
                        height: Val::Vh(80.0),
                        left: Val::Vw(10.0),
                        top: Val::Vh(10.0),
                        flex_direction: (if (row) {
                            FlexDirection::Column
                        } else {
                            FlexDirection::Row
                        }),
                        ..default()
                    },
                    ImageNode::new(ctx.assets.backpanel.clone()),
                ))
                .with_children(builder);
        });
    }
}

fn default_button_node() -> Node {
    Node {
        // horizontally center child text
        justify_content: JustifyContent::Center,
        // vertically center child text
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn unequip_loadout(
    mut context: ResMut<XcomState>,
    mut interaction_query: Query<
        (&Interaction, &ShipComponent, &mut Children),
        (Changed<Interaction>, With<ShipComponent>),
    >,
    mut image_query: Query<&mut ImageNode>,
) {
    for (interaction, component, mut children) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            dbg!(*interaction);
            context.loadout.insert(component.0, None);
            let mut node = image_query.get_mut(children[0]).unwrap();

            *node = match **component {
                (Slot::Core1 | Slot::Engine) => {
                    ImageNode::new(context.assets.button_equip_alt.clone())
                }
                _ => ImageNode::new(context.assets.button_equip.clone()),
            }
        }
    }
}

pub fn equip_loadout(
    mut context: ResMut<XcomState>,
    mut interaction_query: Query<(&Interaction, &Equipment), (Changed<Interaction>)>,
    mut loadout_query: Query<(&ShipComponent, &mut Children)>,
    mut image_query: Query<&mut ImageNode>,
    mut pressed_already: Local<bool>,
) {
    for (interaction, equipment) in &mut interaction_query {
        println!("About to equip");
        if *interaction == Interaction::Pressed {
            if !*pressed_already {
                *pressed_already = true;
                dbg!(*interaction);
                let XcomState {
                    assets, loadout, ..
                } = &mut *context;
                for (key, value) in loadout {
                    if value.is_none() {
                        println!("Equip!");
                        *value = Some(equipment.0);

                        //We must find the box to fill from key
                        for (component, mut children) in &mut loadout_query {
                            if component.0 == *key {
                                let mut node = image_query.get_mut(children[0]).unwrap();
                                *node = ImageNode::new(assets.icons[&equipment.0].clone());
                                return;
                            }
                        }
                    }
                }
            } else {
                continue;
            }
        } else {
            *pressed_already = false;
        }
    }
}

pub fn spawn_mission_hud(commands: &mut Commands, context: &XcomState) {
    commands.spawn_hud(
        context,
        MissionScreen,
        |parent| {
            //Top 30% of the screen for found research and icons

            let center: f32 = 40.0;
            parent
                .spawn((
                    Node {
                        width: Val::Px(512.0),
                        height: Val::Px(512.0),
                        top: Val::Vh(0.0),
                        flex_direction: FlexDirection::Column,
                        ..default_button_node()
                    },
                    ImageNode::new(context.assets.loadout.clone()),
                ))
                .with_children(|ship_box| {
                    //An if only for the assets. I wanna die..
                    if let Some(tech) = context.loadout[&Slot::Front] {
                        make_ship_icon(
                            ship_box,
                            context.assets.icons[&tech].clone(),
                            context,
                            Val::Px(0.0),
                            Val::Px(0.0),
                            Slot::Front,
                        );
                    } else {
                        make_ship_icon(
                            ship_box,
                            context.assets.button_equip.clone(),
                            context,
                            Val::Px(0.0),
                            Val::Px(0.0),
                            Slot::Front,
                        );
                    }

                    if let Some(tech) = context.loadout[&Slot::Core1] {
                        make_ship_icon(
                            ship_box,
                            context.assets.icons[&tech].clone(),
                            context,
                            Val::Px(0.0),
                            Val::Px(32.0),
                            Slot::Core1,
                        );
                    } else {
                        make_ship_icon(
                            ship_box,
                            context.assets.button_equip_alt.clone(),
                            context,
                            Val::Px(0.0),
                            Val::Px(32.0),
                            Slot::Core1,
                        );
                    }

                    if let Some(tech) = context.loadout[&Slot::Engine] {
                        make_ship_icon(
                            ship_box,
                            context.assets.icons[&tech].clone(),
                            context,
                            Val::Px(0.0),
                            Val::Px(64.0),
                            Slot::Engine,
                        );
                    } else {
                        make_ship_icon(
                            ship_box,
                            context.assets.button_equip_alt.clone(),
                            context,
                            Val::Px(0.0),
                            Val::Px(64.0),
                            Slot::Engine,
                        );
                    }

                    if let Some(tech) = context.loadout[&Slot::LeftWing1] {
                        make_ship_icon(
                            ship_box,
                            context.assets.icons[&tech].clone(),
                            context,
                            Val::Px(-96.0),
                            Val::Px(-32.0),
                            Slot::LeftWing1,
                        );
                    } else {
                        make_ship_icon(
                            ship_box,
                            context.assets.button_equip.clone(),
                            context,
                            Val::Px(-96.0),
                            Val::Px(-32.0),
                            Slot::LeftWing1,
                        );
                    }

                    if let Some(tech) = context.loadout[&Slot::RightWing1] {
                        make_ship_icon(
                            ship_box,
                            context.assets.icons[&tech].clone(),
                            context,
                            Val::Px(96.0),
                            Val::Px(-96.0),
                            Slot::RightWing1,
                        );
                    } else {
                        make_ship_icon(
                            ship_box,
                            context.assets.button_equip.clone(),
                            context,
                            Val::Px(96.0),
                            Val::Px(-96.0),
                            Slot::RightWing1,
                        );
                    }
                });
            //Equipment board
            parent
                .spawn(
                    (Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(50.0),
                        left: Val::Px(128.0),
                        flex_direction: FlexDirection::Column,
                        align_self: AlignSelf::Stretch,
                        overflow: Overflow::scroll_y(),

                        ..default_button_node()
                    }),
                )
                .insert(PickingBehavior {
                    should_block_lower: false,
                    ..default()
                })
                .with_children(|option_box| {
                    //Only done when spawning! TODO
                    //                    make_icon(option_box, context.assets.placeholder.clone(), &(*context));
                    /*                    option_box.spawn((
                        Node {
                            top: Val::Px(-64.0),
                            left: Val::Px(64.0),
                            ..default()
                        },
                        Text::new("x5"),
                        TextFont {
                            font: context.assets.font.clone(),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.7, 0.7, 0.9)),
                        PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        },
                    ));*/

                    for research in context.finished_research.clone() {
                        if research.equipable {
                            make_equipment(option_box, context, Equipment(research.id));
                        }
                    }
                });
            parent
                .spawn(
                    (Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(25.0),
                        top: Val::Percent(50.0),
                        left: -Val::Percent(12.5),
                        flex_direction: FlexDirection::Row,
                        ..default_button_node()
                    }),
                )
                .with_children(|option_box| {
                    make_button(
                        option_box,
                        "Start mission",
                        ButtonPath::StartMission,
                        context,
                        Val::Percent(50.0),
                        Val::Percent(20.0),
                    );
                    make_button(
                        option_box,
                        "Exit",
                        ButtonPath::MainMenu,
                        context,
                        Val::Percent(50.0),
                        Val::Percent(20.0),
                    );
                });
        },
        false,
    );
}

fn make_equipment(parent: &mut ChildBuilder, context: &XcomState, equipment_type: Equipment) {
    parent
        .spawn((
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                ..default_button_node()
            },
            Button,
            equipment_type.clone(),
            ImageNode::new(context.assets.button_equip.clone()),
        ))
        .with_child((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(context.assets.icons[&equipment_type.0].clone()),
        ));
}

fn make_icon(parent: &mut ChildBuilder, image_handler: Handle<Image>, context: &XcomState) {
    parent
        .spawn((
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                margin: UiRect {
                    left: Val::Px(2.0),
                    right: Val::Px(2.0),
                    top: Val::Px(2.0),
                    bottom: Val::Px(2.0),
                },
                ..default_button_node()
            },
            ImageNode::new(context.assets.button_green.clone()),
        ))
        .with_child((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(image_handler),
        ));
}

fn make_ship_icon(
    parent: &mut ChildBuilder,
    image_handler: Handle<Image>,
    context: &XcomState,
    x: Val,
    y: Val,
    slot: Slot,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                left: x,
                top: y,
                ..default()
            },
            ShipComponent(slot),
            ImageNode::new(context.assets.button_green.clone()),
        ))
        .with_child((
            LoadoutIcon,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ImageNode::new(image_handler),
        ));
}
