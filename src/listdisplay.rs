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
    list::{ListDelegate, ListItem, ListState},
};
use crate::AppState;


pub struct ListDisplay {
    pub appstate: Rc<RefCell<Entity<AppState>>>,
    pub selected_index: Option<IndexPath>,
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
                .child(Label::new(item))
                .selected(Some(ix) == self.selected_index)
        })
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        cx: &mut Context<ListState<ListDisplay>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }

}

