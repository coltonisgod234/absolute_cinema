# absolute_cinema
play videos in your terminal with the power of opencv!

## building from source
### Linux
First install rustup with this command (if you haven't already)
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then clone the repo and cd into it
```sh
git clone https://github.com/coltonisgod234/absolute_cinema.git
cd absolute_cinema
```

Now build the project
```sh
cargo build --release
```

Be patient, Rust can take a bit to build.
Now, the binary you want will be in `target/release/absolute_cinema`

You can copy that to wherever in your `$PATH`, maybe `~/.local/bin`

### Windows (Win10 1809+)
> [!WARNING]
> Windows is not supported officially.
> You will encounter bugs, this install guide may not even work.

First, we'll install rustup via winget!

Go into the Start Menu > search for "powershell" and open a new `POWERSHELL.EXE` window (or `PWSH.EXE`)

Type in the following and press enter after each line of text shown here. Wait for the operation to complete before typing in the next.
```powershell
winget install Microsoft.VisualStudio.2022.Community --add Microsoft.VisualStudio.Workload.NativeDesktop --includeRecommended
winget install Rustlang.Rustup
winget install Git.Git
```

close the powershell window and open a new one.
Now type these commands (remember to press enter!)
```powershell
git clone https://github.com/coltonisgod234/absolute_cinema.git
cd absolute_cinema
cargo build --release
start .
```

When it's done, a new file browser will pop up.
from there, go into `target\release` and you'll find `absolute_cinema`, that's the file you want to run in your `powershell` window to run the application.
