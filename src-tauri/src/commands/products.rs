use crate::db::{self, AppState};
use crate::domain::product::{Product, ProductInput};
use tauri::State;

#[tauri::command]
pub fn product_upsert(state: State<'_, AppState>, input: ProductInput) -> Result<Product, String> {
    input.validate()?;
    let product = Product::from_input(input);
    let conn = state
        .conn
        .lock()
        .map_err(|_| "database lock poisoned".to_string())?;
    db::upsert_product(&conn, &product)?;
    Ok(product)
}

#[tauri::command]
pub fn product_list(
    state: State<'_, AppState>,
    company_id: String,
) -> Result<Vec<Product>, String> {
    if company_id.trim().is_empty() {
        return Err("company_id is required".to_string());
    }
    let conn = state
        .conn
        .lock()
        .map_err(|_| "database lock poisoned".to_string())?;
    db::list_products(&conn, &company_id)
}
