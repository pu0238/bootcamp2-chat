use std::{cell::{Ref, RefCell}, collections::HashMap};

use candid::Principal;
use ic_cdk::caller;

thread_local! {
    static NOTES: RefCell<HashMap<Principal, Vec<String>>> = RefCell::default();
    static CHAT: RefCell<HashMap<[Principal; 2], Vec<String>>> = RefCell::default();
}

#[ic_cdk::query]
fn get_chat(user1: Principal, user2: Principal) -> Option<Vec<String>> {
    let mut principals = [user1, user2];
    principals.sort();
    CHAT.with_borrow(|chats|  chats.get(&principals).cloned())
}

#[ic_cdk::update]
fn add_chat_msg(msg: String, user2: Principal) {
    let user1 = caller();

    if user1 == Principal::anonymous() {
        panic!("Anonymous Principal!")
    }

    let mut principals = [user1, user2];
    principals.sort();


    CHAT.with_borrow_mut(|chats| {
        let mut_chat = chats.get_mut(&principals);

        if let Some(chat_msgs) = mut_chat {
            chat_msgs.push(msg);
        } else {
            chats.insert(principals, vec![msg]);
        }
    })
}

#[ic_cdk::query]
fn get_notes(user: Principal) -> Option<Vec<String>> {
    NOTES.with_borrow(|notes|  notes.get(&user).cloned())
}

#[ic_cdk::update]
fn add_note(note: String) {
    let user = caller();

    if user == Principal::anonymous() {
        panic!("Anonymous Principal!")
    }

    NOTES.with_borrow_mut(|notes| {
        let mut_notes = notes.get_mut(&user);

        if let Some(notes_vec) = mut_notes {
            notes_vec.push(note);
        } else {
            notes.insert(user, vec![note]);
        }
    })
}