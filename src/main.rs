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
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn add_account(bank: &mut Bank, account: &Account) {
    bank.accounts.push(account.clone());
}

fn main() {
    let account = Account::new(1, String::from("me"));
    let mut bank = Bank::new();

    add_account(&mut bank, &account);

    println!("{:#?}", bank);
    print_account(&account);
}
