#[derive(Debug)]
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

fn change_account(account: &mut Account) {
    account.balace += 20;
}

fn main() {
    let mut account = Account::new(1, String::from("me"));
    let account_ref = &mut account;

    // cannot assign to `account.balace` because it is borrowed
    // `account.balace` is assigned to here but it was already borrowed
    // account.balace = 100;

    print_account(account_ref);
    change_account(account_ref);

    println!("{:#?}", account);
}
