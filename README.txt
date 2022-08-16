=== Algebrisk ===

Usage
-----
In order to launch Algebrisk input bar, you can use the default keyboard
shortcut of [Win+`]. You can also launch it from the settings page, which
can be accessed by clicking the Algebrisk icon in your systray. Once launched,
type in an algebraic expression to see the solution. Algebrisk can then be
closed by hitting Escape button, or by hitting Enter. Hitting Enter will also
copy the result of your expression to your clipboard. You can view recently
entered expressions by using the up and down arrows when Algebrisk is open.

Supported Algebraic Expressions
-------------------------------
We use the Kalk library (https://github.com/PaddiM8/kalker) as our algebraic
engine. This library supports a plethora of arithmetic operations such as:
  Operators: +, -, *, /, !, %, ^
  Functions: sin, cos, tan, sqrt, ceil, floor, pi, integrate, min, max, log
  Statements: Multiple arithmetic statements separated by semicolons (;)
  Equations: f(x) = x*2; A(x) = { f(x) + A(x-1) if x > 1; 0 otherwise }; A(10)

For a more detailed list of supported features and functions, please read:
https://raw.githubusercontent.com/PaddiM8/kalker/v2.0.0/cli/help.txt

Disabling Startup on Login
--------------------------
By default, Algebrisk will start a background process when you log in so that
it can listen for your keyboard shortcuts. To stop this background process from
running, you can click the "Quit Algebrisk" button in the settings, and the
program will not open via keyboard shortcut until launched again. After you
quit the program, open up Task Manager [Ctrl+Shift+Escape] and navigate to the
Startup tab. Find the Algebrisk entry, right click on it, and click Disable.
This will stop Algebrisk from running when you log in, and you will need to
launch the program manually.

Contact
-------
If you experience any errors or bugs, you can send them to nhilton777@gmail.com

Thank you!