## Stability
This crate is not stable at all. Use at your own risk.

# Clusters 
Clusters are the main component of this crate. They act similarly to
String, with the main difference being the index/iteration processes
going over grapheme clusters, instead of char.

## Why this crate
This crate stems from the need to have easy to use structures, similar
to String, but with the grapheme clusters as its element. For example,
it means that this is now trivial:
```
		import unicode_extension::Clusters;
        let s = "Étiré";
		let c = Clusters::new(s, true)
        let expected_clusters = Clusters(vec!["É", "t", "i", "r", "é"]);
        assert_eq!(c, expected_clusters);
```

## Goals
Have a similar api to `std::string::String`.

