import {
  HomeOutlined,
  HomeFilled,
  BookOutlined,
  BookFilled,
  PriceTagOutlined,
  PriceTagFilled
} from "$lib/ui-components"

export const sidebarItems = [
  {
    name: "Principale",
    href: "/",
    outlinedIcon: HomeOutlined,
    filledIcon: HomeFilled
  },
  {
    name: "Prenotazioni",
    href: "/bookings",
    outlinedIcon: BookOutlined,
    filledIcon: BookFilled
  },
  {
    name: "Prezzi",
    href: "/prices",
    outlinedIcon: PriceTagOutlined,
    filledIcon: PriceTagFilled
  }
]
