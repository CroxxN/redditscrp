# A reddit scrapper for insults

### This project was created when I was just learning Rust, so the code is stroke-inducing.

## What I wanted it to do:

- Scrape r/insults for the top posts of all time.
- Store the posts in mongodb.
- Have a discord bot query it for insults.
- The server would return the insult.
- The discord bot would insult someone with the server-replied insult

## What it does:

- Does the intended job but as in `just works`

Didn't know about the webdata parameter in actix-web at that time, so the server would make unneccessary calls to initiate mongo client everytime lmao. Pretty funny because I looked for solutions and apparently, the solution was right in the first basic actix-web example.

## ToDo:

Although I don't think I will do anything with this project, I do see a potential learning opportunity. I could use this project as a execuse for developing a reddit api client lmao. Let's us, future me.

### All in all, a good experience