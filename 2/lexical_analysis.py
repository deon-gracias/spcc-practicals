symbol_table = {
    "keyword": {'int', 'float', 'double', 'string', 'char'},
    "special_symbol": {';'},
    "operator": {'+', '-', '*', '=', '/'}
}

def check_literal(token):
    if token[0] in "'\"":
        return True
    return False

def check_constant(token):
    if token.replace('.', '', 1).isdigit():
        return True

def parse(lines):
    table = []

    for line in lines:
        line = line.strip()
        tokens = line.split(" ")
        for token in tokens:
            token_type = 'identifier'
            for key in symbol_table.keys():
                if token in symbol_table[key]:
                    token_type = key
            if check_literal(token):
                token_type = 'literal'
            if check_constant(token):
                token_type = 'constant'
            table.append([token, token_type])
    return table

def parse_file(file_path):
    with open(file_path) as f:
        return parse(f.readlines())

if __name__ == '__main__':
    result = parse_file('input.txt')
    for i in result:
        print(i[0] + " is " + i[1])
