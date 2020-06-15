# unsafe_features_example

When to our Rust project we add a third party dependency, we would like to know if that dependency does something to files in the file system.  
The compiler should protect us from dependencies that want to read/write files without our knowledge.  
This example will try to demonstrate how access to files could be controlled with "unsafe feature".  
"Unsafe feature" is a new concept, it is a pre-pre-RFC:  
<https://internals.rust-lang.org/t/crate-capability-lists/8933/2?u=bascule>  
that could be used to whitelist usage of unsafe in dependencies.  

First requirement - every unsafe code must be wrapped in an "unsafe feature".  
The compiler must enforce this for every module that uses unsafe. I don't know how, but looks possible.  
Also the standard library contains unsafe and most of it should be under "unsafe feature".  
File access in std::fs is also unsafe. Today we don't see std:fs as unsafe, because of the standard library, but inside it is.  
Therefore also std::fs must be encapsulated in an "unsafe feature".  

By default every "unsafe feature" in the dependency is not allowed.
We must opt-in and allow it consciously.

## Try it

`$ cargo run`  

```bash
private_key: 
no feature, no access.

song lyrics: 
Do you have the time to listen to me whine?
```

We allowed the dependency library to read the song, adding the feature in Cargo.toml like this:

```toml
[dependencies]
library = { path = "../library", features = [
    "unsafe_feature_read_song"
    ] }
```

We did not mention anywhere the existence of the feature "unsafe_feature_read_private_key", therefore it is forbidden-by-default.  

## Development

In this example there are 2 folders:  

- songs  
- private_keys  

We have a third party library that know how to read the lyrics of the songs.  
The same library knows also how to buy the song with one click.  
Because of the compiler requirement both unsafe std:fs are wrapped in a "unsafe feature".

We build a client.  
We want it to read the lyrics.  
We didn't even know it can buy songs and read our private key.  
A silly human mistake, because there are much too many dependencies and transient dependencies to check them all.  
But because "unsafe features" are opt-in, the code with the dangerous function cannot access the file system.  
