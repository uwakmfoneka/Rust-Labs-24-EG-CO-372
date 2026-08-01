#[derive(Debug, Clone)]
struct BankAccount {
    _owner: String,
    balance: f64,
}

impl BankAccount {
    // Associated function (constructor)
    fn new(owner: &str, initial_balance: f64) -> Self {
        BankAccount {
            _owner: owner.to_string(),
            balance: initial_balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!(
            "Deposited ${:.2}. New balance: ${:.2}",
            amount,
            self.balance
        );
    }

    // Withdraw money from the account
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            Err("Insufficient funds".into())
        } else {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2}. New balance: ${:.2}",
                amount,
                self.balance
            );
            Ok(())
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

pub fn new() {
    let mut acc = BankAccount::new("Alice", 1000.0);

    acc.deposit(500.0);

    match acc.withdraw(200.0) {
        Ok(()) => println!("Withdrawal successful"),
        Err(msg) => println!("Error: {}", msg),
    }

    println!("Final balance: ${:.2}", acc.balance());
}