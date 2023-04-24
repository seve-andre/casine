<div align="center">
  <img src="./public/casine-home.svg" alt="Le Casine logo" height="300" />
</div>

# Le Casine
Apartment management system. A total of 12 apartments inside 2 houses (A and B) rented by groups of people during summer

<!-- Remove heading and replace it with screenshots of the app -->
# Screenshots

## Required features
- [ ] add/remove guests
- [ ] search for guests
- [ ] edit guests data
- [ ] see bookings
- [ ] see apartments prices
- [ ] make it as easy as possible for non-technical users (adding UI & UX components to better understand what's going on, such as snackbars, tooltips, alerts, ...)

## Optional features
- [ ] guests history
- [ ] calculate apartment final price (apartment month-based price + [tourist tax](https://www.ovest.com/it/la-tassa-di-soggiorno-in-italia/))
- [ ] export guests to multiple formats (PDF, CSV, ...)
- [ ] i18n
- [ ] dark theme
- [ ] improve UI & UX even further (skeleton-loaders, loading animations, lazy components, ...)
- [ ] add splashscreen

---

### What I used
This project uses the following:
- [Tauri]: to build optimized, secure, and frontend-independent application for multi-platform deployment using [Rust] as backend
- [Svelte]: as frontend JS framework
- [SvelteKit]: to rapidly develop a Svelte app (includes routing, accessibility, ...)
- [Flowbite-Svelte]: as Svelte-based UI framework with built-in UI components (buttons, chips, ...)
- [Vite]: as frontend tool for faster development
- [TypeScript]: as main language for frontend
- [Diesel]: ORM and Query Builder in [Rust] to store all infos about houses, apartments, guests, ...

### Why Tauri?
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
