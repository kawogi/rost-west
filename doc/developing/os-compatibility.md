OS/library compatibility policy
===============================

This document describes how we decide which minimum versions of operating systems, C++ standards,
libraries, build tools (CMake) or compilers Luanti requires.

Most important is that we do not increase our minimum requirements without a reason or use case.
A reason can be as simple as "cleaning up legacy support code", but it needs to exist.

As most development happens on Linux the first measure is to check the version of the component in question on:
* the oldest still-supported **Ubuntu** (End of Standard Support)
* the oldest still-supported **Debian** (*not* LTS)
* optional: the second newest **RHEL (derivative)**

Generally this leads to versions about 5 years old and works as a reasonable result for BSDs and other platforms too.

Needless to say that any new requirements need to work on our other platforms too, as listed below.

## Other parts

**Compilers**: gcc, clang

We require **OpenGL** 2.0 or ES 2.0, so shaders can be relied on.
Graphics code should generally work on both. Newer features can be used as long as a fallback exists.

General **system requirements** are not bounded either.
Being able to play Luanti on a recent low-end phone is a reasonable target.

## On totality

These rules are not absolute and there can be exceptions.

But consider how much trouble it would be to chase down a new version of a component on an old distro:
* C++ standard library: probably impossible without breaking your system(?)
* compiler: very annoying
* CMake: somewhat annoying
* some ordinary library: reasonably easy

The rules can be seen more relaxed for optional dependencies, but remember to be reasonable.
Sound is optional at build-time but nobody would call an engine build without sound complete.

In general also consider:
* Is the proposition important enough to warrant a new dependency?
* Can we make it easier for users to build the library together with Luanti?
* Maybe even vendor the library?
* Or could the engine include a transparent fallback implementation?

## A concrete example

(as of April 2024)

```
Situation: someone wants C++20 to use std::span

gcc with libstdc++ 10 or later
clang with libc++ 7 or later (note: no mainstream Linux distros use this)

Debian 11 has libstdc++ 10
Ubuntu 20.04 LTS has libstdc++ 9
(optional) Rocky Linux 8 has libstdc++ 8

Verdict: not possible. maybe next year.

Possible alternative: use a library that provides a polyfill for std::span
```

## Links

* Ubuntu support table: https://wiki.ubuntu.com/Releases
* Debian support table: https://wiki.debian.org/LTS
* Release table of a RHEL derivative: https://en.wikipedia.org/wiki/AlmaLinux#Releases
* C++ standard support information: https://en.cppreference.com/w/cpp/compiler_support
* Distribution-independent package search: https://repology.org/ or https://pkgs.org/
