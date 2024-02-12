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
    - Rust with axum & maud
    - htmx
    - tailwind + daisyUI

I want to make a bot that makes sensible decisions instead of just random ones.
I want to do that in a robust language instead of the janky typescript I wrote 2-3 years ago.
