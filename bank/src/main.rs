#[derive(Debug)]
struct Account {
    id: u64,
    balance: i64,
    holder: String
}

impl Account {  
    fn new(id: u64, holder: String, balance: i64) -> Self {
        Account { id, balance, holder }
    }

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }

    fn deposit(&mut self, account: &mut Account, amount: i64) -> i64{
        if amount > account.balance {
            println!("Insufficient balance");
            return self.balance;
        }
        if amount < 0 {
            println!("Invalid amount");
            return self.balance;
        }

        self.balance += amount;
        account.balance -= amount;
        self.balance
    }

    fn withdraw(&mut self, account: &mut Account, amount: i64) -> i64 {
        if amount < 0 {
            println!("Invalid amount");
            return self.balance;
        }
        if amount > self.balance {
            println!("Insufficient funds");
            return self.balance;
        }

        self.balance -= amount;
        account.balance += amount;
        self.balance
    }
}

#[derive(Debug)]
struct Bank {
    account: Vec<Account>
}

impl Bank {
    fn new() -> Self {
        Bank { account: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.account.push(account);
    }

    fn total_balance(&self) -> i64 {
        self.account.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.account.iter().map(|account| account.summary()).collect()
    }
}


fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, "John".to_string(), 10);
    let mut account2 = Account::new(2, "Jim".to_string(), 100);

    account.deposit(&mut account2, 50);
    account.withdraw(&mut account2, 10);


    bank.add_account(account);
    println!("{:#?}", bank.summary());

    println!("{}", bank.total_balance());
}
