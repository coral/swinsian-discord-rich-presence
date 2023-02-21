# Display Swinsian music status in Discord

Always bothered me that Spotify gets the royal treatment, time for the quick and dirty. Swinsian sadly doesn't have a good IPC so we're dragging the data out of Swinsian using AppleScript (yes, shoot me). This is some shitty rust that:

- Compiles `swinsian-apple-script.scpt` to a compiled applescript due to perf
- Calls this abomination of hand formatted JSON every 5 secs
- Tells Discord "yo bro, this is the good stuff"
- Discord goes "ok man, sounds good to me"

![/assets/example.png]

## Usage

`cargo run`
yep that's it

## Contributing

Just open a PR LUL

## License

All under MIT