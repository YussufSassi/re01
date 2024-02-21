Solution for <https://crackmes.one/crackme/5e66aea233c5d4439bb2dde8>

Tools used (in order):

- DIE
- objdump
- Ghidra

Validation function explanation:

- User enters a password
- check_key function is called
- It checks if the string is greater than 7 but less than 11
- It then adds up the ascii values of each character and outputs a success message if the sum of the password's ascii characters is greater than or equal to 1000
