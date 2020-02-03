# batcall

Create a symbolic link for executing the bat or cmd file

# What is it?

At the command prompt, the bat or cmd file can be executed without an extension by defining the environment variable PATHEXE.

```bat
> SET PATHEXT
PATHEXT=.COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH
```

But, on other systems, it will not be searched automatically.  
This create exe file that runs bat or cmd to assist the system, As a symbolic link.

# Usage

```cmd
# create symbolic link in the same directory as target command
> batcall --batcall-where-mklink target_command
# the same 
> cd C:/path/to/target_command_dir
> mklink target_command.exe C:/path/to/batcall.exe
```

And run.

```cmd
> target_command
# run target_command.bat or target_command.cmd
```

# Dependency

Depends on mklink installed

# License

MIT
