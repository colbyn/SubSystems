# Writing Assignment - Companion Piece Commentary

Given the project instructions:

> In addition to your Final Research Paper, you will create a Companion Piece that repurposes or remixes the argument of your essay in a different genre/medium. The purpose of this additional assignment is to get you thinking about how to present your argument in a new way and for a non-scholarly audience. 

My audience herein are programmers.

Why this is relevant, is because I plan on pursuing my aspirations in analog computing, and this software marks the beginnings of such. Furthermore, in my essay, I frequently remarked about the role of software in analog computing. Even going as far as saying:

> I believe that accessibility benefits everyone, just as much as I believe that reducing the barrier to entry in engineering disciplines, results in more innovation, and more sophisticated products therefrom. In this regard, software is perhaps the most accessible, because you don’t need a lab stocked with expensive ad hoc equipment. But in this case, perhaps, access to some cloud based compute infrastructure that affords easy and cheap access to the more specialized analog computing hardware.
>
> Furthermore, humans are utterly unrivaled when it comes to natures ability to self assemble into sophisticated structures (including ourselves). That is, we can plant a seed, and grow a tree, but planting a seed to grow a house is unfathomable. Just as unfathomable as designing robots to kill cancer cells, which again, nature has parallels. Presumably this is due to complex factors that may be abstracted within my proposed computer architecture. (Remember that aforementioned isomorphism?)
>
> When we abstract, we may sometimes reduce ‘complex’ things into simpler, more conceptually ‘discrete’ things. Which therein, due to this simpler nature, may further permit others to effectively build upon such, and therein, may create something more sophisticated, perhaps even, greater than the sum of it’s components.
>
> That is, a system where discrete units build upon other discrete units and therein produce more complex non-discrete units. This system permits for abstraction, so complex non-discrete units may be abstracted into simple discrete units. Thereafter this process of production and abstraction enables further production and abstraction and so forth. Each iteration or generation may be considered to be more sophisticated than prior generations, given that each generation is a product of prior generations... From this analogy, you can imagine these bottom-up and cumulative processes will eventually give rise to very sophisticated products, and perhaps one day, akin to how emergence gives rise to the complexity found in nature.
>
> From personal experience, my (https://imager.io)[https://imager.io] project wouldn’t be possible without the various open source components it’s built upon. Simply because my time is finite, and especially because lower-level encoding details are just too complicated for me to understand and implement on my own. I am nevertheless able to compose such components into a larger and more sophisticated end product.
>
> Just as the industrial revolution introduced a force multiplier of human muscle, so too does abstraction introduce a force multiplier of the human mind. What we need now, as I have argued, is a paradigm shift in amplifying our ability to rival the sophistication we see in nature.

# Overview of SubSystems

This began it's life with the aspirations of creating software that can solve chemistry problems, originally I referred to this as [chem-bot](https://github.com/colbyn/chem-bot). Over time, I ended up implementing the beginnings of a computer algebra system. Some notable components include the following. 

| Path| |
|-----|--|
| [src/ast/expr_parser.rs](https://github.com/colbyn/SubSystems/blob/main/compiler/src/ast/expr_parser.rs)| The parser for the frontend language. |
| [src/ast/expr.rs](https://github.com/colbyn/SubSystems/blob/main/compiler/src/ast/expr.rs)| Something akin to a computer algebra system. |
| [src/ast/funs.rs](https://github.com/colbyn/SubSystems/blob/main/compiler/src/ast/funs.rs)| My ad-hoc implementation of a computer algebra rewrite or transformation system. |

The code in `src/ast/expr.rs` was particular challenging, since I had to figure out how to implement basic processes that we take for granted. Such as simplifying fractions and cancelling common factors, which includes algebraic expressions (such as being able to simplify `1/(1/x) = x` and `(ab)/a = b`).

Likewise, the code under `src/ast/funs.rs` implements an entire DSL for defining bottom-up transformations of tree like data structures. It's where this sorta functionality `mole(energy(photon(wavelength = nm(325))))` is implemented. 

Further still, `src/ast/expr.rs` in conjunction with `src/ast/funs.rs` implements higher level ideas such as simplifying units in dimensional analysis problems. Such as as how ㎐ and seconds cancel out.

## Change of Direction

Recently, I discovered this project called [egison](https://www.egison.org), which implements my ideas in a much more elegant manner. It's a DSL where you can express rewrite rules for transforming abstract syntax trees. Specially, non-linear pattern matching with backtracking for non-free data types. Which is perfect for my use. 

The only issue is that [egison](https://www.egison.org) is implemented in Haskell, which is a language I'm comfortable with, but Rust is easier to integrate into projects, and supports compiling to the web VIA WebAssembly. So my game plan is to drop my preexisting implementation of SubScript and reimplement [egison](https://www.egison.org) in a manner that better suits my preferences. 

Overall, the most frequently cited issue of analog computing seems to pertain to both programmability issues and supporting software ecosystem, which I plan on accommodating. 

My long term aspirations is to begin with a business model founded upon offering solutions to computational chemistry and biology fields. Which, according to Grand View Research, Inc, is expected to reach $13.6 billion, by 2026. Specially, I'm imagining a cloud based business model focused on hardware accelerators optimized for various application specific workloads. Where my aforementioned language will serve as the frontend to such.

