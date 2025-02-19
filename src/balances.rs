use std::collections::BTreeMap;
pub struct Pallet {
  balances: BTreeMap<String, u128>,
}

impl Pallet {
  pub fn new() -> Self {
    Self {
      balances: BTreeMap::new(),
    }
  }

  pub fn set_balance(&mut self, who: &String, amount: u128) {
    self.balances.insert(who.clone(), amount);
  }

  pub fn balance(&self, who: &String) -> u128 {
    *self.balances.get(who).unwrap_or(&0)
  }

  pub fn transfer(
    &mut self,
    caller: String,
    to: String,
    amount: u128,
  ) -> Result<(), &'static str> {
    let caller_balance = self.balance(&caller);
    let to_balance = self.balance(&to);

    let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Not enough funds.")?;
    let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow.")?;

    self.balances.insert(caller, new_caller_balance);
    self.balances.insert(to, new_to_balance);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn init_balances() {
    let mut balances = super::Pallet::new();
  
    assert_eq!(balances.balance(&"luanfariaf".to_string()), 0);
    balances.set_balance(&"luanfariaf".to_string(), 100);
    assert_eq!(balances.balance(&"luanfariaf".to_string()), 100);
    assert_eq!(balances.balance(&"jhondoe".to_string()), 0);
  }

  #[test]
  fn transfer_balance() {
    let mut balances = super::Pallet::new();

    assert_eq!(
      balances.transfer("luanfariaf".to_string(), "jhondoe".to_string(), 51), 
      Err("Not enough funds.")
    );

    balances.set_balance(&"luanfariaf".to_string(), 100);
    assert_eq!(balances.transfer("luanfariaf".to_string(), "jhondoe".to_string(), 51), Ok(()));
    assert_eq!(balances.balance(&"luanfariaf".to_string()), 49);
    assert_eq!(balances.balance(&"jhondoe".to_string()), 51);

    assert_eq!(
      balances.transfer("luanfariaf".to_string(), "jhondoe".to_string(), 51), 
      Err("Not enough funds.")
    );
  }

}