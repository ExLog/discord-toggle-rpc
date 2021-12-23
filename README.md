# discord-toggle-rpc

A simple command-line app to toggle Discord Rich Presence written in Rust.

Discord Rich Presence is status that shows up when you're playing game or listening to Spotify.

## Install

```sh
cargo install discord-toggle-rpc
```

## Environment Variables

- DISCORD_TOKEN

## How to Find Your Discord Token

Your Discord token usually will look like
`mfa.<random characters>` or `<random characters>`

To find your Discord token open your Discord with Discord App or browser

- On browser, open your browser DevTool (Inspect Element) and go to Network tab.
  Click on any XHR request and find your `authorization` request header.

- On Discord App, hold `Ctrl+Shift+i` to open the DevTool and do the rest following the browser method.
  If this doesn't work, please use the browser method instead.

## Set the environment variables

Copy your Discord Token and add new line in `/etc/environment`

```sh
DISCORD_TOKEN=<your token>
```
