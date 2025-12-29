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
use crate::core::{ AppItem, collect_apps };
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

        let matcher = SkimMatcherV2::default();

        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
        let input_state = input_state.clone();

        move |this, _, ev: &InputEvent, _window, cx| {
            if let InputEvent::Change = ev {
                this.handle_input_change(
                    cx,
                    &input_state,
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
        matcher: &SkimMatcherV2,
    ) {
        self.search_text = input_state.read(cx).value();

        let mut app_vec = collect_apps();

        let mut result: Vec<(i64, &AppItem)> = app_vec
            .iter()
            .filter_map(|item| {
                matcher
                    .fuzzy_match(&item.name, self.search_text.as_str())
                    .map(|score| (score, item))
            })
            .collect();

        result.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let top_n: Vec<AppItem> = result
            .into_iter()
            .take(3)
            .map(|(_, s)| s.clone())
            .collect();

        println!("{:?}", top_n);

        self.appstate
            .borrow_mut()
            .write(cx, AppState {
                selected_index: None,
                application_vec: top_n,
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
