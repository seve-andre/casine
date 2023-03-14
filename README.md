# What's this?
Modern desktop app using Tauri (see [why Tauri](https://tauri.app/) and the [differences with Electron](https://betterprogramming.pub/will-tauri-be-an-electron-killer-38fd6478004)). Tauri uses Rust, a high-perfomance and secure programming language, as backend and any JS framework you want for the frontend (Svelte in my case).

Specifically, it's an app for apartment rental management, built for the owner. Specifically, he rents the apartments inside of 2 houses (A and B), usually for a month. He wants to add guests inside a table in each apartment, see prices and calculate final price when rents end. For later versions, I also added optional features, such as better UX, exporting data, internationalization, dark mode and tourist tax automatic computation.

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
