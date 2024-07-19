// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod quantum_rnd;
use quantum_rnd::{QuantumRnd, RndMode};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn change_rnd_mode(real: bool) -> Result<>

const DEFAULT_PORT: &'static str = "COM7";

struct RndState {
  rnd: QuantumRnd
}

impl Default for RndState {
    fn default() -> Self {
        Self { rnd: QuantumRnd::new(DEFAULT_PORT).unwrap() }
    }
}



// // remember to call `.manage(MyState::default())`
#[tauri::command]
fn set_rnd_mode(state: tauri::State<'_, RndState>, real: bool) -> Result<(), String> {
    state.rnd.set_mode(if real { RndMode::Real } else { RndMode::Fake }).map_err(|_| "Set Mode Error".to_string())?;

    Ok(())
}

#[tauri::command]
fn get_random(state: tauri::State<'_, RndState>) -> Result<Vec<u8>, String> {
    state.rnd.random().map_or_else(|_| Err("Get Random Error".to_string()), |v| Ok(v.to_vec()))
}

fn main() {
    tauri::Builder::default()
        .manage(RndState::default())
        .invoke_handler(tauri::generate_handler![greet, set_rnd_mode, get_random])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
