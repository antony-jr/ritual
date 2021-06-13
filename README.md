# Ritual

**Note**: This program is not relevant now since the output from this program can't be used since the main
program is still under rewrite and will probably take a while to have a stable release. In the next major release
of spirit (**v0.2.0**) this program is needed to make spirits.

Make Spirits for the Twenty First Century Window Sitter.

This program takes in a **spirit directory**(which is a normal directory) with a specific structure
which has all the data and instruction to render a spirit.

# Usage 

```
 ritual make ./Inori # Here Inori is a Spirit Directory :)
```

**Now you should have Inori.spirit which Spirit can now consume.**

# Spirit Directory Specification (v0.1.0)

- Spirit File Name/ (ex: Inori which will become Inori.spirit after ritual)
|--|
   |-actions/
   |-audio/
   |-meta.json

#### meta.json

```
{
	"name": "Spirit Name",
	"version": "v0.1.0", // Semver is recommended
	"author": "Author of this Spirit Directory",
	"copyright": "Copyright of all the Artwork",
	"positions" : {
		"yoff-px": 0 // Y offset if needed
	},
	actions: {
		"default": { // *This is required
			"frames": ["0-*"],
			// The Frame Sequence to render this action named "default"
			// 0-* means from 0 file index to the last.
			// Here 0 does not mean the file name but it is the index no.
			// i.e Under the actions/ directory
			// You are required to make a directory known as default
			// and copy all your artwork in there.
			// if You have named your each frame of animation as 
			// say pixmap00.png to pixmap100.png or 00.png to 100.png
			// , You only need to specify the index of the file when it 
			// is sorted
			// Here 0-* means from 0th index of the sorted list files 
			// found under actions/default
			// Take the first set of animation as all files sorted 
			// alphabetically.

			loop: false, // This is true for default action by default.
			play: "hero", // Play audio/hero.mp3 when rendering this action. (Optional)
		},
		/// ... You can add many action as you want
	}
}
```

#### actions/

This directory holds all the artwork for your actions. If you have a action known as ```happy``` you should 
make a directory called ```happy``` and copy all your frames of animation.

#### audio/ (Optional)

This directory holds all your audio files. Optionally to play a audio when a action is rendered. The files should be of 
**.mp3** format.


# LICENSE

The MIT License.

Copyright (C) Antony J.R.

