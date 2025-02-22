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
                        justify_content: JustifyContent::FlexEnd,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ImageNode::new(context.assets.button_normal_big.clone()),
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

            make_button(
                parent,
                "Research".to_string(),
                ButtonPath::ScienceMenu,
                &(*context),
                Val::Px(256.0),
                Val::Px(64.0),
            );
            make_button(
                parent,
                "Production".to_string(),
                ButtonPath::ProductionMenu,
                &*context,
                Val::Px(256.0),
                Val::Px(64.0),
            );
            make_button(
                parent,
                "Save".to_string(),
                ButtonPath::MainMenu,
                &*context,
                Val::Px(256.0),
                Val::Px(64.0),
            );
            make_button(
                parent,
                "Load".to_string(),
                ButtonPath::MainMenu,
                &*context,
                Val::Px(256.0),
                Val::Px(64.0),
            );
        });
}

fn make_button(
    parent: &mut ChildBuilder,
    text: String,
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
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode::new(context.assets.button_normal.clone()),
        ))
        .with_child((
            Text::new(text),
            TextFont {
                font: context.assets.font.clone(),
                font_size: 33.0,
                ..default()
            },
            TextColor(Color::srgb(0.7, 0.7, 0.9)),
        ));
}

pub fn spawn_science_hud(commands: &mut Commands, context: &XcomState) {
    commands
        .spawn((
            ScienceScreen, //The fade backdrop. Will also be a button out
            //            BackDropFade,
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
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ImageNode::new(context.assets.backpanel.clone()),
                ))
                .with_children(|parent| {
                    //Top 30% of the screen for found research and icons
                    parent
                        .spawn(
                            (Node {
                                width: Val::Percent(50.0),
                                height: Val::Percent(30.0),
                                top: Val::Vh(0.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            }),
                        )
                        .with_children(|research_icon| {
                            for unlocked_technology in &context.research {
                                let icon = context.assets.icons[&unlocked_technology.id].clone();
                                make_icon(research_icon, icon, &(*context));
                            }
                        });

                    parent
                        .spawn(
                            (Node {
                                width: Val::Percent(50.0),
                                height: Val::Percent(70.0),
                                bottom: Val::Vh(0.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            }),
                        )
                        .with_children(|option_box| {
                            //Make the research dynamic? TODO

                            make_button(
                                option_box,
                                "Heavy Frame".to_string(),
                                ButtonPath::ScienceMenu,
                                &(*context),
                                Val::Percent(80.0),
                                Val::Percent(20.0),
                            );
                            make_button(
                                option_box,
                                "Hover Magic".to_string(),
                                ButtonPath::ScienceMenu,
                                &(*context),
                                Val::Percent(80.0),
                                Val::Percent(20.0),
                            );
                            make_button(
                                option_box,
                                "Ace Frame".to_string(),
                                ButtonPath::MainMenu,
                                &*context,
                                Val::Percent(80.0),
                                Val::Percent(20.0),
                            );
                            make_button(
                                option_box,
                                "Exit".to_string(),
                                ButtonPath::MainMenu,
                                &*context,
                                Val::Percent(80.0),
                                Val::Percent(20.0),
                            );
                        });
                });
        });
}

pub fn spawn_manufacturing_hud(commands: &mut Commands, context: &XcomState) {
    commands
        .spawn((
            ProdScreen, //The fade backdrop. Will also be a button out
            //            BackDropFade,
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
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ImageNode::new(context.assets.backpanel.clone()),
                ))
                .with_children(|parent| {
                    //Top 30% of the screen for found research and icons
                    parent.spawn((
                        (Node {
                            width: Val::Percent(50.0),
                            height: Val::Percent(30.0),
                            top: Val::Vh(5.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Row,
                            ..default()
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
                        .spawn(
                            (Node {
                                width: Val::Percent(50.0),
                                height: Val::Percent(70.0),
                                bottom: Val::Vh(0.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            }),
                        )
                        .with_children(|production_area| {
                            for unlocked_technology in &context.research {
                                let icon = context.assets.icons[&unlocked_technology.id].clone();
                                make_icon(production_area, icon, &(*context));
                            }
                        });
                });
        });
}

pub fn spawn_mission_hud(commands: &mut Commands, context: &XcomState) {
    commands
        .spawn((
            MissionScreen, //The fade backdrop. Will also be a button out
            //            BackDropFade,
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
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ImageNode::new(context.assets.backpanel.clone()),
                ))
                .with_children(|parent| {
                    //Top 30% of the screen for found research and icons
                    parent
                        .spawn(
                            (Node {
                                width: Val::Px(256.0),
                                height: Val::Percent(100.0),
                                top: Val::Vh(0.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            }),
                        )
                        .with_children(|ship_box| {
                            make_ship_icon(
                                ship_box,
                                context.assets.button_green.clone(),
                                &(*context),
                                Val::Px(20.0),
                                Val::Px(20.0),
                            );
                            make_ship_icon(
                                ship_box,
                                context.assets.button_green.clone(),
                                &(*context),
                                Val::Px(210.0),
                                Val::Px(20.0),
                            );
                        });

                    parent
                        .spawn(
                            (Node {
                                width: Val::Percent(50.0),
                                height: Val::Percent(70.0),
                                bottom: Val::Vh(0.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Column,
                                ..default()
                            }),
                        )
                        .with_children(|option_box| {
                            //Make the research dynamic? TODO

                            make_button(
                                option_box,
                                "Exit".to_string(),
                                ButtonPath::MainMenu,
                                &*context,
                                Val::Percent(80.0),
                                Val::Percent(20.0),
                            );
                        });
                });
        });
}

fn make_icon(parent: &mut ChildBuilder, image_handler: Handle<Image>, context: &XcomState) {
    parent
        .spawn((
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Px(8.0),
                    right: Val::Px(8.0),
                    top: Val::Px(8.0),
                    bottom: Val::Px(8.0),
                },
                ..default()
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
) {
    parent
        .spawn((
            Node {
                width: Val::Px(64.0),
                height: Val::Px(64.0),
                left: x,
                bottom: y,
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                margin: UiRect {
                    left: Val::Px(8.0),
                    right: Val::Px(8.0),
                    top: Val::Px(8.0),
                    bottom: Val::Px(8.0),
                },
                ..default()
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
