## Welcome To Cobol! Common Business-Oriented Language

### What is COBOL?
COBOL, which stands for Common Business-Oriented Language, is a high-level programming language developed in the late 1950s. It was specifically designed for business data processing and is known for its readability and ease of use. COBOL is often associated with mainframe and legacy systems, but it continues to be used in various industries today.

### History

COBOL has a rich history dating back to the late 1950s when it was first developed by a committee of experts. Its primary purpose was to create a programming language that could handle business data processing effectively. Over the years, COBOL has gone through several revisions and updates, with the most recent standard being COBOL 2014.

### Key Features

COBOL comes with a set of features that make it suitable for business-oriented programming:

- **Readability:** COBOL code is known for its readability, thanks to its English-like syntax and self-explanatory keywords. This makes it easy for programmers to understand and maintain COBOL programs.

- **Data Processing:** COBOL excels at handling large volumes of data, making it a preferred choice for applications that require extensive data processing, such as financial systems and government databases.

- **Legacy Compatibility:** Many legacy systems, especially those running on mainframes, are written in COBOL. It's essential for businesses to maintain and update these systems, and COBOL continues to play a crucial role in this context.

- **Structured Programming:** While COBOL is often associated with older programming paradigms, modern versions of COBOL support structured programming features, including subroutines and modular code.

### Getting Started (Using WSL to Run COBOL Compiler)

Pre - Requisites(Software Needed and Configurations)

|Name | Why |
|-----|-----|
|WSL Enabled | To be able to run Linux on Windows |
|Debian| Linux Distro to run GNUCobol Compiler|
|VSCode|Development IDE [VSCode Download Link](https://code.visualstudio.com/download)|
|Remote WSL | To use WSL, [VSCode Extension WSL Link](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-wsl) |
|COBOL VSCode Extenesion | COBOL lines and snippets [VSCode Extension COBOL Link](https://marketplace.visualstudio.com/items?itemName=bitlang.cobol)|

1. Enable WSL on windows 10/11
- Follow this guide [Enable WSL on Windows](https://woshub.com/install-wsl-windows-subsystem-linux/#:~:text=By%20default%2C%20WSL%20is%20disabled,OK%2C%20and%20restart%20your%20computer.&text=Then%20you%20need%20to%20restart%20your%20computer.)

Note: You need to restart your Computer

2. Download Debian from Microsoft Store
- Download it from here [Debian](https://www.microsoft.com/store/productid/9MSVKQC78PK6?ocid=pdpshare)

3. Open Debian and install GNUCobol Compiler
- Set UNIX Username and Password
- Update it by typing this command
```zsh
sudo apt update
```
- Upgrade it by typing this command
```zsh
sudo apt-get upgrade
```
- Installing GNU Cobol
```zsh
sudo apt-get install gnucobol
```

### Running Hello World
1. Open a folder and create a new file called hello.cbl
2. Follow the hello.cbl file on how to create your hello world cobol program
3. Open the terminal and change the terminal profile to **Debian WSL**
4. Run this command
```linux
cobc -x  hello.cbl -o hello.exe
./hello.exe
```

Voila your first program in COBOL!
