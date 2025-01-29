while True:
    try:
        number = int(input("Enter the times table you want to create: "))
        for i in range(1, 13):
            print(f"{n} x {i} = {n * i}")
    except ValueError:
        print("Please enter a valid number.")
        continue
    
    while True:
        another_go = input("Do you want another go? (yes/no): ").strip().lower()
        if another_go == "yes" | another_go == "y":
            break
        elif another_go == "no" | another_go == "n":
            print("Goodbye!")
            return
        else:
            print("Please answer with 'yes' or 'no'.")