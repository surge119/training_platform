
def main():
    print('Break out of this simple python jail! You are not allowed to use the words eval, exec, import, open, os, read, system, or write.')
    text = input('>>> ')
    for keyword in ['eval', 'exec', 'import', 'open', 'os', 'read', 'system', 'write']:
        if keyword in text:
            print('Play by the rules!!! Try again.')
            return
    exec(text)

if __name__ == "__main__":
	main()