#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Window;
use tokio::time;
use std::collections::HashMap;


#[derive(Clone, serde::Serialize)]
struct Ship {
  energy: f64,
  energy_growth: f64,
  food: f64,
  food_growth: f64,
  oxygen: f64,
  oxygen_growth: f64,
}


#[tauri::command]
async fn game_loop(window: Window, ship_values: HashMap<String, f64>){
  let mut ship = Ship {
    energy: ship_values["energy"],
    energy_growth: ship_values["energy_growth"],
    food: ship_values["food"],
    food_growth: ship_values["food_growth"],
    oxygen: ship_values["oxygen"],
    oxygen_growth: ship_values["oxygen_growth"],
  };

  let mut interval = time::interval(time::Duration::from_secs(1));
  loop {
    interval.tick().await;
    
    let energy_growth_ppm = ship.energy_growth / 1000000.0;
    let food_growth_ppm = ship.food_growth / 1000000.0;
    let oxygen_growth_ppm = ship.oxygen_growth / 1000000.0;

    ship.energy += energy_growth_ppm;
    ship.food += food_growth_ppm;
    ship.oxygen += oxygen_growth_ppm;

    window.emit("ShipProgress", Ship {
      energy: ship.energy,
      energy_growth: ship.energy_growth,
      food: ship.food,
      food_growth: ship.food_growth,
      oxygen: ship.oxygen,
      oxygen_growth: ship.oxygen_growth,
    }).unwrap();
  } 
}


fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![game_loop])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

