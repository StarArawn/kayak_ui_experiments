use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseScrollUnit, MouseWheel},
        ButtonState,
    },
    prelude::*,
};

use crate::{
    context::{Context, CustomEventReader},
    event_dispatcher::EventDispatcher,
    input_event::InputEvent,
};

pub(crate) fn process_events(world: &mut World) {
    let window_size = if let Some(windows) = world.get_resource::<Windows>() {
        if let Some(window) = windows.get_primary() {
            Vec2::new(window.width(), window.height())
        } else {
            // log::warn!("Couldn't find primiary window!");
            return;
        }
    } else {
        // log::warn!("Couldn't find primiary window!");
        return;
    };

    let mut input_events = Vec::new();

    query_world::<
        (
            Res<Events<CursorMoved>>,
            Res<Events<MouseButtonInput>>,
            Res<Events<MouseWheel>>,
            Res<Events<ReceivedCharacter>>,
            Res<Events<KeyboardInput>>,
            ResMut<CustomEventReader<CursorMoved>>,
            ResMut<CustomEventReader<MouseButtonInput>>,
            ResMut<CustomEventReader<MouseWheel>>,
            ResMut<CustomEventReader<ReceivedCharacter>>,
            ResMut<CustomEventReader<KeyboardInput>>,
        ),
        _,
        _,
    >(
        |(
            cursor_moved_events,
            mouse_button_input_events,
            mouse_wheel_events,
            char_input_events,
            keyboard_input_events,
            mut custom_event_reader_cursor,
            mut custom_event_mouse_button,
            mut custom_event_mouse_wheel,
            mut custom_event_char_input,
            mut custom_event_keyboard,
        )| {
            if let Some(event) = custom_event_reader_cursor
                .0
                .iter(&cursor_moved_events)
                .last()
            {
                // Currently, we can only handle a single MouseMoved event at a time so everything but the last needs to be skipped
                input_events.push(InputEvent::MouseMoved((
                    event.position.x as f32,
                    window_size.y - event.position.y as f32,
                )));
            }

            for event in custom_event_mouse_button.0.iter(&mouse_button_input_events) {
                match event.button {
                    MouseButton::Left => {
                        if event.state == ButtonState::Pressed {
                            input_events.push(InputEvent::MouseLeftPress);
                        } else if event.state == ButtonState::Released {
                            input_events.push(InputEvent::MouseLeftRelease);
                        }
                    }
                    _ => {}
                }
            }

            for MouseWheel { x, y, unit } in custom_event_mouse_wheel.0.iter(&mouse_wheel_events) {
                input_events.push(InputEvent::Scroll {
                    dx: *x,
                    dy: *y,
                    is_line: matches!(unit, MouseScrollUnit::Line),
                })
            }

            for event in custom_event_char_input.0.iter(&char_input_events) {
                input_events.push(InputEvent::CharEvent { c: event.char });
            }

            for event in custom_event_keyboard.0.iter(&keyboard_input_events) {
                if let Some(key_code) = event.key_code {
                    input_events.push(InputEvent::Keyboard {
                        key: key_code,
                        is_pressed: matches!(event.state, ButtonState::Pressed),
                    });
                }
            }
        },
        world,
    );

    world.resource_scope::<EventDispatcher, _>(|world, mut event_dispatcher| {
        world.resource_scope::<Context, _>(|world, mut context| {
            event_dispatcher.process_events(input_events, &mut context, world);
        });
    });
}

fn query_world<T: bevy::ecs::system::SystemParam, F, R>(mut f: F, world: &mut World) -> R
where
    F: FnMut(<T::Fetch as bevy::ecs::system::SystemParamFetch<'_, '_>>::Item) -> R,
{
    let mut system_state = bevy::ecs::system::SystemState::<T>::new(world);
    let r = {
        let test = system_state.get_mut(world);
        f(test)
    };
    system_state.apply(world);

    r
}
