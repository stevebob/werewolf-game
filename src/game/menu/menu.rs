use game::*;

pub type Menu<T> = Vec<MenuItem<T>>;

pub mod menu_operation {

    use game::*;

    pub fn run<R: KnowledgeRenderer, I: InputSource, T>(renderer: &mut R,
                                                        input: &mut I,
                                                        prelude: Option<MessageType>,
                                                        language: &Box<Language>,
                                                        menu: Menu<T>,
                                                        initial_state: Option<MenuState>) -> (T, MenuState) {

        let mut state = initial_state.unwrap_or_default();

        loop {
            renderer.publish_fullscreen_menu(prelude, &menu, &state, language);

            if let Some(event) = input.next_input() {
                match event {
                    InputEvent::Down => {
                        state.select_next(&menu);
                    }
                    InputEvent::Up => {
                        state.select_prev(&menu);
                    }
                    InputEvent::Return => {
                        return (state.confirm(menu), state);
                    }
                    _ => {}
                }
            }
        }
    }
}
