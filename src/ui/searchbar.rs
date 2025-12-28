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
use crate::ui::appstate::AppState;


pub struct SearchBar {
    appstate: Rc<RefCell<Entity<AppState>>>,
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
    pub fn new(window: &mut Window, cx: &mut Context<Self>, appstate: Rc<RefCell<Entity<AppState>>>) -> Self {
        let input_state = cx.new(|cx| InputState::new(window, cx).placeholder("Search"));
        let output = Command::new("ls")
            .arg("/Applications")
            .output()
            .expect("failed to execute ls");

        let matcher = SkimMatcherV2::default();

        let working_vec: Vec<String> = String::from_utf8_lossy(&output.stdout).split("\n").map(|x| x.to_string()).collect();

        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
        let input_state = input_state.clone();
        let working_vec = working_vec.clone();

        move |this, _, ev: &InputEvent, _window, cx| {
            if let InputEvent::Change = ev {
                this.handle_input_change(
                    cx,
                    &input_state,
                    &working_vec,
                    &matcher,
                );
            }
        }
        })];

        Self {
            appstate: appstate,
            input_state: input_state,
            search_text: SharedString::default(),
            _subscriptions: _subscriptions,
            focus_handle: cx.focus_handle(),
        }
    }

    fn handle_input_change(
        &mut self,
        cx: &mut Context<Self>,
        input_state: &Entity<InputState>,
        working_vec: &[String],
        matcher: &SkimMatcherV2,
    ) {
        self.search_text = input_state.read(cx).value();

        let mut apps_mut = working_vec.to_vec();

        let mut result: Vec<(i64, &String)> = apps_mut
            .iter()
            .filter_map(|item| {
                matcher
                    .fuzzy_match(item, self.search_text.as_str())
                    .map(|score| (score, item))
            })
            .collect();

        result.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let top_n: Vec<String> = result
            .into_iter()
            .take(3)
            .map(|(_, s)| s.to_string())
            .collect();

        println!("{:?}", top_n);

        for (i, value) in top_n.iter().enumerate() {
            apps_mut[i] = value.clone();
        }

        self.appstate
            .borrow_mut()
            .write(cx, AppState {
                selected_index: None,
                application_vec: apps_mut,
            });
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
