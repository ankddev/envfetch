# Source code for the VHS envfetch example.
# YOU NEED TO GLOBALLY INSTALL ENVFETCH FROM SOURCE CODE TO GENERATE RECORDING
#
# To run:
#
#     vhs assets/envfetch.tape

Output assets/envfetch.gif
Output assets/envfetch.ascii

Require envfetch

Set FontSize 18
Set Width 900
Set Height 800

Hide
Type `export PS1="> " && source ~/.bashrc` Enter
Show

# Basic set
Type `envfetch set MY_VAR_TEST "Hello for everyone" -- echo hello` Sleep 500ms  Enter
Sleep 500ms
Type "envfetch get MY_VAR_TEST" Sleep 500ms  Enter
Sleep 500ms
# Global set
Type `sudo -E envfetch set MY_VAR_TEST "Hello for everyone" --global` Sleep 500ms  Enter
Sleep 500ms
Type "source ~/.bashrc" Enter
# Get with wrong name
Type "envfetch get MY_VAR_TESTT" Sleep 500ms  Enter
Sleep 500ms
# Get with normal name
Type "envfetch get MY_VAR_TEST" Sleep 500ms  Enter
Sleep 500ms
# Add globally
Type "sudo -E envfetch add MY_VAR_TEST test --global" Sleep 500ms  Enter
Sleep 500ms
Type "source ~/.bashrc" Enter
# Remove globally
Type "sudo -E envfetch delete MY_VAR_TEST --global" Sleep 500ms  Enter
Sleep 500ms
# Load
Type "envfetch get HELLO_VAR" Sleep 500ms  Enter
Sleep 500ms
Type `sudo -E envfetch load --global --file "assets/.env.example"` Sleep 500ms  Enter
Sleep 500ms
Type "source ~/.bashrc" Enter
Type "envfetch get HELLO_VAR" Sleep 500ms  Enter
Sleep 500ms
Screenshot "assets/main.png"
# Print list of variables
Type "envfetch print" Sleep 500ms  Enter
Sleep 500ms
# Interactive mode
Type "envfetch interactive" Sleep 500ms  Enter
Sleep 2s
Down@2
Sleep 600ms
Ctrl+r
Sleep 1s
Down
Down
Down
Sleep 600ms
Up
Sleep 500ms
Screenshot "assets/interactive_main.png"
Type "a"
Screenshot "assets/interactive_add.png"
Escape
Ctrl+q

Sleep 5s
