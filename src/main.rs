use gilrs::{Axis, Button, Event, EventType, Gilrs};
use raylib::prelude::*;
use std::option::Option::*;

#[derive(Debug, Clone, PartialEq)]
struct FSState {
    axis: Vector2,
    buttons: Vec<Button>,
}

enum FSButton {
    West = 0,
    South = 1,
    North = 2,
    East = 3,
    RightTrigger = 4,
    RightTrigger2 = 5,
    LeftTrigger = 6,
    LeftTrigger2 = 7,
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
        axis: Vector2 { x: 0.0, y: 0.0 },
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
                } => handle_buttons(buttons, axis, button, val),
                Event {
                    event: EventType::AxisChanged(ax, val, _),
                    ..
                } => handle_axis(buttons, axis, ax, val),
                Event {
                    event: EventType::Disconnected,
                    ..
                } => {
                    fs_state_history.clear();
                    fs_state = FSState {
                        axis: Vector2 { x: 0., y: 0. },
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

        //* Raylib Drawing
        let mut d = rl.begin_drawing(&thread);
        draw_fightstick(&mut d, &fs_state_history);
        drop(d);
    }
}

fn handle_buttons(buttons: &mut Vec<Button>, axis: &mut Vector2, button: Button, val: f32) {
    // Controlling the axis with the dpad
    if Button::is_dpad(button) {
        if [Button::DPadLeft, Button::DPadRight].contains(&button) {
            if val > 0.25 {
                axis.x = if button == Button::DPadLeft {
                    -1.0
                } else {
                    1.0
                };
            } else {
                axis.x = 0.0;
            };
        } else if val > 0.25 {
            axis.y = if button == Button::DPadDown {
                -1.0
            } else {
                1.0
            };
        } else {
            axis.y = 0.0;
        };
    } else if val > 0.25 && !buttons.contains(&button) {
        buttons.push(button);
    } else if buttons.contains(&button) {
        buttons.retain(|b| *b != button);
    }
    // println!("Button {:?} state changed to {} by {}", button, val, id);
}

fn handle_axis(buttons: &mut Vec<Button>, axis: &mut Vector2, ax: Axis, val: f32) {
    if [Axis::DPadX, Axis::LeftStickX].contains(&ax) {
        axis.x = val.signum() * if val.abs() > 0.25 { 1. } else { 0. };
    } else if [Axis::DPadY, Axis::LeftStickY].contains(&ax) {
        axis.y = val.signum() * if val.abs() > 0.25 { 1. } else { 0. };
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

fn draw_fightstick(d: &mut RaylibDrawHandle, fs_state_history: &Vec<FSState>) {
    d.clear_background(Color::new(0, 0, 0, 0));
    let radius = 56.;
    let sep = (radius as i32) * 2 + 8; // Separation
    let FSState {
        axis: stick,
        buttons: btns,
    } = fs_state_history.last().cloned().unwrap();

    {
        //* Draw Stick
        // let i = max_history;
        // stick_inputs.iter().rev().collect();
        // for i in (0..max_history).rev()

        let sep = (sep as f32 * 0.75) as i32;
        // let stick = curr_state.axis;
        let center_pos = Vector2 {
            x: 3.5 * radius,
            y: 5.2 * radius,
        };
        let stick_inputs = fs_state_history
            .iter()
            .map(|s| Vector2 {
                x: center_pos.x + (s.axis.x * sep as f32),
                y: center_pos.y + (-s.axis.y * sep as f32),
            })
            .rev()
            .collect::<Vec<Vector2>>();

        {
            let mut i: usize = 0;
            while i < stick_inputs.len() - 1 {
                d.draw_line_ex(stick_inputs[i], stick_inputs[i + 1], 4.0, Color::GOLD);
                i += 1;
            }
        }
        // d.draw_line_strip(stick_inputs.as_slice(), Color::GOLD);
        d.draw_rectangle_rounded_lines(
            Rectangle {
                x: center_pos.x - (sep) as f32,
                y: center_pos.y - (sep) as f32,
                width: (sep * 2) as f32,
                height: (sep * 2) as f32,
            },
            0.25,
            16,
            2,
            Color::LIGHTGRAY,
        );

        let color = if stick.x != 0.0 || stick.y != 0.0 {
            Color::RED
        } else {
            Color::LIGHTGRAY
        };
        d.draw_circle(
            center_pos.x as i32 + (sep * stick.x as i32),
            center_pos.y as i32 + (sep * -stick.y as i32),
            radius,
            color,
        );
    }

    {
        //* Draw button Layout
        let start_btn_pos = (6 * radius as i32, (1.2 * radius) as i32);
        for i in 0..8 {
            let center_x = start_btn_pos.0 + sep * (i / 2 + 1);
            let center_y =
                start_btn_pos.1 + sep * (i % 2 + 1) + if i < 2 { radius as i32 } else { 0 };
            let color: Color = {
                // let btns = curr_state.buttons;
                if btns.contains(&Button::West) && i == FSButton::West as i32
                    || btns.contains(&Button::South) && i == FSButton::South as i32
                    || btns.contains(&Button::North) && i == FSButton::North as i32
                    || btns.contains(&Button::East) && i == FSButton::East as i32
                    || btns.contains(&Button::RightTrigger) && i == FSButton::RightTrigger as i32
                    || btns.contains(&Button::RightTrigger2) && i == FSButton::RightTrigger2 as i32
                    || btns.contains(&Button::LeftTrigger) && i == FSButton::LeftTrigger as i32
                    || btns.contains(&Button::LeftTrigger2) && i == FSButton::LeftTrigger2 as i32
                {
                    Color::RED
                } else {
                    Color::DARKBLUE
                }
            };
            d.draw_circle(center_x, center_y, radius, color);
        }
    }

    d.draw_text(
        &format!("{:?}", fs_state_history.last().unwrap()),
        20,
        540 - 14,
        12,
        Color::WHITE,
    );
}

// (1 - t) * v0 + t * v1
fn lerp_color(c_start: Color, c_end: Color, amount: f32) -> Color {
    let r = ((1.0 - amount) * c_start.r as f32 + amount * c_end.r as f32) as u8;
    let g = ((1.0 - amount) * c_start.g as f32 + amount * c_end.g as f32) as u8;
    let b = ((1.0 - amount) * c_start.b as f32 + amount * c_end.b as f32) as u8;
    let a = ((1.0 - amount) * c_start.a as f32 + amount * c_end.a as f32) as u8;
    Color { r, g, b, a }
}
