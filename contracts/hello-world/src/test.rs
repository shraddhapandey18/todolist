#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String, Vec};

fn setup_contract(env: &Env) -> TodoContract {
    TodoContract
}

#[test]
fn test_add_and_get_todo() {
    let env = Env::default();
    let contract = setup_contract(&env);

    contract.add_todo(env.clone(), String::from_slice(&env, "Buy milk"));
    contract.add_todo(env.clone(), String::from_slice(&env, "Read book"));

    let todos = contract.get_todos(env.clone());
    assert_eq!(todos.len(), 2);
    assert_eq!(todos.get(0).unwrap(), String::from_slice(&env, "Buy milk"));
    assert_eq!(todos.get(1).unwrap(), String::from_slice(&env, "Read book"));
}

#[test]
fn test_remove_todo() {
    let env = Env::default();
    let contract = setup_contract(&env);

    contract.add_todo(env.clone(), String::from_slice(&env, "Task 1"));
    contract.add_todo(env.clone(), String::from_slice(&env, "Task 2"));
    contract.add_todo(env.clone(), String::from_slice(&env, "Task 3"));

    contract.remove_todo(env.clone(), 1);

    let todos = contract.get_todos(env.clone());
    assert_eq!(todos.len(), 2);
    assert_eq!(todos.get(0).unwrap(), String::from_slice(&env, "Task 1"));
    assert_eq!(todos.get(1).unwrap(), String::from_slice(&env, "Task 3"));
}

#[test]
fn test_update_todo() {
    let env = Env::default();
    let contract = setup_contract(&env);

    contract.add_todo(env.clone(), String::from_slice(&env, "Old Task"));

    contract.update_todo(env.clone(), 0, String::from_slice(&env, "New Task"));

    let todos = contract.get_todos(env.clone());
    assert_eq!(todos.len(), 1);
    assert_eq!(todos.get(0).unwrap(), String::from_slice(&env, "New Task"));
}