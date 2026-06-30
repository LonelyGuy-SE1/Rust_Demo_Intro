struct BankAccount {
    owner: String,
    balance: i32,
}

impl BankAccount {
    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }
    
    fn display(&self) {
        println!("{} has ${}", self.owner, self.balance);
    }
}

fn main() {
    let mut account = BankAccount {
        owner: String::from("Mort"),
        balance: 5000,
    };
    
    account.display();
}