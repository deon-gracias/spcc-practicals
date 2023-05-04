import pandas as pd
from random import randint

table = []
operators = "+=-*/%"
special_symbols = ",()&$@!;"
addresses = {}

def distinguish_token(token):
    if token in operators:
        return 'operators'
    if token in special_symbols:
        return 'special_symbol'
    return 'identifier'

def create_table(exp):
    global address

    tokens = [*exp]
    for token in tokens:
        enter_symbol(token)

def generate_random():
    while True:
        i = randint(1000, 10000)
        if i not in addresses.values():
            return i

def get_address(token):
    if token not in addresses.keys():
        addresses[token] = generate_random()

    return addresses[token]

def enter_symbol(token):
    table.append({
        'Symbol': token,
        'Address': get_address(token),
        'Type': distinguish_token(token)
    })

def remove_symbol(token):
    global table
    table = filter(
        lambda row: False if row['Symbol'] == token else True, table)

def search_table(token):
    result = filter(
        lambda row: True if row['Symbol'] == token else False, table)
    return result

def create_menu():
    while True:
        option = int(input(
            "\n1. Create table \n2. Search table \n3. Enter Symbol \n4. Remove symbol \n5. View table \n6. Exit\n# Enter your choice : "))
        if option == 6:  # Exit
            break
        elif option == 1:  # Create Table
            expText = input("# Enter Expression: ")
            create_table(expText)
        elif option == 2:  # Search table
            token = input("# Enter Search Table Name: ")
            print(pd.DataFrame(search_table(token)))
        elif option == 3:  # Enter Symbol
            token = input("# Enter Symbol Add: ")
            enter_symbol(token)
        elif option == 4:  # Remove Symbol
            token = input("# Enter Symbol to remover: ")
            remove_symbol(token)
        elif option == 5:  # View Table
            print(pd.DataFrame(table))
        else:
            print("Wrong Option")

if __name__ == "__main__":
    # create_table("D=A+B*C")
    # print(pd.DataFrame(table))
    # enter_symbol('E')
    # remove_symbol('D')
    # print(pd.DataFrame(table))

    create_menu()
