<div align="center">
  <img src="https://github.com/seve-andre/casine/blob/main/public/casine.svg" alt="Le Casine di Cervia logo" height="150" />
  
  # casine
  [![License](https://img.shields.io/github/license/seve-andre/casine)](https://github.com/seve-andre/casine/blob/master/LICENSE)
  [![Stars](https://img.shields.io/github/stars/seve-andre/casine?style=flat&logo=github&label=stars)](https://github.com/seve-andre/casine/stargazers)
  
  Apartment management system. A total of 12 apartments inside 2 houses (A and B) rented by groups of people during summer
</div>

<!-- Remove heading and replace it with screenshots of the app -->
# Screenshots

# Required features
- [ ] Add guests to data table for fast visualization
- [ ] See apartments prices
- [ ] Make it as easy as possible for non-technical users (adding UI & UX components to better understand what's going on, such as snackbars, tooltips, alerts, ...)
- [ ] See all guests
- [ ] Search for guests inside apartment
- [ ] Make guests info editable in data table
- [ ] See bookings

# Optional features
- [ ] Calculate apartment final price (apartment month-based price + tourist tax)
- [ ] Calculate tourist tax based on guests. see [how to](https://www.ovest.com/it/la-tassa-di-soggiorno-in-italia/)
- [ ] Export data table using multiple formats (PDF, CSV, ...)
- [ ] Detect if usb stick is plugged in and export guests info to it if permission is granted
- [ ] Add i18n
- [ ] Add dark theme support
- [ ] Build better UI & UX (skeleton-loaders, loading animations, lazy components, ...)
- [ ] Add splashscreen when entering the app (skeleton-loader sorta thing)



# Tauri + Svelte + Vite template
This project uses the following:
- [Tauri]: to build optimized, secure, and frontend-independent application for multi-platform deployment using [Rust] as backend
- [Svelte]: as frontend JS framework
- [SvelteKit]: to rapidly develop a Svelte app (includes routing, accessibility, ...)
- [Flowbite-Svelte]: as Svelte-based UI framework with built-in UI components (buttons, chips, ...)
- [Vite]: as frontend tool for faster development
- [TypeScript]: as main language for frontend
- [Diesel]: ORM and Query Builder in [Rust] to store all infos about houses, apartments, guests, ...

# Why Tauri?
[Tauri] is a modern framework for building tiny, blazingly fast binaries for all major desktop platforms. It uses [Rust], a high-perfomance and secure programming language, as backend and any JS framework you want for the frontend ([Svelte] in my case). I've chosen Tauri over [Electron] after reading [this article](https://betterprogramming.pub/will-tauri-be-an-electron-killer-38fd6478004) that explains the differences between the two.

<!-- Links used -->
[Tauri]: https://tauri.app/
[Rust]: https://www.rust-lang.org
[Svelte]: https://svelte.dev/
[SvelteKit]: https://kit.svelte.dev/
[Flowbite-Svelte]: https://flowbite-svelte.com/
[Vite]: https://vitejs.dev/
[TypeScript]: https://www.typescriptlang.org/
[Diesel]: https://diesel.rs/
[Electron]: https://www.electronjs.org/
