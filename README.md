# Fit Reader TUI
Just a simple dashboard to filter through my workouts and show an overall summary and summary of road/mtb/indoor individually. Workouts are only implemented by importing .fit files which are generated after each workout by smart devices (Garmin, Wahoo, Mobile apps...). Manual workout editing or adding is not available.

![image](/images/tui.jpg)

## Set up
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
- Summaries are not calculated yet
- Last 7 weeks section is missing a bar chart which will be based on hours
- TUI is not responsive what so ever (texts don't wrap)
- Something seems to be wrong with the session graphs. Some data seems to be missing time to time. (need to look into it)
- And what ever else I come up on the go...
