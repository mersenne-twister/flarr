# Vec's suck.
that's why flarr exists. to relieve the crying souls of society of useful, fast,
well-formed objects, and to provide a vastly superior dynamic array.

Have you ever thought "I hate that indexing always gives me the same result"?
have you ever thoght [sik] [sic] that you wish you could index a ccommpletely
non-deterministic indice? worried that it was too easy to iterate through your array?

well despair not, for **FLARR** is here!

how to use Flarr? create a vec with your desired elements and pass it to Flarr:`:new()`!
then enjoy Flarr in all it's dynamically-allocated-and-indexed glory!!!!

```
let flarr = Flarr::new(vec![1, 2, 3, 5, 6, 7, 8, 9, 9, 10]);
println!("{}{}{}", flarr[0.3], flarr[0.52352114], flarr[0.87]);
```