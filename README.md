# Christer Carlsson Compiler (CCC)



A hobby project to explore and learn about compiler design and implementation using Rust. This project serves as my learning journey into both compiler theory and the Rust programming language.



## About



CCC (Christer Carlsson Compiler) is a personal educational project where I document my journey of building a compiler from scratch. As someone interested in understanding how compilers work under the hood, this project allows me to dive deep into concepts like lexical analysis, parsing, semantic analysis, and code generation.



## Goals



- Learn and understand compiler design principles

- Gain practical experience with Rust programming

- Document the learning process for others interested in similar projects



## Development Approach



In this learning journey, I'm utilizing modern AI tools to assist in development and learning:



- GitHub Copilot for code suggestions and pair programming

- Microsoft Copilot for conceptual understanding and problem-solving



I believe in being transparent about using these tools as they are valuable aids in modern software development and learning.



## License



This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Feel free to use, modify, and distribute the code as you see fit.



## Status



This is a work in progress. Updates and improvements will be made as I continue learning and implementing new features.


## Build

```bash
> cargo run >> output.asm
> nasm -f elf64 -o output.o output.asm
> gcc -nostartfiles -nostdlib -no-pie -o a.out output.o

