# Godot PureData (GPD):

This is a WIP for using Puredata patches as sound sources in Godot 4. 

There have been [previous (and far more complete)](https://github.com/magogware/godot-audiostreampd) integrations in 
Godot 3, but with the move to [GD Extensions](https://godotengine.org/article/introducing-gd-extensions/) a lot of 
libraries (such as the puredata one) are no longer usable. 

This library is fairly idiosyncratic for two reasons:
 1. I've only worked on it for 2 days
 2. The [Godot documentation of custom audio streams is a mess](https://www.reddit.com/r/godot/comments/o2kazd/has_anyone_actually_gotten_a_custom_audiostream/) and
 3. I am currently doing something very dumb to get this to "work".

What this library currently supports:
 - Reading and playing a puredata patch.
 - `AudioPlayerStream`, `AudioPlayerStream2D`, `AudioPlayerStream3D`.
 - Multipatch loading.

What it doesn't support:
 - multiple instances.

This lack is multiple instances is obviously a huge limitation, but it's something [I am inheriting from an underlying library](https://github.com/alisomay/libpd-rs/issues/20). 
Now `libpd` has had support for this for about 5 years, so maybe I should just buckle down and not use a wrapper library... more to come on that maybe.

## How To Use:
This plug-in works by dumping an audio buffer into a `GenerativePlayback` resource in Godot. The current setup is to instance a `GPD` node and then child the audio player you want to use under it. 
Select `Generator` for the stream in the audio player. Specify the node path to the audio player, the path (relative to the project root) for `GPD` to find your patches, and the name of the entry 
("main") patch. After all of that is done, just have some GD script call `play()` on the `GPD` node and listened to those sweet generative patches play out.


## Roadmap/Contributions:
I am sort of toying around with this in as part of a larger game/tool-set. Therefore its direction will be first and foremost guided by what _I_ need it to do. The code isn't clean, there 
are many ergonomic issues, etc. But it currently works for my use-case. 

If you have improvements, please feel free to chime in, but if you need it to do something other than what it currently does, forking it might be the most efficient option. 
Hopefully someone who actually knows what they are doing will be inspired to make a better version of this tool. 




















