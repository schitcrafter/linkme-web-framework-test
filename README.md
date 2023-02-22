# linkme web framework test

The crate at the root is the user crate, and the mylinkmemacros and mylinkmetypes are the library crates.

## What is this?

I wanted to see if it is possible to use the `linkme` crate in order to automatically
register http handlers.

## Usage

In order to use the library, you need to add linkme and the mylinkmetypes crate as dependencies.
(it might be possible to not have to depend on linkme in the user crate, but I haven't really tried it)

Then, just annotate the appropriate handler function with `#[get]`,
and call `execute_handlers()`.