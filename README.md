#sha-rust

## About
Implementation of the sha1 algorithm in rust - my first rust program :)

This was a way to learn more about sha1 and to make a real program using rust.
Sha1 uses lots of bit twiddling and does just enough input, output, looping and
memory allocation to be a good first program.

## Usage
Send data to stdin and the sha program will print the sha1 hash on std out:

    $ echo -n "What is the sha1 hash of this string?" | ./sha
    4810253f49ccefe8804a3ba76efe0acc8a4ca9fe


## Notes
I learned a lot about rust writing this, and there are a few things I really
love:

- Built in testing with #[test], this made developing the code and finding bugs
  so easy
- Built in bit maniplation is great.  Operators like ^, & and | work just like
  you would expect, but there are also things like overflowing_add, and
  rotate_left built in to primative integer types that make twiddling easy.

There are also several things about rust that I struggled with:
- str vs String 
- Slices and their sizes. What is checked at compile time? Are slice range
  lengths?
- overflow_add returns a tuple, which makes it awkward to chain together
- How many allocations am I really doing?  I'm still not clear on what ends up
  on the stack vs what is dynamically allocated with a vec. I ended up
  refactoring the program at the end so that there is basically one big
  allocation at the beginning to store the incoming message and I think that is
  the only alloc, but I'm not really sure.
- Macros - I don't yet have a sense for when I should expect something to be a
  macro vs expecting it to be in a library. Does concat! only work wtih static
  strings?  Why are some thing macros instead of library functions?
- Weird to me that sometimes mut applies to the binding, and sometimes to the
  type.  Not always intuitive to me where to put the 'mut' keyword.  I think mut
  never applies to the type, but goes along with it in function signatures.(?)
- I though slices and ranges  would work like in python.  I enjoy having these
  features in a lower level language, but didn't find the usage as intuitive. 

## Performance
Performance of this code is interesting.  It seems to be a lot faster than
shasum distributed with OSX on smaller files, but slower on larger files (larger
than 150MB). 

```
$ du -h ~/Downloads/sintel.mp4 
123M	/Users/nathan/Downloads/sintel.mp4
$ time cat ~/Downloads/sintel.mp4 | ./target/release/sha 
50f1c70eeabbe3445d11a1c602c7851648156c2d

real	0m1.325s
user	0m1.216s
sys	0m0.220s
$ time cat ~/Downloads/sintel.mp4 | shasum
50f1c70eeabbe3445d11a1c602c7851648156c2d  -

real	0m0.746s
user	0m0.695s
sys	0m0.181s

$ time echo -n "What is my sha1 message digest?" | ./target/release/sha 
3255d4afb6e4fcd2392d1da6b8105f8cb2ca3dda

real	0m0.010s
user	0m0.003s
sys	0m0.006s
$ time echo -n "What is my sha1 message digest?" | shasum
3255d4afb6e4fcd2392d1da6b8105f8cb2ca3dda  -

real	0m0.056s
user	0m0.041s
sys	0m0.012s
```
