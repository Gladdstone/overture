use std::rc::Rc;
use std::cell::RefCell;
use gpui::{
    App,
    Context,
    Entity,
    ParentElement,
    Window
};
use gpui_component::{
    IndexPath,
label::Label,
    list::{ListDelegate, ListEvent, ListItem, ListState},
};
use crate::ui::appstate::AppState;


pub struct ListDisplay {
    pub appstate: Rc<RefCell<Entity<AppState>>>,
}

impl ListDelegate for ListDisplay {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.appstate.borrow().read(_cx).application_vec.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        _cx: &mut Context<'_, ListState<ListDisplay>>,
    ) -> Option<Self::Item> {
        self.appstate.borrow().read(_cx).application_vec.get(ix.row).map(|item| {
            ListItem::new(ix)
                .child(Label::new(&item.name))
                .selected(Some(ix) == self.appstate.borrow().read(_cx).selected_index)
        })
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        cx: &mut Context<ListState<ListDisplay>>,
    ) {
        self.appstate.borrow_mut().update(cx, |state, _cx| {
            state.selected_index = ix;
            state.application_vec.get(ix.unwrap().row).unwrap().launch();
        });
        // self.appstate.borrow().
        cx.notify();
    }

}

