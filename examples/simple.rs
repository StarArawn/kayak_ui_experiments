use bevy::{prelude::*};
use kayak_ui::prelude::*;

#[derive(Component, Default)]
pub struct MyWidget2 {
    bar: u32,
}

fn my_widget_2_update(
    In((_widget_tree, entity)): In<(WidgetTree, Entity)>,
    query: Query<&MyWidget2, Or<(Added<MyWidget2>, Changed<MyWidget2>)>>,
) -> bool  {
    if let Ok(my_widget2) = query.get(entity) {
        dbg!(my_widget2.bar);
    }

    true
}

impl Widget for MyWidget2 {}

#[derive(Component, Default)]
pub struct MyWidget {
    pub foo: u32,
}

fn my_widget_1_update(
    In((widget_tree, entity)): In<(WidgetTree, Entity)>,
    mut commands: Commands,
    my_resource: Res<MyResource>,
    mut query: Query<&mut MyWidget>,
) -> bool {
    if my_resource.is_changed() {
        if let Ok(mut my_widget) = query.get_mut(entity) {
            my_widget.foo = my_resource.0;
            dbg!(my_widget.foo);

            let my_child = MyWidget2 {
                bar: my_widget.foo,
            };
            let should_update = my_widget.foo == my_child.bar;
            let child_id = commands.spawn().insert(my_child).id();
            widget_tree.add::<MyWidget2>(child_id, Some(entity));

            return should_update;
        }
    }

    false
}

impl Widget for MyWidget {}

pub struct MyResource(pub u32);

fn startup(mut commands: Commands) {
    let mut context = Context::new();
    context.register_widget_system(MyWidget::default().get_name(), my_widget_1_update);
    context.register_widget_system(MyWidget2::default().get_name(), my_widget_2_update);
    let entity = commands.spawn().insert(MyWidget { foo: 0 }).id();
    context.add_widget::<MyWidget>(None, entity);
    commands.insert_resource(Some(context));
}

fn update_resource(keyboard_input: Res<Input<KeyCode>>, mut my_resource: ResMut<MyResource>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        my_resource.0 += 1;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ContextPlugin)
        .insert_resource(MyResource(1))
        .add_startup_system(startup)
        .add_system(update_resource)
        .run()
}
