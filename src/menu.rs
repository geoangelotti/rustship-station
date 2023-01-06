use inquire::Select;

/// Build a simple menu based on `items`
pub fn menu(items: &[String]) -> String {
    Select::new("MENU", items.to_vec()).prompt().unwrap()
}
