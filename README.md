An implementation of Automatic Adjoint Differentiation (AAD) in the programming language Rust.

This project is part of a larger ambition by Numerical Solution to provide real contributions to the scientific endevor. See more on www.NumericalSolution.Com

There are plenty of books on this subject. A particularly good book is "Evaluating Derivatives: Principles and Techniques of Algorithmic Differentiation, Second Edition"
by authors Andreas Griewank and Andrea Walther (see https://epubs.siam.org/doi/book/10.1137/1.9780898717761). The example used in this book to illustrate the concepts is also used as a test case in this code.

AAD is a method for automatically evaluating derivatives for any given (smooth) mathematical expression. Other methods include finite differences such as forward, backward and centered difference which are all based on Taylor Series expansions.
AAD is not based on Taylor Series expansion. Instead, the idea takes any given mathematical expression, f,  and construct an abstract-syntax-tree (AST) structure by breaking down the expression into its basic operations, say +, -, /, *, ln(x), sqrt, e^(x) etc.

The idea is that for each of the basic operations, the derivative is known analytically, and this can be used to propagate derivative calculations (called adjoints) backwards through the AST, from output towards the input. This is made possible by chaining the basic operations together using the chain rule of calculus and the derivative rules i.e. the sum-rule, product-rule, quotient-rule etc.

When the process completes df/dx1, df/dx2, ..., df/dxn has has been calculated for each input, 1 to n. In addition, the derivatives of all intermediary operations have been calculated with respect to f.

While there are plenty of implementations of AAD in C++ and some in Rust, almost every implementation of AAD and every book on this subject assumes that functions are scalar-functions. That is they assume that the user is working on classical functions such as those prevalent in Finance (for example for calculating the price of an option). This is also the case for the reference book listed above.

However, for most scientific purposes, relationships are not expressed naturally as scalar functions. Instead, many scientific relationships are expressed using Vectors or One-forms (sometimes called Dual Vectors). More generally, scientific relationships are expressed in Tensors, where Tensors of rank 0 are scalars, Tensors of Rank (1,0) are Vectors and Tensors of Rank (0,1) are One-forms. But many important scientific relationships are expressed in general Tensors of rank (m, n). Most notably Maxwells equations of electromagnetism in its covariant form is a (single) general Tensor equation. Another prominent example is Einsteins Field equations which, in its expanded form, consists of 16 partial differential equations that must all be true simultaneously. Therefore, it is quite unwieldy unless expressed as a general Tensor equation in which case it boils down to a single equation.

The long-term goal of this library, is to provide an implementation in Rust of Automatic Adjoint Differentiation that operates on general (m, n) Tensors.

There are plenty of books on Tensor Algebra (sometimes called multilinear algebra) and Tensor Calculus. These books provide the mathematical foundation for the implementation. The primary pieces to this puzzle is of-course the chain-rule for Tensor calculus and the derivatives (sometimes called Covariant derivatives) for each basic Tensor operation in Tensor Algebra.

There also exists a multitude of commercially available tools - especially for engineering-purposes, that probably run on a closed-source implementations of AAD for (m, n) Tensors. But there are very few open source libraries.

Should you wish to join this effort and contribute code, the project builds on the following principles:

1) First make it work, then make it fast. Premature optimization can lay any good code base in ruins before it has even been completed.

2) Use functional language principles wherever possible, to increase readability

3) Iterations, iterations, iterations. Every great project started out small. While the vision for this project is both grand and ambitious, we take one step at a time. We acknowledge that we have much to learn, and we plan on learning by doing.

4) Free and Open Source Software. This software is MIT Licensed granting a wide range of permission with very few restrictions and high license compatibility.
