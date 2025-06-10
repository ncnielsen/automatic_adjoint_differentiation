An implementation of Automatic Adjoint Differentiation (AAD) in the programming language Rust.

There are plenty of books on this subject. A particularly good book is "Evaluating Derivatives: Principles and Techniques of Algorithmic Differentiation, Second Edition"
by authors Andreas Griewank and Andrea Walther (see https://epubs.siam.org/doi/book/10.1137/1.9780898717761)

Another good book is Modern Computational Finance by author Antoine Savine. The example used in this codes' main function is taken directly from this book.
(see https://antoinesavine.com/books-by-antoine-savine/)

AAD is a method for automatically evaluating derivatives for any given (smooth) mathematical expression. Other methods include finite differences such as forward, backward and centered difference which are all based on Taylor Series expansions.
AAD is not based on Tailor Series expansion. Instead, the idea is taking any given (smooth) mathematical expression and construct an abstract-syntax-tree (AST) structure by breaking down the expression into its basic operations, say +, -, /, *, ln(x), sqrt, e^(x) etc.

The idea is that for each of the basic operations, the derivative is known analytically, and this can be used to propagate derivative calculations (called adjoints) backwards through the AST, from output towards the input. The is made possible by chaining the basic operations together using the chain rule of calculus for which each basic operation also has a chain-rule, say sum-rule, product-rule, quotient-rule etc.

When the process completes, the dInput/dOutput has been calculated for each input. In addition, the sensitivity/derivative of all intermediary operations have also been calculated with respect to the output.

While there are plenty of implementations of AAD in C++ and some in Rust, almost every implementation of AAD and every book on this subject assumes that functions are scalar-functions. That is they assume that the user is working on classical functions such as those prevalent in Finance (for example for calculating the price of an option). This is also the case for the two reference books I have provided.

However, for most scientific purposes, relationships are not expressed naturally as scalar functions. Instead, many scientific relationships are expressed using Vectors or One-forms (sometimes called Dual Vectors). More generally, scientific relationships are expressed in Tensors, where Tensors of rank 0 are scalars, Tensors of Rank 1,0 are Vectors and Tensors of Rank 0,1 are One-forms. But many important scientific relationships are expressed in general Tensors of rank m,n. Most notably Maxwells equations of electromagnetism in its covariant form is a (single) general Tensor equation. Another prominent example is Einsteins Field equations which can basically only be expressed as a general Tensor equation.

The long-term goal of this library, is to provide an implementation in Rust of Automatic Adjoint Differentiation that operates on general m,n Tensors.

There are plenty of books on Tensor Algebra (sometimes called multilinear algebra) and Tensor Calculus. These books provide the mathematical foundation for the implementation. The primary pieces to this puzzle is of-course the chain-rule for Tensor calculus and the derivatives (sometimes called Covariant derivatives) for each basic Tensor operation in Tensor Algebra.

There are plenty of commercially available tools - especially for engineering-purposes, that probably run on a closed-source implement of AAD for m,n Tensors. But there are very few open source libraries. 

There are two underlying principles for this implementation:
1) Make it work, then make it fast
2) Use functional language principles to increase readability


