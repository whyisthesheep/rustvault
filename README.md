# Rustvault

> [!CAUTION] 
> The database is not encrypted

A cli password manager made with rust using sqlite and clap
This is not designed to be used

# Installation 

1. Clone the repo `git clone https://github.com/whyisthesheep/rustvault/`
2. Build the project `cargo build --release`
3. Run the binary `./target/release/rustvault`

# Commands

```
init - Initialise the db
add (service) (username) - Add a login to the db
get (service) - Recall a login from the db
generate (length) - Generate a new password
```

# Todo
- [ ] Encrypt db
- [ ] Update randomisation
