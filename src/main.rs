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

fn print_account(mut account: Account) -> Account {
    println!("{:#?}", account);
    account.balace += 10;
    account
}

fn main() {
    let mut account = Account::new(1, String::from("me"));
    let bank = Bank::new();

    account = print_account(account);
    println!("{:#?}", account);
}
