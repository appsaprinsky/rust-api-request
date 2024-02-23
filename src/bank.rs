
pub struct BankMain{
    pub account_holder: String,
    pub balance: u16,
}

impl BankMain {
    pub fn new(account_holder: &str, balance: u16) -> Self {
        Self {
            account_holder: account_holder.to_string(),
            balance: balance,
        }
    }

    pub fn print_all_info(&self) -> () {
        println!("Hello My dear!");
    }


    pub fn print_money(&self) -> () {
        println!("{}, is your account number", self.balance);

    }

    pub fn add_money(&mut self, money: u16) -> () {
        self.balance = self.balance + money;
        println!("{}, is your New account number", self.balance);

    }
}