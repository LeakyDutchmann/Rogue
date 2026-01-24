1. I recommend against wildcard imports (`*`). You will not be able to see 
what you have imported from where (if you have more than one import).
I read a symbol, and I don't know - is this your code thing or an 
external library thing, and which library?

2. I also recommend against code in `mod.rs`, but it might be more of a taste thing.
You can reexport symbols from `mod.rs` with `pub use`, if you want them in this
particular module.

3. In one module, try to declare things before they are used, unlike 
`AnimationId` in `animations/mod.rs`.

4. Formatting. Please be consistent with newlines. One between peer functions, maybe
two between different groups of functions/structs.
Do not leave trailing spaces or more than 2 empty lines anywhere, it
looks dirty.
Trust the autoformatter, but not too much.

5. Instead of xy_idx(), you may introduce a Point/IVec2 type,
and add API to WorldMap to access it by a Point. It 
will simplify a lot of code. (I'm reading the `cave_generation.rs`)
For example, `tile.position.x == x && tile.position.y == y`
would be `tile.position == pos`.

6. I see a lot of code that can be moved into methods. The
criterion is: if the code only works with one type's data - 
then there's a definitely method buried there. If it works 
with one type's data and a bit more - it still might
or might not be a method with extra arguments, copied or referenced.

7. Eliminate warnings. You can leave a few unused fields if you are working
on completing the code right now, but otherwise eliminate them.
You may miss something important in the warnings if you are flooded
by them.
