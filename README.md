# Sand Namer

This is a very fast created program, the goal was to generate project name for my sandbox project where i create tons of environement to test small snippets of code in different languages, React, typescript, Rust etc...
The subgoals was to learn Rust as well as i'm fully novice with this language but want to learn it.

I've created an alias for myself to create the folder for me without having to think of the name which curiously takes me some times even if for sandbox project it is kind of useless.

```
alias mkdirgn='TMP_VAR=$(gen_name) && mkdir $TMP_VAR && cd $TMP_VAR && echo "New project folder : $TMP_VAR"'
```
