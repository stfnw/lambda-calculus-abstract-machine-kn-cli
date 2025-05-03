A small demo project performing normal-order evaluation of untyped lambda calculus terms.
It wraps the library https://github.com/stfnw/lambda-calculus-abstract-machine-kn into a small cli app.
The library implements a full-reducing variant (KN) of the Krivine abstract machine;
specifically the formulation in Á. GARCÍA-PÉREZ and P. NOGUEIRA, "The full-reducing Krivine abstract machine KN simulates pure normal-order reduction in lockstep: A proof via corresponding calculus", Journal of Functional Programming, vol. 29, p. e7, 2019. doi:10.1017/S0956796819000017; Fig. 3. "A version of KN that works with closures with closed terms".
(This repo is unaffiliated; goal is me learning).

Example run:

```
$ cargo run -- '((λa. (a a)) (λa. a))'
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/lambda-calculus-full-reducing-krivine-machine-cli '((λa. (a a)) (λa. a))'`
parsed term:
    named: ((λa. (a a)) (λa. a))
    blc:   01000110100010
reduced term:
    named: (λa. a)
    blc:   0010
```
