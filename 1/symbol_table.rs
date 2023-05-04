use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug)]
struct TableRow {
    symbol: char,
    address: u32,
    s_type: String,
}

struct SymbolTable {
    operators: HashSet<char>,
    special_symbols: HashSet<char>,
    table: Vec<TableRow>,
    addresses: HashMap<char, u32>,
    address_count: u32,
}

impl SymbolTable {
    fn new() -> SymbolTable {
        SymbolTable {
            operators: HashSet::from_iter("+=-*/%".chars()),
            special_symbols: HashSet::from_iter(",()&$@!;".chars()),
            table: Vec::new(),
            addresses: HashMap::new(),
            address_count: 100,
        }
    }

    fn check(&self) {
        println!("Operators : {0:?}", self.operators);
        println!("Special Symbols : {0:?}", self.special_symbols);
        println!("Addresses : {0:?}", self.addresses);
        println!("Table : {0:?}", self.table);
    }

    fn distinguish_token(&self, token: char) -> &str {
        if self.operators.contains(&token) {
            return "operator";
        }
        if self.special_symbols.contains(&token) {
            return "special_symbol";
        }
        "identifier"
    }

    fn get_address(&mut self, token: char) -> u32 {
        if !self.addresses.contains_key(&token) {
            self.addresses.insert(token, self.address_count);
            self.address_count += 1;
        }

        self.addresses[&token]
    }

    fn print_table(&self) {
        println!("Symbol\tAddress\tType");
        for row in self.table.iter() {
            println!("{0}\t{1}\t{2}", row.symbol, row.address, row.s_type);
        }
    }

    fn enter_symbol(&mut self, token: char) {
        let address = self.get_address(token);
        self.table.push(TableRow {
            address,
            symbol: token,
            s_type: String::from(self.distinguish_token(token)),
        });
    }

    fn create_table(&mut self, exp: &str) {
        for token in exp.chars() {
            self.enter_symbol(token);
        }
    }

    fn remove_symbol(&mut self, token: char) {
        self.table.retain(|row| row.symbol != token);
    }

    fn search_symbol(&mut self, token: char) -> Vec<&TableRow> {
        self.table
            .iter()
            .filter(|row| row.symbol == token)
            .collect()
    }

    fn menu() {
        println!(
            "1. Create Table\n2. Add Symbol\n3. Search Symbol\n4. Remove Symbol\nEnter choice: "
        );
    }
}

fn main() {
    let mut symbol_table = SymbolTable::new();

    symbol_table.create_table("A+B-A");

    symbol_table.print_table();

    println!("Symbol\tAddress\tType");
    for row in symbol_table.search_symbol('A') {
        println!("{}\t{}\t{}", row.symbol, row.address, row.s_type);
    }

    symbol_table.remove_symbol('A');
    symbol_table.print_table();

    // symbol_table.check();
}
