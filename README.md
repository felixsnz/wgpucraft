# Wgpu Minecraft Clone

## Why another minecraft clone?

There will never be enough minecraft clones out there! if i really consider myself a game developer even as a hobbyist (for now), a minecraft clone is a must step i have to take!

## What's different about this minecraft clone?

Im not using any game engine, instead im using a pure graphics library (wgpu) and the rust programming language.

I have seen a couple similar minecraft clones but are pretty basic, in general my objective with this repo is to create a playable minecraft clone similar to this one:  
https://github.com/jdah/minecraft-weekend   
 but as i mentioned, using wgpu and rust.

## why not using a game engine?

Because i'm too cool for that, i like to struggle, and that keeps my mind busy, so i decided that a making game from scratch using a pure graphics library would be a great pain to experience.

## Screenshots
![Gameplay](./screenshots/1.png)



## Roadmap

### What has been done ?

* atlas texture
* block rendering
* face culling (only visible block faces are rendered)
* fps controller 
* basic chunk generation

### Work in progress...

* optimize chunk system (pending for occlusion branch)
* chunk culling (pending for occlusion branch)
* terrain generation based on noise map 

### Future features

* greddy mesh algorithm
* block manipulation
* ECS (Entity Component System)
* HUD elements

### Current status of Occlusion branch

in the terrain module in determinate_visibility method, the condition when the neighbor block is out of current chunk
handling that condition is going to be left behind at the moment to focus in other crucial things, the matter here is that
determinating visibility for each block of a single chunk its done, but when introduccing multiple chunks for the terraing procedural
generation, the border sides of the chunks have neighbor on faces that should not render.

the occlusion branch will handle that optimization latter, for now the main branch will focus on other important implementations.

the last status of occlusion branch was that since im using paralel iterator for each chunk mesh calculation, i suppose there was some race conditions
when trying to access blocks from other chunks, so one solution could be that from the beggining each chunkshould have access to its neighbor chunks.