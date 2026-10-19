use modalkit::keybindings::BindingMachine;

use super::*;

#[test]
fn insert_arrows_recall_sent_input_without_leaving_insert_mode() {
    let mut bindings = setup_keybindings();
    bindings.input_key("i".parse().unwrap());
    while bindings.pop().is_some() {}

    for (key, expected) in [("<Up>", MoveDir1D::Previous), ("<Down>", MoveDir1D::Next)] {
        bindings.input_key(key.parse().unwrap());
        let mut recalls = 0;
        while let Some((action, _)) = bindings.pop() {
            if let Action::Prompt(PromptAction::Recall(filter, direction, count)) = action {
                assert_eq!(filter, RecallFilter::All);
                assert_eq!(direction, expected);
                assert_eq!(count, Count::Exact(1));
                recalls += 1;
            }
        }
        assert_eq!(recalls, 1);
        assert_eq!(bindings.mode(), VimMode::Insert);
    }
}

#[test]
fn sent_input_history_recalls_latest_and_restores_draft() {
    use modalkit::editing::history::{HistoryList, ScrollbackState};

    let mut history = HistoryList::new(EditRope::from(""), 100);
    history.select(EditRope::from("fj list"));
    history.select(EditRope::from("fj show #42"));
    let mut position = ScrollbackState::Pending;
    let draft = EditRope::from("unfinished draft");
    let latest = history
        .recall(&draft, &mut position, &RecallFilter::All, MoveDir1D::Previous, 1)
        .unwrap();
    assert_eq!(latest.to_string(), "fj show #42");
    let previous = history
        .recall(&latest, &mut position, &RecallFilter::All, MoveDir1D::Previous, 1)
        .unwrap();
    assert_eq!(previous.to_string(), "fj list");
    let restored = history
        .recall(&previous, &mut position, &RecallFilter::All, MoveDir1D::Next, 2)
        .unwrap();
    assert_eq!(restored.to_string(), "unfinished draft");

    history.select(EditRope::from("newly sent"));
    position = ScrollbackState::Pending;
    let recalled = history
        .recall(&EditRope::from(""), &mut position, &RecallFilter::All, MoveDir1D::Previous, 1)
        .unwrap();
    assert_eq!(recalled.to_string(), "newly sent");
    let blank = history
        .recall(&recalled, &mut position, &RecallFilter::All, MoveDir1D::Next, 1)
        .unwrap();
    assert!(blank.is_blank());
}

#[test]
fn insert_shift_arrows_move_between_draft_lines() {
    let mut bindings = setup_keybindings();
    bindings.input_key("i".parse().unwrap());
    while bindings.pop().is_some() {}

    for (key, expected) in [
        ("<S-Up>", MoveDir1D::Previous),
        ("<S-Down>", MoveDir1D::Next),
    ] {
        bindings.input_key(key.parse().unwrap());
        let mut motions = 0;
        while let Some((action, _)) = bindings.pop() {
            assert!(!matches!(action, Action::Prompt(PromptAction::Recall(..))));
            if let Action::Editor(EditorAction::Edit(
                _,
                EditTarget::Motion(MoveType::Line(direction), count),
            )) = action
            {
                assert_eq!(direction, expected);
                assert_eq!(count, Count::Exact(1));
                motions += 1;
            }
        }
        assert_eq!(motions, 1);
        assert_eq!(bindings.mode(), VimMode::Insert);
    }
}

#[test]
fn normal_arrows_do_not_recall_input() {
    let mut bindings = setup_keybindings();
    for key in ["<Up>", "<Down>"] {
        bindings.input_key(key.parse().unwrap());
        let mut actions = 0;
        while let Some((action, _)) = bindings.pop() {
            assert!(!matches!(action, Action::Prompt(PromptAction::Recall(..))));
            actions += 1;
        }
        assert!(actions > 0);
        assert_eq!(bindings.mode(), VimMode::Normal);
    }
}
