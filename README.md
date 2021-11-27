# Satori CLI
Command line program to submit solutions on the code testing platform [Satori](https://satori.tcs.uj.edu.pl/)

## Usage
### Step 1 (you only need to do it once)
```sh
satori set-username YOUR_SATORI_USERNAME
```
(you can skip this step and use the `SATORI_USERNAME` env variable instead)
### Step 2
Add a comment anywhere in the solution code, containing the problem URL, for example:
```cpp
#include <iostream>
// https://satori.tcs.uj.edu.pl/contest/0123456/problems/0123456

int main() {}
```
### Step 3
Run
```sh
satori submit solution-filename.cpp
```
Type your Satori password if prompted or provide it as `SATORI_PASSWORD`.

You can enable the `--ci` flag to prevent asking for password if the env variable is not specified. This is useful for scripting.

If you want to open the results page in your default web browser automatically, use the `--open` (`-o`) flag.

### Note about passwords
This program doesn't store your password. Instead it stores a session token (in the form of a cookie). This is why the program might ask you for your password multiple times.
