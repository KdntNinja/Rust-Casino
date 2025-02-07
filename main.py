while True:
	num1 = input("Input num1: ")
	num2 = input("Input num2: ")

	try:
		num1 = float(num1)
		num2 = float(num2)
	except ValueError:
		print("Invalid input. Please enter numeric values.")
		continue

	operation = input("Enter operation (add, subtract, divide, multiply): ").strip().lower()

	if operation == "add":
		result = num1 + num2
	elif operation == "subtract":
		result = num1 - num2
	elif operation == "divide":
		if num2 == 0:
			print("Cannot divide by zero.")
			continue
		result = num1 / num2
	elif operation == "multiply":
		result = num1 * num2
	else:
		print("Invalid operation.")
		continue

	print(f"Result as float: {result}")
	print(f"Result as integer: {int(result)}")

	while True:
		repeat = input("Do you want to repeat the program? (yes/no): ").strip().lower()
		if repeat == "yes":
			break
		elif repeat == "no":
			exit()
		else:
			print("Please enter 'yes' or 'no'.")

