<p align="center">
<img width="200" src="https://github.com/Demizo/Jotterly/blob/master/Jotterly%20Banner.svg" alt="Jotterly Banner">
</p>
<h1 align="center">Jot down anything without cluttering your notes app</h1>

Traditional note taking apps are meant to store organized **monolithic** notes. When you just want to **jot** down a *quick* fact or idea, finding the write place to put the information can be challenging. Does it *belong* in an **existing** note? Does it deserve a *whole* **new** note?

**Jotterly** streamlines jotting down facts and ideas without the hassle or clutter of managing **monolithic** notes...

Simply **Jot** something down and search for it later!

## Installation
### Windows
Download the latest **.msi** installer from [releases](https://github.com/Demizo/Jotterly/releases).
### Linux
**Jotterly** is available as an **AppImage** and will run anywhere an **AppImage** can. You can use [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher) to integrate **Jotterly** with your system. Download the latest **AppImage** from [releases](https://github.com/Demizo/Jotterly/releases).

## Features
- [x] Create Jots
- [x] Theming options
- [ ] Custom theme option
- [x] Tag Jots
- [x] Filter by tag
- [x] Markdown Support
- [ ] Keyboard Shortcuts
- [x] Keyboard Navigation
- [ ] System Tray / Global shortcuts
- [ ] Color Tags
- [ ] Image / Screenshot Jot support

## Theming
todo

## Development
Jotterly is built using [Tauri](https://tauri.app/) and [Svelte](https://svelte.dev/).
- Clone the repo
- Install Tauri dependencies
- Run with `npm run tauri dev` or `cargo tauri dev`
- Build with `cargo tauri build`
#### Setup database for sqlx
- Create a .env file in the project root with the following contents `DATABASE_URL=sqlite:./jotterly.db`
- Create a jotterly.db file in /src-tauri/
- Run migrations on the database with `sqlx migrate run`

## License

This software is free software licensed under the GNU General Public License 3.0.
