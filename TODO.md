- [ ] Stop game at round 7
- [ ] Show running score for Bot
- [ ] Reset game button
- [ ] Restoring a game should show the correct round number
- [ ] Saved game should save all previous rolls
- [ ] DaisyUI-ify so that it looks less jank
- [ ] About page that links to the company's official store page:
    https://horribleguild.com/product-tag/railroad-ink/
- [ ] Deploy app. One of these options:
    - Use vercel, adapt code to use their sql package.
    - Dockerize


More broadly, I want to evaluate if I should migrate tech stack.
The new stack I have in mind would be:
    - Rust axum
        - axum
        - askama. I want to try maud at some point, but I like askama for integrating with html snippets.
    - htmx
    - tailwind + daisyUI

I want to make a bot that makes sensible decisions instead of just random ones.
I want to do that in a robust language instead of the janky typescript I wrote 2-3 years ago.


## Bot development
This youtube video covers Neuro Evolution of Augmenting Topologies (NEAT). I think it may be a good fit for the problem.
https://www.youtube.com/watch?v=dkvFcYBznPI&ab_channel=b2studios

They have an implementation of NEAT in C#:
https://github.com/b2developer/MonopolyNEAT

There's also an implementation of NEAT in python here:
https://neat-python.readthedocs.io/en/latest/installation.html


Current plan of action is:
- Build bot that chooses legal actions but at random. 
- Build a bot that maximizes score for the round.
- Build out NEAT algorithm

Use those as baselines to compare against the NEAT based bot. Scoring is the fitness function, so it's a necessary step to using Neat.

I need to decide between:
- Python 
- C#
- Rust

I think I will read the initial paper, before proceeding.
Ok I read the paper. I think the C# implementation is clearer. So I will use that as a reference. The amount of code is not huge and it's based on the paper, so I will reimplement the NEAT algorithm in Rust.

I'm going to simplify the game further for the sake of figuring out unknowns:

- There are no special routes, only the rolled dice each round.
- Limit to 1 die roll per round. (Scale the number of rounds fourfold to compensate, until we bring up the number of dice)
- Limit die faces to 6 basic faces, instead of the special die w/ stations.

With those simplifications, what is the size of the input vector?
What is the size of the output decision vector?


There are 9 possible die faces. 
There are 49 locations on the board. Each die can be placed one of 8 different ways. (rotations & reflections; technically this space can be pruned down. Only the right angle station needs to account for reflections. )


There are 34 die patterns:
4 angle rail
4 3 rail
2 straight rail
4 angle road
4 3 road
2 straight road
2 overpass
4 straight station
8 angle station


Have an output node for each grid tile for each die pattern. Then given the die rolled, determine the available patterns. Determine where the legal placements are, and select the highest rated one.

This works well for a single die roll.


## Concerns
- I'm unsure how to give this output format the flexibility to encode decisions for multiple die placements.
The order of placement effects legality.
- The scope of this project feels huge. Implementing a NEAT algorithm by hand feels like I'll need to build some serious debugging tools.
- The size of my input and output vectors feels huge compared to the monopoly example. It took them 2 weeks to train the model. I worry about the time involved to train this model.
