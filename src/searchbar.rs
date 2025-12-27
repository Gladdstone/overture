use std::process::Command;
use std::rc::Rc;
use std::cell::RefCell;
use gpui::{
    App,
    AppContext,
    Context,
    Entity,
    EventEmitter,
    Focusable,
    FocusHandle,
    prelude::*,
    rgba,
    SharedString,
    Subscription,
    Window,
};
use gpui_component::{
    input::{InputEvent, InputState, Input},
    v_flex,
};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use crate::AppState;


pub struct SearchBar {
    input_state: Entity<InputState>,
    search_text: SharedString,
    _subscriptions: Vec<Subscription>,
    focus_handle: FocusHandle,
}

struct SearchEvent {
    search_text: String,
}

impl EventEmitter<SearchEvent> for SearchBar {}

impl SearchBar {
    pub fn new(window: &mut Window, cx: &mut Context<Self>, application_vec: Rc<RefCell<Entity<AppState>>>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Search"));
        let output = Command::new("ls")
            .arg("/Applications")
            .output()
            .expect("failed to execute ls");

        let matcher = SkimMatcherV2::default();

        let working_vec: Vec<String> = String::from_utf8_lossy(&output.stdout).split("\n").map(|x| x.to_string()).collect();

        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
            let input_state = input_state.clone();
            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    this.search_text = input_state.read(cx).value();

                    let mut apps_mut = working_vec.clone();
                    let mut result: Vec<(i64, &String)> = apps_mut
                        .iter()
                        .filter_map(|item| {
                            matcher
                                .fuzzy_match(item, &this.search_text.as_str())
                                .map(|score| (score, item))
                        })
                        .collect();

                    result.sort_unstable_by(|a, b| b.0.cmp(&a.0));
                    let top_n: Vec<String> = result.into_iter().take(3).map(|(_num, s)| s.to_string()).collect();

                    println!("{:?}", top_n);
                    for value in 0..top_n.len() {
                        apps_mut[value] = top_n[value].clone();
                    }
                    application_vec.borrow_mut().write(cx, AppState{ application_vec: apps_mut });
                }
                _ => {}
            }
        })];

        Self {
            input_state: input_state,
            search_text: SharedString::default(),
            // appstate: application_vec,
            _subscriptions: _subscriptions,
            focus_handle: cx.focus_handle(),
        }
    }

}

impl Focusable for SearchBar {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SearchBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .p_5()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .track_focus(&self.focus_handle(_cx))
            .child(Input::new(&self.input_state))
            .bg(rgba(0x1e1e1e66))

    }
}
