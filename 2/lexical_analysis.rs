use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug)]
struct TableRow {
    symbol: char,
    address: u32,
    s_type: String,
}

struct LexicalAnalyzer {
    operators: HashSet<String>,
    special_symbols: HashSet<String>,
    keywords: HashSet<String>,
    table: Vec<TableRow>,
    addresses: HashMap<String, u32>,
    address_count: u32,
}

impl LexicalAnalyzer {
    fn new() -> LexicalAnalyzer {
        LexicalAnalyzer {
            operators: HashSet::from_iter("+=-*/%".chars()),
            special_symbols: HashSet::from_iter(",()&$@!;".chars()),
            keywords: HashSet::from_iter("int float double string char".split(" ").map(|s| String::from(s))),
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

    fn distinguish_token(&self, token: String) -> &str {
        if self.operators.contains(token) {
            return "operator";
        }
        if self.special_symbols.contains(token) {
            return "special_symbol";
        }
        "identifier"
    }

    fn get_address(&mut self, token: String) -> u32 {
        if !self.addresses.contains_key(token) {
            self.addresses.insert(token, self.address_count);
            self.address_count += 1;
        }

        self.addresses[token]
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

    fn parse_line(&self, exp: &str) {
        for token in exp.split(" ") {
            println!("{0:?}", token);
            table.insert(token,self.distinguish_token(&token));
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

}

fn main() {
    let mut analyzer = LexicalAnalyzer::new();

    analyzer.parse_line("int a = 10 ;");

    analyzer.print_table();

    println!("Symbol\tAddress\tType");
    for row in analyzer.search_symbol('A') {
        println!("{}\t{}\t{}", row.symbol, row.address, row.s_type);
    }

    analyzer.remove_symbol('A');
    analyzer.print_table();

    // analyzer.check();
}
