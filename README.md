# Galactica

Your swiss army knife

## Examples

[![asciicast](https://asciinema.org/a/KumzgCUpylL4ipEPIdMN4jaU6.svg)](https://asciinema.org/a/KumzgCUpylL4ipEPIdMN4jaU6)

### Code review a single file
```sh
$ cat src/main.rs | galactica code 'review the quality of this code - output 
only "HIGH" or "LOW" and a one sentence reason why'
HIGH - The code is well structured, modular, and uses good programming practices 
like async-await, error handling, and command-line argument parsing.
```

### Describe the components of a project
```sh
$ for f in src/*; do echo Reviewing $f; cat $f | galactica code 'review this 
code and output a single sentence describing the functionality'; done
Reviewing src/config.rs
This code provides functionality to read, write, and manipulate a configuration 
file related to a Discord bot through the use of the `Config` struct and 
associated methods.
Reviewing src/discord_login.rs
The code launches a web server, awaits and captures a token via an OAuth2 
authorization code flow from a Discord API callback, and stores the token in a 
configurable Rust struct configuration.
Reviewing src/errors.rs
The code defines an enum type "Error" with different variants, and an 
implementation of Display trait to display the error messages based on the 
variant.
Reviewing src/galactica_api.rs
This code provides functions for making asynchronous API calls, getting Discord 
access tokens, and sending instructions to a server.
Reviewing src/lib.rs
The code declares four modules for configuration, Discord login, error handling, 
and integration with the Galactica API.
Reviewing src/main.rs
This code is a command line interface (CLI) for interacting with an AI-powered 
chatbot and generating code based on requirements, with features including 
login, conversation history, and resetting history.
```

## Code generation

```sh
$ galactica code 'a regex to validate an email address'
/^[^\s@]+@[^\s@]+\.[^\s@]+$/
```

Embedding galactica in your shell scripts
```sh
$ cat database.txt | grep -e $(galactica code 'a regex to validate an email 
address')
```

```sh
$ galactica code 'a package.json for a project called FooDog which uses the 
latest version of react'
{
  "name": "FooDog",
  "version": "1.0.0",
  "description": "",
  "main": "index.js",
  "dependencies": {
    "react": "^17.0.2"
  },
  "devDependencies": {},
  "scripts": {
    "start": "react-scripts start",
    "build": "react-scripts build",
    "test": "react-scripts test",
    "eject": "react-scripts eject"
  }
}
```

## Open ended chat conversation

Galactica also supports interactive chat, where history and context is 
maintained throughout, similar to ChatGPT.

```sh
$ galactica chat 'please tell me an interesting 1 sentence story of a pirate'
As a notorious pirate, Blackbeard would braid his long beard with slow burning 
fuses, making himself appear like the devil himself to strike fear into his 
enemies.

$ galactica chat 'next sentence'
Legend has it that he buried treasure worth millions, which has never been found 
to this day.

$ galactica chat 'next sentence'
Despite his fearsome reputation, Blackbeard was killed in battle at age 38, and 
his head was hung from the end of a ship's bow as a warning to other pirates.

$ galactica chat 'twist'
However, some believe he faked his own death and sailed off into the sunset with 
his treasure, living out the rest of his days in riches on a secluded island.

$ galactica chat 'end'
But whether it's true or not, Blackbeard's legend and legacy have lived on for 
centuries, becoming an iconic figure of piracy and adventure on the high seas.
```

You can view & reset your history:

```sh
$ galactica history
<...your entire chat history ...>

$ galactica reset
<... your chat history is cleared...>
```