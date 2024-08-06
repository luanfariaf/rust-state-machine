mod balances;
mod system;
fn main() {
    println!("Hello Rust!");
    let mut pallet = balances::Pallet::new();
    pallet.set_balance(&"luanfariaf".to_string(), 50);

    let balance = pallet.balance(&"luanfariaf".to_string());
    println!("Balance: {}", balance);
}
