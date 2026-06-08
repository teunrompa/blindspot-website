enum Pages {
  Landing,
  Home,
  About,
  Events,
}

enum EventStatus {
  SoldOut,
  Available,
  ComingSoon,
}

interface Event {
  id: number;
  name: string;
  date: string;
  description: string;
  venue: string;
  lineup: Artist[];
  status: EventStatus;
}

interface Artist {
  name: string;
  instagram_url: string;
  spotify_url: string;
  photo_url: string;
}

export { Pages, EventStatus };
export type { Event, Artist };
