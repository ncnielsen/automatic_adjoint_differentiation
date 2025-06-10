An implemenation of Automatic Adjoint Differentiation (AAD) in the programming lanugage Rust.

There are plenty of books on this subject. A particularly good book is "Evaluating Derivatives: Principles and Techniques of Algorithmic Differentiation, Second Edition"
by authors Andreas Griewank and Andrea Walther (see https://epubs.siam.org/doi/book/10.1137/1.9780898717761)

Another good book is Modern Computational Finance by author Antoine Savine. The example used in this codes main function is taken straight from this book.
(see https://antoinesavine.com/books-by-antoine-savine/)

AAD is a method for automatically evaluating derivatives for any given (smooth) mathematical expression. Other methods include finite differences such as forward, backward and centered difference which are all based on Taylor Series expansions.
AAD is not based on Tailor Series expansion. Instead the idea is takes any given (smooth) mathematical expression and constuct an abstract-syntax-tree (AST) structure by breaking down the expression into its basic operations, say +, -, /, *, ln(x), sqrt, e^(x) etc.

The idea is that for each of the basic operations, the derivative is known analytically, and this can be used to propagate derivative calculations (called adjoints) backwards through the AST, from output towards the input. The is made possible by chaining the basic operations toghether using the chain rule of calculus for which each basic operation also has a chain-rule, say sum-rule, product-rule, quotient-rule etc.

When the process completes, the dInput/dOutput has been calulated for each input. In addition the sensitivity/derivative of all intermediary operations have also been calcuated with respect to the output.
