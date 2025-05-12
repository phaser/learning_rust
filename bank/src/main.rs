#[derive(Debug)]
struct Account {
    id: u64,
    balance: i64,
    holder: String
}

impl Account {
    fn new(id: u64, holder: String) -> Self {
        Account { id, holder, balance: 0 }
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
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();

    let other_bank = &bank;
    println!("{:#?}", other_bank);
}
