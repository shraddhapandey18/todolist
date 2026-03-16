#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, String, Vec, Symbol};

const TODOS_KEY: Symbol = symbol_short!("todos");

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {
    pub fn add_todo(env: Env, item: String) {
        let mut todos = Self::get_todos(&env);
        todos.push_back(item);
        env.storage().instance().set(&TODOS_KEY, &todos);
    }

    pub fn get_todos(env: &Env) -> Vec<String> {
        env.storage().instance().get(&TODOS_KEY).unwrap_or(Vec::new(env))
    }

    pub fn remove_todo(env: Env, index: u32) {
        let mut todos = Self::get_todos(&env);
        if index < todos.len() {
            todos.remove(index);
            env.storage().instance().set(&TODOS_KEY, &todos);
        }
    }

    pub fn update_todo(env: Env, index: u32, new_item: String) {
        let mut todos = Self::get_todos(&env);
        if index < todos.len() {
            todos.set(index, new_item);
            env.storage().instance().set(&TODOS_KEY, &todos);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, String};

    #[test]
    fn test_add_and_get_todo() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TodoContract);
        env.as_contract(&contract_id, || {
            TodoContract::add_todo(env.clone(), String::from_str(&env, "Buy milk"));
            TodoContract::add_todo(env.clone(), String::from_str(&env, "Read book"));

            let todos = TodoContract::get_todos(&env);
            assert_eq!(todos.len(), 2);
            assert_eq!(todos.get(0).unwrap(), String::from_str(&env, "Buy milk"));
            assert_eq!(todos.get(1).unwrap(), String::from_str(&env, "Read book"));
        });
    }

    #[test]
    fn test_remove_todo() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TodoContract);
        env.as_contract(&contract_id, || {
            TodoContract::add_todo(env.clone(), String::from_str(&env, "Task 1"));
            TodoContract::add_todo(env.clone(), String::from_str(&env, "Task 2"));
            TodoContract::add_todo(env.clone(), String::from_str(&env, "Task 3"));

            TodoContract::remove_todo(env.clone(), 1);

            let todos = TodoContract::get_todos(&env);
            assert_eq!(todos.len(), 2);
            assert_eq!(todos.get(0).unwrap(), String::from_str(&env, "Task 1"));
            assert_eq!(todos.get(1).unwrap(), String::from_str(&env, "Task 3"));
        });
    }

    #[test]
    fn test_update_todo() {
        let env = Env::default();
        let contract_id = env.register_contract(None, TodoContract);
        env.as_contract(&contract_id, || {
            TodoContract::add_todo(env.clone(), String::from_str(&env, "Old Task"));

            TodoContract::update_todo(env.clone(), 0, String::from_str(&env, "New Task"));

            let todos = TodoContract::get_todos(&env);
            assert_eq!(todos.len(), 1);
            assert_eq!(todos.get(0).unwrap(), String::from_str(&env, "New Task"));
        });
    }
}
