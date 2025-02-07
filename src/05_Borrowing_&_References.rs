// References and Borrowing (Basically the same thing)

// Understanding Referneces
// This is done by borrowinng from the original owner
// A value has only one owner, cannot have multiples owners for the same value
// That's why we have to make referneces of the value 
// References can be both Immutable and Mutable


fn main() {
    // let mut _x:i32 = 5;
    // let _r:&i32 = &mut _x;

    // *_r += 1;

    let mut account = BankAccount {
        owner: "Jhay".to_string(),
        balance: 150.55,
    };

    // Immutable Borrow to Check Balance
    account.check_balance();

    // Mutable Borrow to Withdraw Money
    account.withdraw(45.50);

    account.check_balance();

}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn withdraw(&mut self, amount:f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}
