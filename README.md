## What is this?
The Murgi Bot is a Discord Bot Service that listens to messages in a guild/server, and when called using it's specified prefixes, or receives an alert that requires it to respond, it does exactly that. \
It was, and is made to be a simple bot that was created to entertain my friend group, and a fun little experiment on working with the Rust Language, and Discord API.

There's a bunch of cool features that comes with the Murgi Bot, and here's a list of these features,
- [X] Howls upon request, and when all four Murgis howl.
- [X] Queries piratebay for magnet links to games.
- [X] Can roll DnD dices, and random numbers in a specified range.
- [ ] Play songs from YouTube.
- [ ] Play anime from GogoAnime.
- [ ] Stream videos from YouTube.

## Installation
In order to set up the Murgi Bot, you will be required to install Rust, and set up a Rust Development environment, which is present in detail in the [Rust Book](https://doc.rust-lang.org/stable/book/ch01-01-installation.html).

The bot also utilizes the Serenity Library in order to interact with the Discord API running in the backend, however, all of the development set up will be done by a simple `cargo build` or a `cargo run`, so, there's not much to worry about there.

There is also an ongoing development of a Dockerfile, and a `docker-compose.yml` for the Murgi Bot, to be developed as a containerized application.

Finally, there is a requirement for a few environment variables in order to work with Murgi Bot, these are defined in the `.env.example` file.

There are currently two containerized approaches that can be utilized to set up the Murgi Bot container.

### Parameters
The application runs through various connections formed through a bunch of tokens and the such, which are listed below,

| Parameter          | Function                              |
|------------------  |---------------------------------------|
| `DISCORD_TOKEN`    | The discord token to control the bot. |
| `JACKETT_RSS_FEED` | RSS Feed link to query TPB.           |

### docker-compose
```dockerfile
version: '3'

services:
  murgi_bot:
    container_name: murgi
    image: datadi/murgi_bot:latest
    environment:
      DISCORD_TOKEN: {DISCORD_TOKEN}
      JACKETT_RSS_FEED: http://{ip_addr:port}/api/v2.0/indexers/thepiratebay/results/torznab/api?apikey={API_KEY}&t=search&cat=&q=
      RUST_LOG: DEBUG
    restart: unless-stopped
```

### Docker CLI
```sh
docker run -d \
    --name=murgi \
    -e DISCORD_TOKEN={DISCORD_TOKEN} \
    -e JACKETT_RSS_FEED=http://{ip_addr:port}/api/v2.0/indexers/thepiratebay/results/torznab/api?apikey={API_KEY}&t=search&cat=&q= \
    --restart: unless-stopped \
    datadi/murgi_bot:latest
```

### Building from source
If there is a need to build the application from source, either to implement or extend upon certain features, building the bot locally is also a feasible solution. \
The step by step process in order to do so, can be done through the following command in the terminal.
```sh
git clone https://github.com/dat-adi/MurgiBot.git
cd MurgiBot
cargo build --release
./target/release/murgi_bot
```

## Contributing
The Murgi Clan loves new features! \
If you're interested in working with a bit of rust, solving a few issues, or just really like chickens, you're free to contribute to the development of the project!

Steps to contribute,
1. Raise an issue detailing what the issue/feature request is.
2. Fork the project into your account.
3. Create a Pull Request linking whichever issue it resolves and send in the PR for evaluation.
4. If everything looks good, the Murgi Clan approves, and you've contributed to Murgi Bot!

For a better view of the details, check out our [CONTRIBUTING.md](https://github.com/dat-adi/MurgiBot/blob/main/CONTRIBUTING.md)!

## License
The Murgi Bot is a project that was made as a means to get the friend group a discord mascot. \
This code can be forked and worked on, to develop your own mascot for your discord server as well! \

[Hosted under the GPLv3 License](https://github.com/dat-adi/MurgiBot/blob/main/LICENSE).

---
<p align="right"><i>dat-adi</i></p>

