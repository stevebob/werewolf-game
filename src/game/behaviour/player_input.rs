use std::ops::Deref;
use std::cmp;

use game::*;
use game::data::*;
use behaviour::LeafResolution;
use direction::Direction;

pub fn player_input<K: KnowledgeRenderer, I: 'static + InputSource + Clone>(input_source: I) -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {
        loop {
            if let Some(meta_action) = get_meta_action(input, input_source.clone()) {
                return LeafResolution::Yield(meta_action);
            }
        }
    })
}

fn get_direction<I: InputSource>(map: &ControlMap, mut input_source: I) -> Option<Direction> {
    input_source.next_input().and_then(|event| {
        map.get(event).and_then(|control| {
            control_to_direction(control)
        })
    })
}

fn control_to_direction(control: Control) -> Option<Direction> {
    match control {
        Control::Direction(d) => Some(d),
        _ => None,
    }
}

fn display_message_log<K: KnowledgeRenderer, I: InputSource>(input: BehaviourInput<K>, mut input_source: I, map: &ControlMap) {

    let mut renderer = input.renderer.borrow_mut();
    let message_log = input.entity.message_log_borrow().unwrap();

    let mut offset = 0;
    let num_lines = renderer.fullscreen_log_num_rows();
    let num_messages = message_log.len();
    let max_offset = if num_messages > num_lines {
        num_messages - num_lines
    } else {
        0
    };

    loop {
        renderer.publish_fullscreen_log(message_log.deref(), offset, input.language);

        if let Some(event) = input_source.next_input() {
            if let Some(control) = map.get(event) {
                match control {
                    Control::Pause |
                        Control::DisplayMessageLog => break,
                    Control::Direction(Direction::North) => {
                        offset = cmp::min(max_offset, offset + 1);
                    }
                    Control::Direction(Direction::South) => {
                        if offset > 0 {
                            offset -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    renderer.publish_all_windows(input.entity, input.language);
}

fn get_meta_action<K: KnowledgeRenderer, I: InputSource>(input: BehaviourInput<K>, mut input_source: I) -> Option<MetaAction> {
    input_source.next_input().and_then(|event| {
        if event == InputEvent::Quit {
            return Some(MetaAction::External(External::Quit));
        }
        input.entity.control_map_borrow().and_then(|map_ref| {
            let map = map_ref.deref();
            map.get(event).and_then(|control| {
                match control {
                    Control::Direction(Direction::East) => Some(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Accelerate))),
                    Control::Direction(Direction::West) => Some(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity.id(), ChangeSpeed::Decelerate))),
                    Control::Direction(Direction::North) => Some(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Up))),
                    Control::Direction(Direction::South) => Some(MetaAction::ActionArgs(ActionArgs::Steer(input.entity.id(), SteerDirection::Down))),
                    Control::Fire => {
                        None
                    }
                    Control::Wait => {
                        Some(MetaAction::ActionArgs(ActionArgs::Null))
                    }
                    Control::Pause => Some(MetaAction::External(External::Pause)),
                    Control::DisplayMessageLog => {
                        display_message_log(input, input_source, map);
                        None
                    }
                    Control::Examine => {
                        None
                    }
                    Control::Use => {
                        Some(MetaAction::ActionArgs(ActionArgs::TryLevelSwitch(input.entity.id())))
                    }
                    _ => None,
                }
            })
        })
    })
}
