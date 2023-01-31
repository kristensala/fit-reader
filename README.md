# Fit Reader TUI
Just a simple dashboard to filter through my workouts and show an overall summary and summary of road/mtb/indoor individually. Workouts are only implemented by importing .fit files which are generated after each workout by smart devices (Garmin, Wahoo, Mobile apps...). Manual workout editing or adding is not available.

**PS! This is my first Rust project. Still learning and trying to make sense of many things.**

![image](/images/tui.jpg)

## Setup
### Linux
- todo!()

### Windows and Mac
- Don't know, don't care

## Navigating in TUI
- Arrow keys and j/k for filtering through sessions
- q to exit out from the TUI

## Importing fit files
- Set file path where the fit files will appear (currently hardcoded)
- Run: `cargo run import`

## TODO
- [ ] Add the ability to read environment variables or the ricing ability
- [ ] Summaries are not calculated yet
- [ ] Last 7 weeks section is missing a bar chart which will be based on hours (for some reasong I'm also getting week 52 with only this years data???)
- [ ] TUI is not responsive what so ever (texts don't wrap)
- [ ] What happens if it fails to read some rows from database?
- [ ] I don't know what will happen if the amount of sessions won't fit into the session list block (Is there a way to make it scrollable? -> probably) 
- And what ever else I come up on the go...
