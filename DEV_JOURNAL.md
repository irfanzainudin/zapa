# Zapa's Development Journal

To document the developmental journey of Zapa and also my own notes and thoughts as I read "Compilers: Principles, Techniques, and Tools" by Alfred V. Aho, Monica S. Lam, Ravi Sethi, and Jeffrey D. Ullman (the Purple Dragon book).

## 30th May 2024

- I started reading the Purple Dragon book. Wish me luck!

### Section 1.1 - Language Processors

- As I began reading, I can't help but thought that this is the learning method of previous generations. They had to read and study the numerous pages of books, flipping away thousands of pages of information. They had to do it the "hard way" whereas with the invention of Google, any information is only a click away. However, in trading off efficiency, the older generations gained focus and depth whereas with Google, you can get distracted with the ads and what have you (irrelevant information, clickbaits etc.). This is only worsen with the advent of Large Language Models or AI as normies call it. With reading books, you are at the disposal of the authors, however with reading great books, you'll learned a lot more from great authors that distill their knowledge into a book. Hence, focus and depth. May Allah make me steadfast into completing this book.

**Exercise for Section 1.1**

1. What is the difference between a compiler and an interpreter?

- They're both language processors
- A compiler translates a source language into a target language. It also reports any errors in the source program that it detects during the translation process.
- An interpreter does the same thing, but also processes the inputs supplied by the users to produce outputs

2. What are the advantages of (a) a compiler over an interpreter (b) an interpreter over a compiler?

- (a) Programs produced by compilers are usually much faster
- (b) Interpreted programs are better at error diagnostics

3. What advantages are there to a language-processing system in which the compiler pro duces assembly language rather than machine language?

- "because assembly language is easier to produce as output and is easier to debug"

4. A compiler that translates a high-level language into another high-level language is called a source-to-source translator. What advantages are there to using C as a target language for a compiler?

- I would say portability. Because C was (and probably still is) a really popular language, it supported a lot of computer architectures and platforms.

5. Describe some of the tasks that an assembler needs to perform.

- produces relocatable machine code as its output

### Section 1.2 - The Structure of a Compiler

**Front-end = Analysis**

- This is where tokens are extracted from the source code
- Also, where syntax errors are reported
- Intermediate representation is created at this end
- "Symbol table" is created here as well
