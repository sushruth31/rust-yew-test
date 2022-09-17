use crate::todo::{Action, Item, Status, TodoList};
use std::rc::Rc;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// The whole view/model bridge: yew owns the `Rc`, `TodoList` owns the rules.
impl Reducible for TodoList {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Action) -> Rc<Self> {
        let mut next = (*self).clone();
        next.apply(action);
        Rc::new(next)
    }
}

#[function_component]
pub fn TodoPage() -> Html {
    let list = use_reducer(TodoList::default);
    let field = use_node_ref();
    let onsubmit = submit_handler(&list, &field);

    html! {
        <section>
            <form {onsubmit}>
                <input ref={field} type="text" placeholder="what needs doing" />
                <button type="submit">{ "add" }</button>
            </form>
            <ul>{ for list.items().iter().map(|item| row(item, &list)) }</ul>
            <p>{ format!("{} remaining", list.remaining()) }</p>
        </section>
    }
}

/// Submit rather than click, so the enter key works; the field is read through a
/// `NodeRef` so every keystroke does not re-render the list.
fn submit_handler(list: &UseReducerHandle<TodoList>, field: &NodeRef) -> Callback<SubmitEvent> {
    let (list, field) = (list.clone(), field.clone());
    Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let Some(input) = field.cast::<HtmlInputElement>() else {
            return;
        };
        list.dispatch(Action::Add(input.value()));
        input.set_value("");
    })
}

fn row(item: &Item, list: &UseReducerHandle<TodoList>) -> Html {
    let done = item.status == Status::Done;
    let toggle = dispatcher(list, Action::Toggle(item.id));
    let remove = dispatcher(list, Action::Remove(item.id));
    let style = if done {
        "text-decoration: line-through"
    } else {
        ""
    };

    html! {
        <li key={item.id.to_string()}>
            <input type="checkbox" checked={done} onchange={toggle} />
            <span {style}>{ item.text.clone() }</span>
            <button onclick={remove}>{ "remove" }</button>
        </li>
    }
}

fn dispatcher<E: 'static>(list: &UseReducerHandle<TodoList>, action: Action) -> Callback<E> {
    let list = list.clone();
    Callback::from(move |_| list.dispatch(action.clone()))
}
