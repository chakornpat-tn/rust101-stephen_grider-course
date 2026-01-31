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

fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    // let account = Account::new(1, String::from("Me"));
    let other_bank = bank;

    println!("{:#?}", bank);
    // borrow of moved value: `bank`
    // value borrowed here after move
}
