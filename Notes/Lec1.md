# **From Step 0 to Step 29**


These are my notes form the Lecture 1 of the "Porting text editor in Rust" playlist. The topics are mentioned in the order of their arrival in the video for easier tracking and reference.

### Doubts - 
- How let Key() is used?
- For turning off some signals, flags are checked. Now, for example, I can press ctrl-c and ctrl-z as input, which means it is already disabled but its associated flag isig is turned on (no -ve sign). Confusion?ignbrk 
- In linux terminal, cooked mode has several flags and raw mode has several flags, but without doing anything, the terminal's flags are neither completely in raw mode nor in cooked mode.- timeout for read() - step 16 - req (for 1/10 ms)? 
- Error handling -> NO error shown after match introduced whereas video has the error?	
- Ok(true) and Ok(false) usage?
- For condition if editor_refresh_screen.isErr(){}, why the task of the function is performedeven if there is no error? And if it is performed then why unused import warning? (main.rs and output.rs)
- Step 24 is not implemented correctly, how to bring clear screen after clicking ctrl-q? (affects step 25)

## Cargo
Original document - https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

Cargo is Rust’s build system and package manager. (Like Pip is Python's package manager)

It handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries.

- It has also initialized a new Git repository along with a .gitignore file. Git files won’t be generated if you run cargo new within an existing Git repository.

- Cargo expects your source files to live inside the src directory.


## Crossterm

Original document - https://docs.rs/crossterm/latest/crossterm/

Used for providing terminal as a user interface

## Crates 

Original document - https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html

Provides functionality.

- Binary crates 
	- Compiles to an executable to run
	- Have main function that defines executable functionality
	- Can be multiple in a package (Package - one or more crates together)

- Library crates
	- Defines functionality that is shared in multiple projects
	- Don't have main function and doesn't compile to an executable
	- At most one in a package

## Cannonical mode vs Raw mode

The terminal operates in two modes - cannonical or cooked mode and the raw mode

- Cannonical mode
	- Default mode in terminal
	- Keyboard input only sent when Enter key is pressed
	- line by line input reading
	- Can see the input in the terminal

- Raw mode
	- Input is taken with each keypress
	- byte by byte input reading
	- Input is not displayed in the terminal

## What is Result

Original document - https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?search=

Result<T,E> is a return type and can be defined as - 
*enum Result<T,E> {
	Ok(T), // If program succeded
	Err(E), // If program fails
}*

T represents the type of the value that will be returned in a success case within the Ok variant, and E represents the type of the error that will be returned in a failure case within the Err variant.


## Use of ? operator 

Reference articles - 
- https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html?highlight=Result#recoverable-errors-with-result
- https://www.becomebetterprogrammer.com/rust-question-mark-operator/#:~:text=operator%20in%20Rust%20is%20used,or%20Option%20in%20a%20function.
- https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about?answertab=trending#tab-top

The ? operator is used for error propogation. By error propogation we mean that when a function's (say F1) implementation calls something (another function/method, say F2) that can fail so instead of handling the error within that function (F2), the error is returned to the calling code (The code that called the function), say F3 through F1. This way the calling code can decide what to do with the error as per the main code's purpose.

The ? operator can be called on *Result* and *Option* types only. For example, if the ? operator is called on a function/method of *Result* return type, then it performs in this way -  

- Returns value inside *Ok* i.e what the function is supposed to do, or when it is succeeded.
- Returns early out of the whole function if an error is encountered and a *Err* value is given to the calling code.

*Option* is a type that can either be *Some* or *None*. But none of these two possible values are error themselves. Now, if ? is used on a function/method with Option return type

- Returns *Some* value, if succeded.
- If no value is found, returns *None*, can be treated as an error but not exactly an error.

**Note :** We can use the ? operator on a Result in a function that returns Result. Similarly, we can use the ? on an Option in a function that returns Option, but can't mix and match. Or else it will throw an error. 

## Enabling Raw Mode

Reference article - https://man7.org/linux/man-pages/man1/stty.1.html

- icrnl	- to translate carriage return to newline
	- turns off Ctrl-M (carriage return) (Step 12)
- ixon	- enable XON/XOFF flow control 
	- turn off ctrl-s and ctrl-q (Step 10)
- icanon	- enable special characters: erase, kill, werase, rprnt
	- turns off canonical mode (step 7)
- opost	- postprocess output
	- translation of "\n" to "\r\n" (Step 13)
- isig	- enable interrupt, quit, and suspend special characters
	- turn off ctrl-c ctrl-z (step 9)

Ctrl-i : gives tab

Ctrl-m : gives enter

Ctrl-q : exits

## if let vs match

Reference article - https://patshaughnessy.net/2018/1/18/learning-rust-if-let-vs--match#:~:text=If%20Let%20Compares%20a%20Pattern,code%20inside%20the%20block.

if let is sugar syntatic of match i.e both performs exactly same and gives same executable code.

## Use of unreachable!()

Indicates unreachable code. This is useful any time that the compiler can’t determine that some code is unreachable. For example: Loops that dynamically terminate.

**CAUTION:** Step 26 is skipped.
