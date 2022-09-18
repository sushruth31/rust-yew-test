use yew_app::todo::{Action, Status, TodoList};

fn list_of(texts: &[&str]) -> TodoList {
    let mut list = TodoList::default();
    for text in texts {
        list.add(text);
    }
    list
}

#[test]
fn add_trims_surrounding_whitespace() {
    let list = list_of(&["  buy milk \n"]);
    assert_eq!(list.items()[0].text, "buy milk");
}

#[test]
fn add_rejects_blank_and_whitespace_only_text() {
    let mut list = TodoList::default();
    assert_eq!(list.add(""), None);
    assert_eq!(list.add("   \t"), None);
    assert!(list.items().is_empty());
}

#[test]
fn duplicate_text_gets_distinct_ids_so_removing_one_keeps_the_other() {
    let mut list = list_of(&["water plants", "water plants"]);
    let first = list.items()[0].id;
    assert_ne!(first, list.items()[1].id);

    assert!(list.remove(first));
    assert_eq!(list.items().len(), 1);
    assert_eq!(list.items()[0].text, "water plants");
}

#[test]
fn ids_are_never_reused_after_a_removal() {
    let mut list = list_of(&["first"]);
    let stale = list.items()[0].id;
    assert!(list.remove(stale));

    let fresh = list.add("second").expect("non-blank text is accepted");
    assert_ne!(stale, fresh);
    // A callback captured before the removal now addresses nothing at all.
    assert!(!list.remove(stale));
    assert_eq!(list.items().len(), 1);
}

#[test]
fn toggling_twice_returns_to_the_original_status() {
    let mut list = list_of(&["ship it"]);
    let id = list.items()[0].id;

    assert!(list.toggle(id));
    assert_eq!(list.items()[0].status, Status::Done);
    assert!(list.toggle(id));
    assert_eq!(list.items()[0].status, Status::Todo);
}

#[test]
fn mutating_an_unknown_id_is_a_no_op_rather_than_a_panic() {
    let mut list = list_of(&["only item"]);
    assert!(!list.toggle(9_999));
    assert!(!list.remove(9_999));
    assert_eq!(list.items().len(), 1);
}

#[test]
fn remaining_counts_only_unfinished_items() {
    let mut list = list_of(&["a", "b", "c"]);
    let id = list.items()[1].id;
    list.toggle(id);
    assert_eq!(list.remaining(), 2);
    assert_eq!(list.items().len(), 3);
}

#[test]
fn apply_reports_whether_the_list_actually_changed() {
    let mut list = TodoList::default();
    assert!(!list.apply(Action::Add("   ".into())));
    assert!(list.apply(Action::Add("real".into())));

    let id = list.items()[0].id;
    assert!(list.apply(Action::Toggle(id)));
    assert!(list.apply(Action::Remove(id)));
    assert!(!list.apply(Action::Remove(id)));
}

#[test]
fn insertion_order_is_preserved_across_edits() {
    let mut list = list_of(&["one", "two", "three"]);
    let middle = list.items()[1].id;
    list.remove(middle);
    list.add("four");

    let texts: Vec<_> = list.items().iter().map(|item| item.text.as_str()).collect();
    assert_eq!(texts, ["one", "three", "four"]);
}
