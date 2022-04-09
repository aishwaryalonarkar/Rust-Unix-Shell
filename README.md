Rust-Unix-Shell

Implementation of Unix Shell in Rust. Following are the features to be impelemented in the Unix Shell with Rust:
a. cmd_history: Displays the history of commands entered in the command line.

b. rmallexn(remove all except n): Remove all the files and directories in the directory path mentioned in the argument except the file mentioned.

c. listDir [-l][-a][-tree][-color] <directory>: List the contents of the directory with optional arguments.

d. rev_search(Reverse search and execute): Search in the history to check if the substring entered matches any entry and accordingly execute it

e. sortbytype (Sort by type): This command is used to combine multiple files of similar pattern or type and it will add those files into a one separate sub directory.

f. quit: Use quit to get out of rust command line.

# Crates
1. json - 0.12.4

# cmd_history
1. Track down all the commands been entered onto the command line.
2. Displays the history of commands entered.

cmd usage: cmd_history

# listDir
Release 2
1. The list command will accept a variety of parameters.
2. The parameters involves:

    a. -l : List the files and directories in the path mentioned as the argument.

    b. -a: List all the files including the hidden files, symbolic links.

    c. -tree: Show the hierarchy of all the files and directory inside the current directory.

    d. -color: Display the files in different colors as per their file type
    
Call to the API
    
listDir [-l] [-a] [-tree] [-color] \<directory\>
  

# rmallexn
Release 2
Deletes the file and folder in a directory except the one passed as argument to the command

Sample syntax: 
1. rmallexn test/abc/def 
The above command will delete all the files and directories in test/abc/ and keep only test/abc/def
    
2. rmallexn test/xyz/file1.txt
The above command will delete all the files and directories in test/xyz/ and keep only test/xyz/file1.txt

The file or the directory to keep should not have whitespaces in the name.

# rev_search
Release 3

# sortbytype
Release 3

# General attributes
Release 3
