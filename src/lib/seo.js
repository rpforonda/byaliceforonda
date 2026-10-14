// Site-wide SEO constants and structured data shared by the pages.

export const SITE_URL = "https://catchupwithalice.com";
export const SITE_NAME = "Alice Foronda";

// Social share image (1200×630) — lives in static/ so its URL never changes.
export const OG_IMAGE = {
  url: `${SITE_URL}/og-image.jpg`,
  width: 1200,
  height: 630,
  alt: "Crosby the crocodile, Carla the capybara and Tank the turtle gazing up at a starry night sky",
};

export const SHOP_URL = "https://catchupwithalice.myshopify.com/";
export const AMAZON_URL = "https://www.amazon.com/dp/B0HFTDNCKW";

export const author = {
  "@type": "Person",
  "@id": `${SITE_URL}/#alice`,
  name: "Alice J. Foronda",
  alternateName: "Alice Foronda",
  jobTitle: "Children's book author",
  url: `${SITE_URL}/about`,
  sameAs: ["https://instagram.com/byaliceforonda", "https://www.tiktok.com/@catchupwithalice"],
};

export const book = {
  "@type": "Book",
  "@id": `${SITE_URL}/book#book`,
  name: "Crosby's Cosmic Adventure",
  isbn: "9798995168706",
  author: { "@id": author["@id"] },
  illustrator: { "@type": "Person", name: "Ponyo Nguyen" },
  inLanguage: "en",
  genre: "Children's picture book",
  audience: { "@type": "PeopleAudience", suggestedMinAge: 4, suggestedMaxAge: 8 },
  description:
    "Crosby the crocodile worries his tail's too thick and his claws too long. With a little courage and help from his friends Carla the capybara and Tank the turtle, he discovers just how brightly he can shine. A heartwarming story about bravery, belonging, and believing in yourself, for ages 4–8.",
  image: OG_IMAGE.url,
  url: `${SITE_URL}/book`,
  // the Amazon listing is the paperback edition of the book
  workExample: {
    "@type": "Book",
    bookFormat: "https://schema.org/Paperback",
    url: AMAZON_URL,
  },
};
