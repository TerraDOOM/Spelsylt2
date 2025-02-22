use crate::prelude::*;
use crate::xcom::*;

pub fn spawn_geo_hud(commands: &mut Commands, context: &XcomState) {
    commands
        .spawn((
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
                |name, id| make_button(parent, name, id, &*context, Val::Px(256.0), Val::Px(64.0));

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
        .with_child((
            Text::new(text.to_string()),
            TextFont {
                font: context.assets.font.clone(),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.9)),
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
                    width: Val::Percent(50.0),
                    height: Val::Percent(30.0),
                    top: Val::Vh(0.0),
                    flex_direction: FlexDirection::Row,
                    ..default_button_node()
                })
                .with_children(|research_icon| {
                    for unlocked_technology in &context.research {
                        let icon = context.assets.icons[&unlocked_technology.id].clone();
                        make_icon(research_icon, icon, &(*context));
                    }
                });

            parent
                .spawn(Node {
                    width: Val::Percent(50.0),
                    height: Val::Percent(70.0),
                    bottom: Val::Vh(0.0),
                    flex_direction: FlexDirection::Column,
                    ..default_button_node()
                })
                .with_children(|option_box| {
                    //Make the research dynamic? TODO

                    let mut make_science_button = |name: &'static str, id| {
                        make_button(
                            option_box,
                            name,
                            id,
                            &*context,
                            Val::Percent(80.0),
                            Val::Percent(20.0),
                        );
                    };

                    make_science_button("Heavy Frame", ButtonPath::ScienceMenu);
                    make_science_button("Hover Magic", ButtonPath::ScienceMenu);
                    make_science_button("Ace Frame", ButtonPath::MainMenu);
                    make_science_button("Exit", ButtonPath::MainMenu);
                });
        },
        false,
    );
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
                    for unlocked_technology in &context.research {
                        let icon = context.assets.icons[&unlocked_technology.id].clone();
                        make_icon(production_area, icon, &(*context));
                    }
                });
        },
        false,
    );
}

trait UiExt {
    fn spawn_hud<T: Component, F>(&mut self, ctx: &XcomState, marker: T, builder: F, row: bool)
    where
        F: for<'r> FnOnce(&mut ChildBuilder<'r>);
}

impl<'a, 'b> UiExt for Commands<'a, 'b> {
    fn spawn_hud<T: Component, F>(&mut self, ctx: &XcomState, marker: T, builder: F, row: bool)
    where
        F: for<'r> FnOnce(&mut ChildBuilder<'r>),
    {
        self.spawn((
            marker,
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
            *node = ImageNode::new(context.assets.button_green.clone());
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
                    make_ship_icon(
                        ship_box,
                        context.assets.placeholder.clone(),
                        &(*context),
                        Val::Px(0.0),
                        Val::Px(0.0),
                        Slot::Front,
                    );
                    make_ship_icon(
                        ship_box,
                        context.assets.button_green.clone(),
                        &(*context),
                        Val::Px(0.0),
                        Val::Px(32.0),
                        Slot::Core1,
                    );
                    make_ship_icon(
                        ship_box,
                        context.assets.icons[&Tech::MagicBullet].clone(),
                        &(*context),
                        Val::Px(0.0),
                        Val::Px(64.0),
                        Slot::Engine,
                    );
                    make_ship_icon(
                        ship_box,
                        context.assets.icons[&Tech::MachineGun].clone(),
                        &(*context),
                        Val::Px(-96.0),
                        Val::Px(-32.0),
                        Slot::LeftWing1,
                    );
                    make_ship_icon(
                        ship_box,
                        context.assets.icons[&Tech::Rocket].clone(),
                        &(*context),
                        Val::Px(96.0),
                        Val::Px(-96.0),
                        Slot::RightWing1,
                    );
                });
            //Equipment board
            parent
                .spawn(
                    (Node {
                        width: Val::Percent(40.0),
                        height: Val::Percent(50.0),
                        left: Val::Px(128.0),
                        flex_direction: FlexDirection::Column,
                        ..default_button_node()
                    }),
                )
                .with_children(|option_box| {
                    //Only done when spawning! TODO

                    make_icon(option_box, context.assets.placeholder.clone(), &(*context));
                    option_box.spawn((
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
                    ));
                    make_icon(option_box, context.assets.placeholder.clone(), &(*context));
                    make_icon(option_box, context.assets.placeholder.clone(), &(*context));
                    make_icon(option_box, context.assets.placeholder.clone(), &(*context));
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
                        ButtonPath::MainMenu,
                        &*context,
                        Val::Percent(50.0),
                        Val::Percent(20.0),
                    );
                    make_button(
                        option_box,
                        "Exit",
                        ButtonPath::MainMenu,
                        &*context,
                        Val::Percent(50.0),
                        Val::Percent(20.0),
                    );
                });
        },
        false,
    );
}

fn make_icon(parent: &mut ChildBuilder, image_handler: Handle<Image>, context: &XcomState) {
    parent
        .spawn((
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                margin: UiRect {
                    left: Val::Px(8.0),
                    right: Val::Px(8.0),
                    top: Val::Px(8.0),
                    bottom: Val::Px(8.0),
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
