use gilrs::{Axis, Button, Event, EventType, Gilrs};
use raylib::prelude::*;
use std::option::Option::*;

#[derive(Debug, Clone, PartialEq)]
struct FSState {
    axis: FSAxis,
    buttons: Vec<Button>,
}

#[derive(Debug, Clone, PartialEq)]
struct FSAxis {
    x: i8,
    y: i8,
}

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let max_history = 24;
    let (mut rl, thread) = raylib::init()
        .size(960, 540)
        .title("FightStick View")
        .transparent()
        .build();
    rl.set_target_fps(60);

    let mut fs_state_history: Vec<FSState> = vec![FSState {
        axis: FSAxis { x: 0, y: 0 },
        buttons: vec![],
    }];

    // Iterate over all connected gamepads
    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    // let mut active_gamepad = None;
    // let mut curr_input_frame: usize = 0;
    while !rl.window_should_close() {
        let mut fs_state = fs_state_history.last().cloned().unwrap();

        // Examine new events
        while let Some(event) = gilrs.next_event() {
            let buttons = &mut fs_state.buttons;
            let axis = &mut fs_state.axis;

            match event {
                Event {
                    event: EventType::ButtonChanged(button, val, _),
                    ..
                } => {
                    // Controlling the axis with the dpad
                    if Button::is_dpad(button) {
                        if [Button::DPadLeft, Button::DPadRight].contains(&button) {
                            if val > 0.25 {
                                axis.x = if button == Button::DPadLeft { -1 } else { 1 };
                            } else {
                                axis.x = 0;
                            };
                        } else if val > 0.25 {
                            axis.y = if button == Button::DPadDown { -1 } else { 1 };
                        } else {
                            axis.y = 0;
                        };
                    } else if val > 0.25 && !buttons.contains(&button) {
                        buttons.push(button);
                    } else if buttons.contains(&button) {
                        buttons.retain(|b| *b != button);
                    }
                    // println!("Button {:?} state changed to {} by {}", button, val, id);
                }
                Event {
                    event: EventType::AxisChanged(ax, val, _),
                    ..
                } => {
                    if [Axis::DPadX, Axis::LeftStickX].contains(&ax) {
                        axis.x = (val.signum() * if val.abs() > 0.25 { 1. } else { 0. }) as i8;
                    } else if [Axis::DPadY, Axis::LeftStickY].contains(&ax) {
                        axis.y = (val.signum() * if val.abs() > 0.25 { 1. } else { 0. }) as i8;
                    } else if [Axis::LeftZ, Axis::RightZ].contains(&ax) {
                        if val > 0.25 {
                            if ax == Axis::LeftZ {
                                buttons.push(Button::LeftTrigger2);
                            } else if ax == Axis::RightZ {
                                buttons.push(Button::RightTrigger2);
                            }
                        } else if buttons.contains(&Button::LeftTrigger2) {
                            buttons.retain(|b| *b != Button::LeftTrigger2);
                        } else if buttons.contains(&Button::RightTrigger2) {
                            buttons.retain(|b| *b != Button::RightTrigger2);
                        }
                    }
                }
                Event {
                    event: EventType::Disconnected,
                    ..
                } => {
                    fs_state_history.clear();
                    fs_state = FSState {
                        axis: FSAxis { x: 0, y: 0 },
                        buttons: vec![],
                    };
                }
                _ => {}
            }
        }

        fs_state_history.push(fs_state);

        if fs_state_history.len() >= max_history {
            fs_state_history.remove(0);
        }
        {
            //* Raylib Drawing
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::new(0, 0, 0, 0));

            // TODO: Draw Stick

            // Draw buttons
            let radius = 56.;
            let start_btn_pos = (6 * radius as i32, (1.2 * radius) as i32);
            let sep = (radius as i32) * 2 + 8; // Separation
            for i in 0..8 {
                let center_x = start_btn_pos.0 + sep * (i / 2 + 1);
                let center_y =
                    start_btn_pos.1 + sep * (i % 2 + 1) + if i < 2 { radius as i32 } else { 0 };
                let color: Color = {
                    let btns = fs_state_history.last().cloned().unwrap().buttons;
                    if btns.contains(&Button::North) && i == 0
                        || btns.contains(&Button::South) && i == 1
                        || btns.contains(&Button::West) && i == 2
                        || btns.contains(&Button::East) && i == 3
                        || btns.contains(&Button::RightTrigger) && i == 4
                        || btns.contains(&Button::RightTrigger2) && i == 5
                        || btns.contains(&Button::LeftTrigger) && i == 6
                        || btns.contains(&Button::LeftTrigger2) && i == 7
                    {
                        Color::RED
                    } else {
                        Color::DARKBLUE
                    }
                };
                d.draw_circle(center_x, center_y, radius, color);
            }

            d.draw_text(
                &format!("{:?}", fs_state_history.last().unwrap()),
                20,
                540 - 14,
                12,
                Color::WHITE,
            );
        }
    }
}
