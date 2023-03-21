<div align="center">
  <img src="https://github.com/seve-andre/casine/blob/main/public/casine.svg" alt="Le Casine di Cervia logo" height="150" />
</div>

# Le Casine di Cervia
Apartment management system, built for the owner. A total of 12 apartments inside 2 houses (A and B) rented by groups of people during summer

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

# Screenshots

# Tauri + Svelte + Vite template
This project uses the following:
- [Tauri](https://tauri.app/): to build optimized, secure, and frontend-independent application for multi-platform deployment using [Rust](https://www.rust-lang.org) as backend
- [Svelte](https://svelte.dev/): as frontend JS framework
- [SvelteKit](https://kit.svelte.dev/): to rapidly develop a Svelte app (includes routing, accessibility, ...)
- [Flowbite-Svelte](https://flowbite-svelte.com/): as Svelte-based UI framework with built-in UI components (buttons, chips, ...)
- [Vite](https://vitejs.dev/): as frontend tool for faster development
- [TypeScript](https://www.typescriptlang.org/): as main language for frontend
- [Diesel - SQLite](https://diesel.rs/): ORM and Query Builder in Rust to store all infos about houses, apartments, guests, ...

# Why Tauri?
[Tauri](https://tauri.app/) is a modern framework for building tiny, blazingly fast binaries for all major desktop platforms. It uses [Rust](https://www.rust-lang.org), a high-perfomance and secure programming language, as backend and any JS framework you want for the frontend ([Svelte](https://svelte.dev/) in my case). I've chosen Tauri over [Electron](https://www.electronjs.org/) after reading [this article](https://betterprogramming.pub/will-tauri-be-an-electron-killer-38fd6478004) that explains the differences between the two.

