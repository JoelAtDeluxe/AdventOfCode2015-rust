# MD5 Hasher

So, it seems like rust doesn't have a standard md5 implementation, but there are a few options.
Here are some timed versions, all in milliseconds:

Machine:

* Intel i7-8550U (4 core/8 thread @ 1.8Ghz)

Solutions:

* 5 zero solution: 282749 hashes
* 6 zero solution: 9962624 hashes

Supporting libraries:

* Converting u8 to hexidecimal string: [hex](https://docs.rs/hex/0.3.1/hex/fn.encode.html)

| Crate                                         | 5-zero debug | 5-zero release | 6-zero debug | 6-zero release |
| --------------------------------------------- | ------------ | -------------- | ------------ | -------------- |
| [md5](https://docs.rs/md5/0.6.1/md5/)         | 1838         | 361            | 65511        | 9544           |
| [md-5](https://docs.rs/md-5/0.8.0/md5/) + hex | 2474         | 91             | 92107        | 3206           |
