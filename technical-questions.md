# Given a task, when (and why) would you use a programming languages such as Rust, Go, C++, etc… versus a scripting language such as Python, Ruby or even Bash ?

I'm very much treating this question as an opinion piece so prepare for opinions!!!

I think the question itself is wrong or perhaps dated in it's nature. The question Creates two categories "compiled languages" and "scripting languages". I believe this distinction is pretty irrelevant when it comes to deciding what language to use in the modern day.

One of the biggest traditional arguments for using a scripting language is that they are faster to develop/prototype in. While this was true in the past, I believe modern compiled languages have more than caught up in how ergonomic they feel and how fast they are to develop in.

Meanwhile one of the biggest arguments against scripting language is that they are slow. While this still holds true, I'd say the gap has narrowed. Not only because scripting languages have gotten faster, but because compiled languages have slowed down.

For example, JavaScript keeps gaining performance with *insane* optimisations and different JIT compilers based on the work load. Lua has a JIT compiler that's blazing fast. And python... well python is still kind of slow

Then on the compiled language side we have Go. While Go still outperforms scripting languages it's still quite a slow language in comparison to other compiled ones. This doesn't make Go a bad language, in fact I think it's quite a good language, accomplishing its goals rather well. Those goals being ease of development over execution speed. Which as I mentioned above, is typically what scripting languages do.


This is why I think the distinction between scripting and compiled is meaningless and you should instead pick your language on other factors. Luckily I find the choice is rather straight forward as Each language tends to fall into a niche.

So lets talk when and why you'd use each language! Prepare for more opinions!

Most languages are general purpose and are an okay fit for most tasks. Though generally scripting languages tend to be weakly typed making them a bad choice for a large projects. Web devs have accepted this fact and created TypeScript! Meanwhile, languages like Rust and Go are strongly typed while being as high level and easy to use as scripting languages. This makes them good for large projects that need to be maintained over long periods of time. As well as small one off "scripts". So you may ask, why would I use a scripting languages over a compiled one? My answer in the general case is. There's no reason to!

Now, the general case aside there are specific cases where you'd want to pick specific compiled or scripting languages.

C, C++ and Rust are all systems languages. This makes them good for three things. Writing bare metal code, low level code that makes use of OS features directly, working with C ABI libraries.

Lua has a rather special purpose of being embedded into other applications and doesn't have much serious usage outside of that.

Bash is incredible at stringing commands together and a no brainer if your program is mostly just calling other binaries and piping the data around.

Python is very easy to learn, has has a massive library ecosystem and is heavily used in the fields of AI and data science. This makes it a great choice for scientists as it's easy to pick up and all the functionality they want is probably prewritten.

JS dominates the web so it may seem like a no brainer for anything you want to run in a browser, however it's not the only option any more. TypeScript is a good familiar replacement. Other languages such as Dart naively compile to JS. And the emergence of web assembly has allowed given Rust, Go, C and so many more languages the ability to run in a browser while also staying performant.

So to answer the original question, I wouldn't use a scripting language as they are generally weakly typed and have been superseded by modern languages like Go and Rust. I may however, decided to use Python, Lua or Js in specific situation where those languages make the most sense.

# What are the 10 last repos/projects you worked on (no forks) what choices did you make there and why ?

## Paru

https://github.com/Morganamilo/paru

Paru is a package manager for the Arch Linux User Repository.

This was a rewrite of a similar tool written in Go. A lot of the decisions here involve not doing things. The original go code acquired a lot of feature creep and was quite a mess. Paru's goal was to rebuild from the ground up, not including unnecessary features that were included previously.

As the Go code grew organically, many things that should have been refactored into separate modules were not. This lead to a lot of code being dependent on each other and not very well separated. This is why in Paru making good abstractions and separating code where possible was a very important goal.

## terraform-provider-ably

https://github.com/ably/terraform-provider-ably

terraform-provider-ably was a terraform provider for the Ably control API.

This project was the first time I had used generics in Go. As some one who learnt Go a long time before generics were introduced, post release they were never something that came to mind.

However, in this project, the repetitive code was becoming an issue, and the interface system just wasn't enough for what we wanted to do. So I made the decision to refactor the project and switch it over to generics. This cut down the LOC we had by half, while leaving the existing code easier to read and nicer to use.

## alpm.rs

https://github.com/archlinux/alpm.rs

alpm.rs is Rust bindings for libalpm, a C package management library for Arch Linux.

When building alpm.rs I had a few core design tenants that I considered to be essential for the project to be a success.

- The bindings are 100% safe and hard to misuse
- The bindings are as ergonomic to use as any other rust library.
- The bindings do not introduce any performance overhead where possible
- The bindings expose 100% of the API surface of libalpm
- The bindings do not add features not present in libalpm.

