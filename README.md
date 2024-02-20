# Find lonely files

This is a simple rust project that compares files in a folder and its subfolders. It compares the files based on their `file_names` and prints out the files that are lonely.

For example, if you have files: 
- `file1.txt`
- `file2.txt`
- `file3.txt`
- `file1.md`

This program will print out `file2.txt` and `file3.txt` as lonely files, because they don't have a corresponding file with the same name but different extension.

## How to run the code

1. Clone the repository
2. Open the terminal and navigate to the project folder
3. Run the following command to install the required packages
```bash
    cargo build
```

4. Run the following command to run the project
```bash
    cargo run -- [pathToFolderToBeScaned]
```
There are additional options that can be used with the command. To see the options run the following command
```bash
    cargo run -- --help
```

## Example
Main branch has a folder named `testFolder`, you can test the code on this folder, by running the following command

```bash
    cargo run -- testFolder
```

If you want to compare only specific file extensions, you can pass in an option to look for specific file extensions. For example, to look for only `.txt` and `.md` files, you can run the following command

```bash
    cargo run -- testFolder lip maw
```
