# Achievement Hunter

Achievement Hunter (placeholder name) is a tool for identifying steam achievements
that you haven't achieved yet.


## Setup

A config file containing your Steam API key and user ID are required, for example:
```toml
[steam]
api_key = "steam_api_key"  # obtained from https://steamcommunity.com/dev/apikey
user_id = "user_id"  # details on find this https://help.steampowered.com/en/faqs/view/2816-BE67-5B69-0FEC
```

For *nix systems this file should be placed at `$HOME/.config/achievement_hunter/config.toml`,
while for Windows it will need to be placed at `C:\ProgramData\achievement_hunter\config.toml`.


## Why rust?

I've been looking for a rust project idea that would solve a real problem I had


## Why does this exist?

When trying to get the last achievement in a game, I spent several hours repeating
the same action to achieve a secret achievement.

What I didn't know was that I was only missing a _different_ and completely
unrelated achievement

![Man facepalming](https://media0.giphy.com/media/v1.Y2lkPTc5MGI3NjExaHUzZGY0bGE4cmM2ancyOXJydW5jbDB0YzR3ZHA1ejR3aTA2bm9hNCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/9PpY6IZmBmFz96Ru7C/giphy.gif)

And so, I wanted to build a tool to give me the list of achievements for a game,
including whether I had achieved them or not so I didn't waste more of my time
trying to get achievements I already had.