Bindings being safe and hard to misuse is typically a given when it comes to Rust. However libalpm can be quite an awkward library to use; due to its lack of documentation (though it's better now), use of void pointers, mixing of data that you do and don't need to free as well as other memory management woes. Even as some one who knows the library very well, libalpm can just feel painful to use sometimes. Because of this I made a really conscious effort to focus on the ergonomics during development.

This is the main reason I choose Rust for this project. Rust is the only language I know that's extremely ergonomic to use while also offering great low level access for things such as calling C functions and pointer manipulation. Rust's excellent type system and RAII principles also allow for the memory management to be effectively automated away without introducing the overhead a garbage collector would bring.

Exposing 100% of the API surface may seem like a trivial point but I think it's a very important one. While why I want to cover the entire library does not need any explanation, the reason to explicitly keep this in mind is to make sure you develop a framework that works for the library as a whole. As an example, part way through the development of alpm.rs, it turned out one function wanted exclusive access to the main handle while also borrowing some package data. This was quite incompatible with the type system I had been building thus far and required a lot of reworking code to get it to fit. If this was accounted for already then the code could have been done right the first time.

The reasoning for not adding any extra features to alpm.rs is rather simple. alpm.rs is meant to be 1:1 with libalpm. Anything that could be added to alpm.rs and be useful would probably also be useful to the users of libalpm and other language bindings. So while it is more of a pain and does take more time. Features should be implemented in libalpm first and trickle down. For the things that only make sense on the rust side but don't fit in alpm.rs there is a separate alpm-utils crate. This crate mostly just contains utilities that cover common patterns.

## aur-depends

https://github.com/Morganamilo/aur-depends

aur-depends is the dependency resolver for Paru.

It was a very explicit choice to make this its own repo and not just module. The dependency resolver is a very large piece of Paru, making it its own library allows for clear separation of API boundaries. This means when developing for Paru we can ignore the actual implementation and only focus on the publicly exposed ABI. It also allows for much greater degree of testability as the dependency resolver does not reply on any state from Paru.
## srcinfo.rs

https://github.com/Morganamilo/srcinfo.rs

srcinfo.rs is a parser for .SRCINFO files.

srcinfo is a weird format, it's generally a list of keys and values, but certain keys are special and treated differently from others. The format also needs some post processing for the data in it to be useful.

The decision here is should the parser give you a struct that represents the file as is? or should it do the post processing for you? I decided on the latter as it's the more useful option and how you want to work with the data most of the time. It could be argued that the former gives you more control and you could even implement the latter version on top of the former, but I simply never found any use for the raw version of the file.

## pacmanconf.rs

https://github.com/Morganamilo/pacmanconf.rs

pacmanconf.rs is a library that parses pacman config files.

The most notable decision here was to split the library into 2 libraries. A generic ini parser and the pacman specific parser. Splitting things up into more parts is generally a good thing by itself as it increases maintainability and allows for more code reuse. In this case, I did have plans to write some other config files down the line using pacman's flavour of ini. The generic ini parser ended up being very handy for this.

Another decision I made, one that may seem quite weird, is to have pacmanconf.rs internally call pacman-conf, a binary that parses the pacman config file and prints the results. This may seem silly, if pacman-conf already parses the config file then why do we parse it again? The simple answer is that pacman-conf is a binary only, but even if it did expose a library component, parsing an ini file is a simple enough task that writing the parser in Rust would still be a better choice than wrapping a C library. But this doesn't explain why we don't just parse the file from scratch instead of calling the binary. This is because pacman-conf ends up normalising the config file when it prints it back out, vastly simplifying the code needed for the Rust parser. And no one is going to miss a few milliseconds here and there.

## Steam-Discord-Bot

https://github.com/Morganamilo/Steam-Discord-Bot

Steam Discord Bot was a bot to bridge the chats of Discord and Steam together.

I decided to write this project in JavaScript despite having never used it before. This was because JavaScript had very good libraries for both Steam and Discord and learning a new language seemed like a better choice than writing said libraries my self.

## ably-rust

https://github.com/ably/ably-rust

ably-rust was a Rust implementation of the Ably client library.

Ably is a realtime messaging service that works via websockets. One important consideration designing this library was how the internal state of the program would be managed. As the library primarily makes web requests, the program was inevitably going to be async and need a lot of state management. And as rust has strict borrowing rules, implementing this in rust can be kind of tricky.

The way I decided to approach this problem was to make it as easy as possible for the user. This meant using reference counting and interor mutability to abstract all the state management from the user. However this does mean that Rust's guarintees are lessened and have to be manually accounted for during development.

## Cube

https://github.com/Morganamilo/cube

Cube was a project I started to teach myself graphics APIs. Originally I opted for C++ and Vulkan. As this was a learning exercise and I did not know Rust at this point in time, C++ seemed the only sane choice. Especially since it was the defacto standard which made learning resources easier to come by. My choice for Vulkan was essentially motivated by it being the cool new kid on the block and it being the successor to OpenGL. I thought it would be better to learn the new tech instead of the old.

I wrote a simple program and shader to render a cube from a hardcoded array of vertices.

Vulkan turned out to be very verbose and fiddly and at the time lacked good learning resources. Because of this I decided learning Vulkan was a bigger time investment than I was prepared for currently and started over using OpenGL instead.

After converting my existing cube code to OpenGL, I decided to work on a simple game engine modelled after Unity's component system with a handful of builtin components to handle movement and the camera.

During this, I had also begun learning Rust and had fallen in love with the language. So as a learning exercise I then converted the game existing code over to Rust.

Finally, I build a Rubix cube simulator on top of the game engine and it all went rather smoothly.

There weren't many technical decisions made during this project as it was for education and fun. Most decisions came down to what I thought would be interesting to create while remaining  doable in a reasonable time frame.

## aur-fetch.rs

https://github.com/Morganamilo/aur-fetch.rs

aur-fetch is a library for downloading and reviewing package from the Arch User Repositories. With this library I decided to be very opinionated and enforce a fixed work flow of downloading packages, then managing their state with some git hackery.

Normally when it comes to libraries, I try to be as general as possible to allow the most flexibility for users. However in this case, being more general lead to more flexible but less pleasant to use solution. The package review process is also a security measure that needs to be done and not making it a compulsory part of the library may lead to users ignoring this measure or implementing it incorrectly.

