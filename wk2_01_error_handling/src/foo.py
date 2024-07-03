# unchecked exception model
def better_get_user_number():
    while True:
        try:
            line = input("Please enter a number: ")
            # throws an exception if the user enters not-a-number
            num = int(line)
            break
        except ValueError:
            print("That was not a number. Please try again.")
        except RuntimeError:
            print("Some runtime error occurred.")
        except:  # catch-all block
            print("An error occurred.")
    return num


def get_user_number():
    line = input("Please enter a number: ")
    num = int(line)  # throws an exception if the user enters not-a-number
    return num


def helper1():
    return get_user_number()


def helper2():
    return helper1()


# may throw an exception if the user enters not-a-number -> exception unwinding
# main idea: exceptions are propagated up the call stack until they are caught
num = helper2()

print(f"Double your number is {num * 2}")
