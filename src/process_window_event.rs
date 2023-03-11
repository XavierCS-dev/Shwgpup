use winit::event::WindowEvent;
use winit::window::WindowId;
use winit::event_loop::ControlFlow;
use winit::event::*;
use crate::state::State;

pub fn window_event(
    ref event: &WindowEvent,
    window_id: &WindowId,
    control_flow: &mut ControlFlow,
    state: &mut State,
) {
    if !state.input(event) {
        // UPDATED!
        match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                state.resize(**new_inner_size);
            }
            _ => {}
        }
    }
}
