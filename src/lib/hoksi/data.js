// Hoksi Haus, the hand-painted banner studio at /hoksi-haus. Sizes and prices match the
// studio's price list and its Google Form; change them here and the page follows.

// The form's public (responder) link. It's linked, not embedded, so a visitor's browser only
// contacts Google when they choose to open it.
export const FORM_URL =
  "https://docs.google.com/forms/d/e/1FAIpQLSccl3h8C6sSvV6LvIAZDAvzjqIbopp_ScWIm44vdfy4euwNqA/viewform";

export const TURNAROUND_DAYS = 10;

// Kept in two parts and joined only in the browser (MailLink.svelte), so the whole address
// never appears in this public repo or in the prerendered HTML for bots to harvest.
export const EMAIL_USER = "hoksihaus";
export const EMAIL_DOMAIN = "gmail.com";

// in the order (and with the names) the form lists them
export const sizes = [
  { size: "XS", dimensions: "85 × 65 cm", price: 15 },
  { size: "S", dimensions: "85 × 75 cm", price: 20 },
  { size: "M", dimensions: "85 × 100 cm", price: 30 },
  { size: "L", dimensions: "85 × 150 cm", price: 50 },
  { size: "XL", dimensions: "85 × 175 cm", price: 60 },
];

export const addOns = [
  { label: "rush order (1 week or less, if approved)", price: "+€15" },
  { label: "detailed characters or borders", price: "+€10–20" },
];
