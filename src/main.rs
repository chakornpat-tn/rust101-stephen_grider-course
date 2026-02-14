#[derive(Debug, Clone)]
struct Account {
    id: u32,
    balace: i32,
    holder: String,
}
impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balace: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, "me".to_string());

    bank.add_account(account);

    println!("{:#?}", bank);
}
