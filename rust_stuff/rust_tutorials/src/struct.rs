
struct BankAccount{
    owner: String,
    balance: f64
}  

// withdraw cannot have mutable access to account to update balance, and immutable access for reading owners name simutaneously
impl BankAccount{
    // withdraw cannot have mutable access to account to update balance, and immutable access for reading owners name simutaneously
    //this prevents race condition for account to be modifies because we can onyl have a single mutable reference in rust at a time
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){ // borrowing, immutable -- many people can access at once
        //no other code modifies balance while we check the balance 
        println!("Account owned by {},  Balance is: {}", self.owner,  self.balance)
    }

}

fn main(){
    let mut account: BankAccount = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55
    };
    //immutable borrowing to check the balance
    account.check_balance();

    //mutable borrowing 
    account.withdraw(45.5);

}