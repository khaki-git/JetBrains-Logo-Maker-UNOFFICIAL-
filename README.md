# JetBrains Logo Generator
According to the JetBrains website, you cannot create material that reassembles
JetBrains logos.\
*"Don’t imitate
Mimicking JetBrains Brand Assets in non-JetBrains materials is not allowed. The same restriction applies for animating JetBrains Brand Assets, altering them (except for size), combining them with any other words, symbols, images, or designs, or incorporating them into a slogan or tagline."*
From the [JetBrains website](https://www.jetbrains.com/company/brand/).\
Therefore, it should be noted that this project is for entertainment purposes only, and was only made by me to test my rust programming ability and to learn image manipulation.\
You cannot use any logos made by this software as your own (and either way, why would you? This program is terrible at making them)
***
# How to use
Here's a very brief guide on how to use this small app, it will be split into two segments.
Compiling and Running it. You can skip the compiling part if you just download the executable off of the GitHub releases tab if you're running on a Windows machine. 
***
<h2>Compiling</h2>
***

Download and install [Git](https://git-scm.com/) if you haven't already, additionally you should also have [Rust](https://www.rust-lang.org/tools/install) installed.
Make sure that you also install Cargo when downloading Rust.
1. Go into an empty directory and open the terminal.
2. Run `git clone <PATH TO THIS GITHUB PAGE>`
3. It should create a new folder in the directory.
4. `cd` into the folder.
5. Write in the terminal `cargo build` or `cargo run`, this will compile the project.
6. Wait for it and additional packages to compile.

And there you have it, your executable. Bring it anywhere, to work, for coffee on a nice Sunday evening. Do whatever you want for it (as long as it doesn't violate the TOS of JetBrains as stated above)
***
<h2>Running</h2>
***
Open the executable, you should be greeted by an interface window.
The buttons at the bottom control the colour of the gradient, the gradient goes in order of the buttons.
There is a shuffle button if you're not particularly feeling the brightest today.
Above the shuffle button is the Movement slider, it controls how distant the gradient nodes are.
And above all of that, is the generate button. After clicking on the generate button, you will get prompted on where to save your file in your Native File Chooser, you should add `.png` to the end of the file name so
the program knows it's an image with alpha (transparency) support.
After that, the program will try to generate a logo. If it freezes, try closing the window and lowering the movement slider as it likely could have not found a place for one of the gradient nodes.\
If nothing happens after saving it, that means that program has successfully generated a logo and is now saved where you put it.
