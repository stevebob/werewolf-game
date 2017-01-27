use std::collections::HashMap;

use game::*;
use colour::*;
use direction::*;

pub struct English;

impl English {
    fn translate_you_see(&self, name: YouSeeMessageType, message: &mut Message) {
        match name {
            YouSeeMessageType::Player => {
                message.push(MessagePart::plain("Myself"));
            }
            YouSeeMessageType::Tree => {
                message.push(MessagePart::plain("A tree"));
            }
        }
    }

    fn translate_action(&self, action: ActionMessageType, message: &mut Message) {
        match action {
            ActionMessageType::PlayerOpenDoor => {
                message.push(MessagePart::plain("I open the door."));
            }
            ActionMessageType::PlayerCloseDoor => {
                message.push(MessagePart::plain("I close the door."));
            }
        }
    }

    fn translate_description(&self, description: DescriptionMessageType, message: &mut Message) {
        match description {
            DescriptionMessageType::Player => {
                message.push(MessagePart::plain("I entered the forest in search of an ancient tome."));
            }
        }
    }

    fn translate_input_event(&self, input_event: InputEvent) -> Option<MessagePart> {
        let message_part = match input_event {
            InputEvent::Char(ch) => MessagePart::Text(TextMessagePart::Plain(format!("{}", ch))),
            InputEvent::Up => MessagePart::plain("up"),
            InputEvent::Down => MessagePart::plain("down"),
            InputEvent::Left => MessagePart::plain("left"),
            InputEvent::Right => MessagePart::plain("right"),
            InputEvent::Escape => MessagePart::plain("esc"),
            InputEvent::Return => MessagePart::plain("return"),
            InputEvent::Quit => return None,
        };

        Some(message_part)
    }

    fn translate_direction(&self, direction: Direction) -> &'static str {
        match direction {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
            Direction::NorthWest => "northwest",
            Direction::SouthWest => "southwest",
            Direction::NorthEast => "northeast",
            Direction::SouthEast => "southeast",
        }
    }

    fn translate_control(&self, control: Control, message: &mut Message) {
        let string = match control {
            Control::Direction(direction) => self.translate_direction(direction),
            Control::Close => "close door",
            Control::Fire => "fire",
            Control::NextTarget => "next target",
            Control::PrevTarget => "previous target",
            Control::Wait => "wait a turn",
            Control::DisplayMessageLog => "full screen message log",
            Control::Examine => "examine",
            Control::Select => "select",
            Control::Quit => "quit",
            Control::Help => "help",
        };

        message.push(MessagePart::plain(string));
    }

    fn translate_intro(&self, message: &mut Message) {
        message.push(MessagePart::plain("Everything beneath the moonlight appears different. "));
        message.push(MessagePart::plain("An arcane tome is rumored to be hidden somewhere in the forest. "));
        message.push(MessagePart::plain("Perhaps the answers lie within."));
    }
}

impl Language for English {
    fn translate_repeated(&self, message_type: MessageType, repeated: usize, message: &mut Message) {

        match message_type {
            MessageType::Empty => {},
            MessageType::Intro => self.translate_intro(message),
            MessageType::PressAnyKey => message.push(MessagePart::plain("Press any key...")),
            MessageType::Welcome => {
                message.push(MessagePart::plain("Welcome to "));
                message.push(MessagePart::colour(colours::PURPLE, "HOWL"));
                message.push(MessagePart::plain("!"));
            }
            MessageType::Action(action) => {
                self.translate_action(action, message);
            }
            MessageType::YouSee(name) => {
                message.push(MessagePart::plain("I see: "));
                if let Some(name) = name {
                    self.translate_you_see(name, message);
                }
            }
            MessageType::YouRemember(name) => {
                message.push(MessagePart::plain("I remember: "));
                if let Some(name) = name {
                    self.translate_you_see(name, message);
                }
            }
            MessageType::Unseen => {
                message.push(MessagePart::plain("I haven't seen this location."));
            }
            MessageType::Description(description) => {
                self.translate_description(description, message);
            }
            MessageType::YouSeeDescription(you_see) => {
                self.translate_you_see(you_see, message);
            }
            MessageType::NoDescription => {
                message.push(MessagePart::plain("I see nothing of interest."));
            }
        }

        if repeated > 1 {
            message.push(MessagePart::Text(TextMessagePart::Plain(format!("(x{})", repeated))));
        }
    }

    fn translate_controls(&self, control_map: &ControlMap, message: &mut Message) {

        const NUM_CONTROLS: usize = 13;
        const CONTROL_ORDER: [Control; NUM_CONTROLS] = [
            Control::Direction(Direction::North),
            Control::Direction(Direction::South),
            Control::Direction(Direction::East),
            Control::Direction(Direction::West),
            Control::Wait,
            Control::Close,
            Control::Fire,
            Control::NextTarget,
            Control::PrevTarget,
            Control::Examine,
            Control::DisplayMessageLog,
            Control::Select,
            Control::Quit,
        ];

        let mut reverse_table = HashMap::new();

        for (input_event, control) in control_map.iter() {
            reverse_table.entry(*control).or_insert_with(Vec::new).push(*input_event);
        }

        let mut remaining = reverse_table.len();
        for control in CONTROL_ORDER.iter() {
            if let Some(input_events) = reverse_table.get(control) {
                self.translate_control(*control, message);
                message.push(MessagePart::plain(": "));

                let mut count = 0;
                for input_event in input_events.iter() {

                    if let Some(part) = self.translate_input_event(*input_event) {
                        if count > 0 {
                            message.push(MessagePart::plain(", "));
                        }

                        message.push(part);

                        count += 1;
                    }
                }

                remaining -= 1;
                if remaining > 0 {
                    message.push(MessagePart::Newline);
                }
            }
        }
    }
}
