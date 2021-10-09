# Oracle-rs

A recreation of the Oracle from TempleOS, by Terry A Davis

## Usage 

Running the binary with no commands will produce a single random word from the wordlist at the default location.

{
  ./oracle-rs -f ~/Doccuments/wordlist.txt -q 17
}

The '-f' flag specifies the path of the file to be read from.

The '-q' flag specifies the number of words to print.

## todo

- Intergation with linux notifications
- Packaging in the AUR
- Triggering a notification with 'F7'

## Credit

Thank you to [orhun](https://github.com/orhun/godsays), for the wordlist, although all the code is my own.
