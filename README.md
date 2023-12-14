# Chess
This project is still a WIP.
It is a playground for me to better develop my Rust and large repository management skills.

## Current status
The chess engine is implemented and is exposed via the server to communicate with the web app.

NOTE: Project is currently on hold because I'm waiting on a next release of `prisma-client-rust` and `rspc` to create game rooms, auth, etc, etc.

## Long term todos
### Chess engine library
 - Written in Rust
 - Location: `/crates/engine`
 - [x] Use bitmaps to represent board states
 - [x] Magic move generation
 - [x] Playable
 - [ ] REPL

### Web App
 - Written in TypeScript, uses ReactJS (next.js)
 - Location: `/apps/web`
 - [x] Basic board UI
 - [ ] Rooms
   - [ ] PvP
   - [ ] PvE
 - [ ] Chat & history

### Server
 - Written in Rust
 - Location: `/apps/server`
 - [x] Chess library to API interface
 - [ ] Users
 - [ ] Rooms
   - [ ] PvP
   - [ ] PvE
      
### React Native App
Will start working on this as soon as the web app is finished.
 - Location: `/apps/expo`

### Tauri App
This is blocked by the server & the web app.
 - Location: `/apps/desktop`

