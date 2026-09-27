// Hebrew Calendar page. Talks to the desktop app through Tauri commands, or
// to the web server through /api/v1 — the same requests either way.
"use strict";

const CITIES = [
  { group: "Israel", name: "Jerusalem", lat: 31.7683, lng: 35.2137, tz: "Asia/Jerusalem", israel: true, candles: 40 },
  { group: "Israel", name: "Tel Aviv", lat: 32.0853, lng: 34.7818, tz: "Asia/Jerusalem", israel: true, candles: 18 },
  { group: "Israel", name: "Haifa", lat: 32.794, lng: 34.9896, tz: "Asia/Jerusalem", israel: true, candles: 30 },
  { group: "Israel", name: "Be'er Sheva", lat: 31.2518, lng: 34.7913, tz: "Asia/Jerusalem", israel: true, candles: 18 },
  { group: "North America", name: "New York", lat: 40.7128, lng: -74.006, tz: "America/New_York" },
  { group: "North America", name: "Lakewood", lat: 40.0821, lng: -74.2097, tz: "America/New_York" },
  { group: "North America", name: "Boston", lat: 42.3601, lng: -71.0589, tz: "America/New_York" },
  { group: "North America", name: "Philadelphia", lat: 39.9526, lng: -75.1652, tz: "America/New_York" },
  { group: "North America", name: "Baltimore", lat: 39.2904, lng: -76.6122, tz: "America/New_York" },
  { group: "North America", name: "Miami", lat: 25.7617, lng: -80.1918, tz: "America/New_York" },
  { group: "North America", name: "Chicago", lat: 41.8781, lng: -87.6298, tz: "America/Chicago" },
  { group: "North America", name: "Dallas", lat: 32.7767, lng: -96.797, tz: "America/Chicago" },
  { group: "North America", name: "Denver", lat: 39.7392, lng: -104.9903, tz: "America/Denver" },
  { group: "North America", name: "Phoenix", lat: 33.4484, lng: -112.074, tz: "America/Phoenix" },
  { group: "North America", name: "Los Angeles", lat: 34.0522, lng: -118.2437, tz: "America/Los_Angeles" },
  { group: "North America", name: "Toronto", lat: 43.6532, lng: -79.3832, tz: "America/Toronto" },
  { group: "North America", name: "Montreal", lat: 45.5017, lng: -73.5673, tz: "America/Toronto" },
  { group: "North America", name: "Mexico City", lat: 19.4326, lng: -99.1332, tz: "America/Mexico_City" },
  { group: "Europe", name: "London", lat: 51.5074, lng: -0.1278, tz: "Europe/London" },
  { group: "Europe", name: "Manchester", lat: 53.4808, lng: -2.2426, tz: "Europe/London" },
  { group: "Europe", name: "Paris", lat: 48.8566, lng: 2.3522, tz: "Europe/Paris" },
  { group: "Europe", name: "Antwerp", lat: 51.2194, lng: 4.4025, tz: "Europe/Brussels" },
  { group: "Europe", name: "Amsterdam", lat: 52.3676, lng: 4.9041, tz: "Europe/Amsterdam" },
  { group: "Europe", name: "Berlin", lat: 52.52, lng: 13.405, tz: "Europe/Berlin" },
  { group: "Europe", name: "Moscow", lat: 55.7558, lng: 37.6173, tz: "Europe/Moscow" },
  { group: "Elsewhere", name: "Johannesburg", lat: -26.2041, lng: 28.0473, tz: "Africa/Johannesburg" },
  { group: "Elsewhere", name: "Melbourne", lat: -37.8136, lng: 144.9631, tz: "Australia/Melbourne" },
  { group: "Elsewhere", name: "Sydney", lat: -33.8688, lng: 151.2093, tz: "Australia/Sydney" },
  { group: "Elsewhere", name: "Buenos Aires", lat: -34.6037, lng: -58.3816, tz: "America/Argentina/Buenos_Aires" },
  { group: "Elsewhere", name: "São Paulo", lat: -23.5505, lng: -46.6333, tz: "America/Sao_Paulo" },
];

const HEBREW_MONTHS = [
  ["Tishrei", "Tishrei"], ["Cheshvan", "Cheshvan"], ["Kislev", "Kislev"], ["Teves", "Tevet"],
  ["Shevat", "Shevat"], ["AdarI", "Adar I"], ["Adar", "Adar (Adar II in a leap year)"],
  ["Nisan", "Nisan"], ["Iyar", "Iyar"], ["Sivan", "Sivan"], ["Tammuz", "Tammuz"], ["Av", "Av"], ["Elul", "Elul"],
];

const ZMANIM = [
  ["alot_hashachar", "Dawn", "עלות השחר"],
  ["misheyakir", "Earliest tallit", "משיכיר"],
  ["sunrise", "Sunrise", "הנץ החמה", true],
  ["sof_zman_shema_mga", "Latest Shema (MGA)", "סוף זמן ק״ש מג״א"],
  ["sof_zman_shema_gra", "Latest Shema (GRA)", "סוף זמן ק״ש גר״א", true],
  ["sof_zman_tefila_mga", "Latest Shacharit (MGA)", "סוף זמן תפילה מג״א"],
  ["sof_zman_tefila_gra", "Latest Shacharit (GRA)", "סוף זמן תפילה גר״א"],
  ["chatzot", "Midday", "חצות", true],
  ["mincha_gedola", "Earliest Mincha", "מנחה גדולה"],
  ["mincha_ketana", "Mincha Ketana", "מנחה קטנה"],
  ["plag_hamincha", "Plag HaMincha", "פלג המנחה"],
  ["sunset", "Sunset", "שקיעה", true],
  ["tzeit_hakochavim", "Nightfall", "צאת הכוכבים", true],
  ["tzeit_72_min", "Nightfall (72 min)", "צאת ר״ת"],
];

const MONTH_NAMES = ["January", "February", "March", "April", "May", "June", "July",
  "August", "September", "October", "November", "December"];

// ── state ───────────────────────────────────────────────
const $ = (id) => document.getElementById(id);
const state = {
  settings: loadSettings(),
  month: null,      // first of the shown month, "YYYY-MM-01"
  selected: null,   // "YYYY-MM-DD"
  days: new Map(),  // iso → DailyData for the shown grid
  loading: 0,
};

function loadSettings() {
  try {
    const saved = JSON.parse(localStorage.getItem("hebrew-calendar.settings") || "null");
    if (saved && typeof saved === "object") return saved;
  } catch (_) { /* first run, or storage unavailable */ }
  return guessSettings();
}

function saveSettings() {
  try { localStorage.setItem("hebrew-calendar.settings", JSON.stringify(state.settings)); } catch (_) {}
}

/** First run: the city named in the link (?city=Jerusalem), else the listed
 * city in this computer's time zone, if any. */
function guessSettings() {
  const tz = Intl.DateTimeFormat().resolvedOptions().timeZone;
  const named = new URLSearchParams(location.search).get("city");
  const city = CITIES.find((c) => named && c.name.toLowerCase() === named.toLowerCase())
    || CITIES.find((c) => c.tz === tz);
  const place = city ? cityPlace(city) : null;
  return {
    place,
    israel: city ? !!city.israel : tz === "Asia/Jerusalem",
    candles: city && city.candles ? city.candles : 18,
    theme: "auto",
  };
}

function cityPlace(c) {
  return { name: c.name, lat: c.lat, lng: c.lng, tz: c.tz, elevation: 0 };
}

// ── talking to the calendar ─────────────────────────────
const tauri = window.__TAURI__ && window.__TAURI__.core;

async function call(command, query) {
  if (tauri) return tauri.invoke(command, { query });
  const path = command.replace(/_/g, "-");
  const params = new URLSearchParams();
  for (const [k, v] of Object.entries(query)) if (v !== undefined && v !== null && v !== "") params.set(k, v);
  const response = await fetch(`api/v1/${path}?${params}`);
  const body = await response.json().catch(() => ({}));
  if (!response.ok) throw new Error(body.error || response.statusText);
  return body;
}

function placeQuery() {
  const s = state.settings;
  const q = { israel: !!s.israel, candles: s.candles };
  if (s.place) Object.assign(q, { lat: s.place.lat, lng: s.place.lng, tz: s.place.tz, elevation: s.place.elevation || 0, name: s.place.name });
  else q.nowhere = true;
  return q;
}

// ── dates ───────────────────────────────────────────────
const pad = (n) => String(n).padStart(2, "0");
const iso = (d) => `${d.getUTCFullYear()}-${pad(d.getUTCMonth() + 1)}-${pad(d.getUTCDate())}`;
const parse = (s) => { const [y, m, d] = s.split("-").map(Number); return new Date(Date.UTC(y, m - 1, d)); };
const addDays = (s, n) => { const d = parse(s); d.setUTCDate(d.getUTCDate() + n); return iso(d); };
const firstOfMonth = (s) => s.slice(0, 8) + "01";
const todayIso = () => { const t = new Date(); return `${t.getFullYear()}-${pad(t.getMonth() + 1)}-${pad(t.getDate())}`; };
function addMonths(first, n) {
  const d = parse(first);
  d.setUTCMonth(d.getUTCMonth() + n, 1);
  return iso(d);
}
function longDate(s) {
  const d = parse(s);
  return d.toLocaleDateString(undefined, { weekday: "long", day: "numeric", month: "long", year: "numeric", timeZone: "UTC" });
}
function shortDate(s) {
  return parse(s).toLocaleDateString(undefined, { weekday: "short", day: "numeric", month: "short", timeZone: "UTC" });
}

// ── rendering ───────────────────────────────────────────
function el(tag, attrs = {}, ...children) {
  const e = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) {
    if (v === undefined || v === null || v === false) continue;
    if (k === "class") e.className = v;
    else if (k === "text") e.textContent = v;
    else if (k.startsWith("on")) e.addEventListener(k.slice(2), v);
    else e.setAttribute(k, v === true ? "" : v);
  }
  for (const c of children.flat()) if (c !== null && c !== undefined && c !== false) e.append(c);
  return e;
}

async function showMonth(first, select) {
  state.month = first;
  const start = addDays(first, -parse(first).getUTCDay());
  const lastOfMonth = addDays(addMonths(first, 1), -1);
  const end = addDays(lastOfMonth, 6 - parse(lastOfMonth).getUTCDay());
  state.selected = select || (state.selected && state.selected.slice(0, 7) === first.slice(0, 7) ? state.selected : first);
  const ticket = ++state.loading;
  try {
    const days = await call("days", { start, end, ...placeQuery() });
    if (ticket !== state.loading) return;
    state.days = new Map(days.map((d) => [d.gregorian.iso_string, d]));
    renderTitles();
    renderGrid();
    renderDetail();
  } catch (e) {
    toast(String(e.message || e));
  }
}

function renderTitles() {
  const d = parse(state.month);
  $("month-title").textContent = `${MONTH_NAMES[d.getUTCMonth()]} ${d.getUTCFullYear()}`;
  const inMonth = [...state.days.values()].filter((x) => x.gregorian.iso_string.slice(0, 7) === state.month.slice(0, 7));
  const first = inMonth[0], last = inMonth[inMonth.length - 1];
  const en = (x) => x.hebrew_display.split(" ").slice(1, -1).join(" ");
  const he = (x) => x.hebrew_display_he.split(" ").slice(1, -1).join(" ");
  const year = (x) => x.hebrew_display.split(" ").slice(-1)[0];
  const heYear = (x) => x.hebrew_display_he.split(" ").slice(-1)[0];
  if (en(first) === en(last)) {
    $("hebrew-span").textContent = `${en(first)} ${year(first)}`;
    $("hebrew-span-he").textContent = `${he(first)} ${heYear(first)}`;
  } else if (year(first) === year(last)) {
    $("hebrew-span").textContent = `${en(first)} – ${en(last)} ${year(last)}`;
    $("hebrew-span-he").textContent = `${he(first)} – ${he(last)} ${heYear(last)}`;
  } else {
    $("hebrew-span").textContent = `${en(first)} ${year(first)} – ${en(last)} ${year(last)}`;
    $("hebrew-span-he").textContent = `${he(first)} ${heYear(first)} – ${he(last)} ${heYear(last)}`;
  }
  const p = state.settings.place;
  $("place-name").textContent = p ? `${p.name || "Custom place"}${state.settings.israel ? " · Israel" : ""}` : "Choose a place";
}

function renderGrid() {
  const grid = $("grid");
  grid.replaceChildren();
  const today = todayIso();
  const monthKey = state.month.slice(0, 7);
  for (const day of state.days.values()) {
    const g = day.gregorian.iso_string;
    const hebDay = day.hebrew_display_he.split(" ")[0];
    const showMonth = day.hebrew.day === 1 || g === [...state.days.keys()][0];
    const chips = day.holidays.filter((h) => h.category !== "erev" || day.holidays.length === 1);
    const cls = ["day", day.is_shabbat && "shabbat", g.slice(0, 7) !== monthKey && "outside",
      g === today && "today", g === state.selected && "selected"].filter(Boolean).join(" ");
    const label = `${longDate(g)}, ${day.hebrew_display}${day.holidays.length ? ", " + day.holidays.map((h) => h.name).join(", ") : ""}`;
    const cell = el("button", { class: cls, role: "gridcell", "aria-label": label, "aria-selected": g === state.selected ? "true" : "false",
      "data-date": g, onclick: () => select(g) },
      el("div", { class: "top" },
        el("span", { class: "g", text: String(day.gregorian.day) }),
        el("span", { class: "h", text: hebDay })),
      showMonth ? el("div", { class: "hm", text: day.hebrew_display.split(" ").slice(1, -1).join(" ") }) : null,
      el("div", { class: "chips" },
        chips.slice(0, 3).map((h) => el("span", { class: `chip ${h.category}`, title: `${h.name} · ${h.hebrew}`, text: shortName(h.name) })),
        chips.length > 3 ? el("span", { class: "more", text: `+${chips.length - 3} more` }) : null),
      footer(day));
    grid.append(cell);
  }
}

const ICONS = {
  candle: '<svg viewBox="0 0 12 16" aria-hidden="true"><path d="M6 1.2c1.3 1.5 1.9 2.6 1.9 3.5A1.9 1.9 0 0 1 6 6.6a1.9 1.9 0 0 1-1.9-1.9c0-.9.6-2 1.9-3.5z" fill="currentColor"/><rect x="3.8" y="7.6" width="4.4" height="7.4" rx="1" fill="currentColor" opacity=".55"/></svg>',
  stars: '<svg viewBox="0 0 16 16" aria-hidden="true"><path d="M8 1.5l1.5 3.4 3.7.3-2.8 2.4.9 3.6L8 9.3l-3.3 1.9.9-3.6-2.8-2.4 3.7-.3z" fill="currentColor"/></svg>',
};

function timeWithIcon(icon, time, title, cls) {
  const span = el("span", { class: `timed ${cls || ""}`, title });
  span.innerHTML = ICONS[icon];
  span.append(time);
  return span;
}

/** Chips name the day briefly; the colour already says Chol HaMoed. */
const shortName = (name) => name.replace(" (Chol HaMoed)", "").replace("Chanukah: ", "Chanukah ");

function footer(day) {
  const parts = [];
  if (day.is_shabbat && day.parsha) parts.push(el("span", { class: "parsha", text: day.parsha.name }));
  if (day.candle_lighting) parts.push(timeWithIcon("candle", day.candle_lighting, "Candle lighting", "candle"));
  else if (day.havdalah) parts.push(timeWithIcon("stars", day.havdalah, "Havdalah"));
  return parts.length ? el("div", { class: "foot" }, parts) : null;
}

function select(g) {
  if (g.slice(0, 7) !== state.month.slice(0, 7)) {
    showMonth(firstOfMonth(g), g);
    return;
  }
  state.selected = g;
  for (const c of $("grid").children) {
    const on = c.dataset.date === g;
    c.classList.toggle("selected", on);
    c.setAttribute("aria-selected", on ? "true" : "false");
  }
  renderDetail();
}

function section(title, ...children) {
  return el("div", { class: "section" }, el("h3", { text: title }), ...children);
}

function renderDetail() {
  try { history.replaceState(null, "", `#${state.selected}`); } catch (_) {}
  const day = state.days.get(state.selected);
  const panel = $("detail");
  if (!day) { panel.replaceChildren(); return; }
  const flags = [];
  if (day.is_shabbat) flags.push(el("span", { class: "chip special_shabbat", text: "Shabbat" }));
  if (day.is_yom_tov) flags.push(el("span", { class: "chip yom_tov", text: "Yom Tov" }));
  if (day.is_fast) flags.push(el("span", { class: "chip fast", text: "Fast day" }));

  const blocks = [
    el("div", { class: "hero" },
      el("div", { class: "he-date", text: day.hebrew_display_he }),
      el("div", { class: "en-date", text: day.hebrew_display }),
      el("div", { class: "greg", text: longDate(day.gregorian.iso_string) }),
      flags.length ? el("div", { class: "flags" }, flags) : null),
  ];

  // Candle lighting and havdalah
  const cards = [];
  if (day.candle_lighting) {
    cards.push(el("div", { class: "time-card" },
      el("div", { class: "label", text: "Candle lighting" }),
      el("div", { class: "value", text: day.candle_lighting }),
      el("div", { class: "sub", text: day.candle_lighting_after_nightfall ? "after nightfall, from an existing flame" : `${state.settings.candles} min before sunset` })));
  }
  if (day.havdalah) {
    cards.push(el("div", { class: "time-card" },
      el("div", { class: "label", text: day.is_shabbat ? "Shabbat ends" : "Festival ends" }),
      el("div", { class: "value", text: day.havdalah }),
      el("div", { class: "sub", text: "Havdalah, at nightfall" })));
  }
  if (cards.length) blocks.push(el("div", { class: "section" }, el("div", { class: "times" }, cards)));

  if (day.holidays.length) {
    blocks.push(section("Today", day.holidays.map((h) => el("div", { class: `item holiday ${h.category}` },
      el("span", { class: "name", text: h.name }), el("span", { class: "hebrew", text: h.hebrew })))));
  }

  const reading = day.is_shabbat ? day.parsha : day.week_parsha;
  if (reading) {
    blocks.push(section(day.is_shabbat ? "Torah reading" : "This week's portion",
      el("div", { class: "item" }, el("span", { class: "name", text: `Parashat ${reading.name}` }), el("span", { class: "hebrew", text: `פרשת ${reading.hebrew}` }))));
  } else if (day.is_shabbat) {
    blocks.push(section("Torah reading", el("p", { class: "note", text: "The festival's own reading replaces the weekly portion." })));
  }

  if (day.chanukah_candles) {
    const candles = [];
    for (let i = 8; i >= 1; i--) {
      if (i === 4) candles.push(el("div", { class: "c shamash", title: "Shamash" }));
      candles.push(el("div", { class: `c${i <= day.chanukah_candles ? " lit" : ""}` }));
    }
    blocks.push(section("Chanukah",
      el("div", { class: "menorah", role: "img", "aria-label": `${day.chanukah_candles} candles tonight` }, candles),
      el("p", { class: "note", text: `Light ${day.chanukah_candles} candle${day.chanukah_candles > 1 ? "s" : ""} tonight, plus the shamash.` })));
  }

  if (day.omer) {
    blocks.push(section("Counting the Omer",
      el("div", { class: "omer" },
        el("span", { text: `Day ${day.omer} of 49` }),
        el("div", { class: "bar-track" }, el("div", { class: "bar-fill", style: `width:${(day.omer / 49) * 100}%` }))),
      el("p", { class: "note", text: "Counted the evening before." })));
  }

  if (day.mevarchim) {
    const m = day.mevarchim;
    blocks.push(section(`Shabbat Mevarchim · ${m.month}`,
      el("div", { class: "item" }, el("span", { class: "name", text: `Rosh Chodesh ${m.month}` }), el("span", { class: "hebrew", text: `ראש חודש ${m.month_hebrew}` })),
      el("p", { class: "note", text: m.announcement })));
  }

  if (day.daf_yomi) {
    blocks.push(section("Daf Yomi",
      el("div", { class: "item" }, el("span", { class: "name", text: day.daf_yomi.name }), el("span", { class: "hebrew", text: day.daf_yomi.hebrew }))));
  }

  if (day.zmanim) {
    const z = day.zmanim;
    const rows = ZMANIM.filter(([k]) => z[k]).map(([k, en, he, key]) =>
      el("tr", { class: key ? "key" : null }, el("td", { text: en }), el("td", { class: "he", text: he }), el("td", { class: "t", text: z[k] })));
    const where = `${z.location.location_name || "Custom place"} · UTC${z.utc_offset}`;
    blocks.push(section("Zmanim",
      el("p", { class: "note", text: where }),
      rows.length ? el("table", { class: "zmanim" }, el("tbody", {}, rows))
        : el("p", { class: "empty", text: "The sun does not rise or set here today." })));
  } else {
    blocks.push(section("Zmanim",
      el("p", { class: "empty", text: "Choose a place to see prayer times and candle lighting." }),
      el("button", { class: "pill primary cta", text: "Choose a place", onclick: openSettings })));
  }

  const upcoming = el("div", { class: "upcoming" }, el("p", { class: "empty", text: "…" }));
  blocks.push(section("Coming up", upcoming));
  panel.replaceChildren(...blocks);
  fillUpcoming(upcoming, day.gregorian.iso_string);
}

async function fillUpcoming(target, from) {
  try {
    const list = await call("holidays", { start: addDays(from, 1), end: addDays(from, 75), israel: !!state.settings.israel });
    const shown = [];
    const seen = new Set();
    for (const h of list) {
      if (["erev", "rosh_chodesh", "special_shabbat"].includes(h.category)) continue;
      // One line per festival: "Sukkot II…VII" collapse into the first day.
      const family = h.name.replace(/ (I|II|III|IV|V|VI|VII|VIII)( \(.*\))?$/, "").replace(/: .*$/, "");
      if (seen.has(family)) continue;
      seen.add(family);
      shown.push(h);
      if (shown.length === 7) break;
    }
    target.replaceChildren(...(shown.length ? shown.map((h) => el("div", { class: `item holiday ${h.category}` },
      el("span", { class: "name", text: h.name }),
      el("button", { class: "link when", text: shortDate(h.date), onclick: () => select(h.date) })))
      : [el("p", { class: "empty", text: "Nothing in the next ten weeks." })]));
  } catch (e) {
    target.replaceChildren(el("p", { class: "empty", text: String(e.message || e) }));
  }
}

// ── settings ────────────────────────────────────────────
function fillCities() {
  const select = $("city");
  select.replaceChildren(el("option", { value: "", text: "No place (dates only)" }));
  const groups = {};
  for (const [i, c] of CITIES.entries()) {
    groups[c.group] = groups[c.group] || el("optgroup", { label: c.group });
    groups[c.group].append(el("option", { value: String(i), text: c.name }));
  }
  select.append(...Object.values(groups), el("option", { value: "custom", text: "Custom place…" }));
  const zones = (Intl.supportedValuesOf ? Intl.supportedValuesOf("timeZone") : []);
  $("zones").replaceChildren(...zones.map((z) => el("option", { value: z })));
}

function openSettings() {
  const s = state.settings;
  $("settings-error").textContent = "";
  const idx = s.place ? CITIES.findIndex((c) => c.name === s.place.name && c.lat === s.place.lat) : -1;
  $("city").value = s.place ? (idx >= 0 ? String(idx) : "custom") : "";
  const p = s.place || {};
  $("c-name").value = idx >= 0 ? "" : (p.name || "");
  $("c-lat").value = idx >= 0 ? "" : (p.lat ?? "");
  $("c-lng").value = idx >= 0 ? "" : (p.lng ?? "");
  $("c-tz").value = idx >= 0 ? "" : (p.tz || Intl.DateTimeFormat().resolvedOptions().timeZone);
  $("c-elev").value = idx >= 0 ? "" : (p.elevation || "");
  $("custom").classList.toggle("on", $("city").value === "custom");
  document.querySelector(`input[name=observance][value=${s.israel ? "israel" : "diaspora"}]`).checked = true;
  $("candles").value = String(s.candles);
  $("theme").value = s.theme || "auto";
  $("settings").showModal();
}

$("city").addEventListener("change", () => {
  const v = $("city").value;
  $("custom").classList.toggle("on", v === "custom");
  const c = CITIES[Number(v)];
  if (v !== "" && v !== "custom" && c) {
    document.querySelector(`input[name=observance][value=${c.israel ? "israel" : "diaspora"}]`).checked = true;
    $("candles").value = String(c.candles || 18);
  }
});

$("locate").addEventListener("click", () => {
  if (!navigator.geolocation) { $("settings-error").textContent = "This browser cannot share its location."; return; }
  $("settings-error").textContent = "Finding you…";
  navigator.geolocation.getCurrentPosition((pos) => {
    $("city").value = "custom";
    $("custom").classList.add("on");
    $("c-name").value = "My location";
    $("c-lat").value = pos.coords.latitude.toFixed(4);
    $("c-lng").value = pos.coords.longitude.toFixed(4);
    $("c-tz").value = Intl.DateTimeFormat().resolvedOptions().timeZone;
    $("c-elev").value = pos.coords.altitude ? Math.round(pos.coords.altitude) : "";
    $("settings-error").textContent = "";
  }, (err) => { $("settings-error").textContent = `Could not get your location: ${err.message}`; },
  { timeout: 15000 });
});

// Save and Go act on the button itself, not on the dialog's close event,
// which a hidden page may not deliver.
$("save").addEventListener("click", async (e) => {
  e.preventDefault();
  const v = $("city").value;
  let place = null;
  if (v === "custom") {
    const lat = Number($("c-lat").value), lng = Number($("c-lng").value);
    const tz = $("c-tz").value.trim();
    if (!$("c-lat").value || !$("c-lng").value || !isFinite(lat) || !isFinite(lng) || Math.abs(lat) > 90 || Math.abs(lng) > 180) {
      $("settings-error").textContent = "Latitude must be −90 to 90 and longitude −180 to 180.";
      return;
    }
    place = { name: $("c-name").value.trim() || "Custom place", lat, lng, tz: tz || "UTC", elevation: Number($("c-elev").value) || 0 };
  } else if (v !== "") {
    place = cityPlace(CITIES[Number(v)]);
  }
  const previous = state.settings;
  state.settings = {
    place,
    israel: document.querySelector("input[name=observance]:checked").value === "israel",
    candles: Number($("candles").value),
    theme: $("theme").value,
  };
  applyTheme();
  try {
    await call("day", { date: state.selected, ...placeQuery() }); // validates the time zone
    saveSettings();
    $("settings").close();
    showMonth(state.month, state.selected);
  } catch (err) {
    state.settings = previous;
    applyTheme();
    $("settings-error").textContent = String(err.message || err);
  }
});

function applyTheme() {
  const t = state.settings.theme || "auto";
  if (t === "auto") document.documentElement.removeAttribute("data-theme");
  else document.documentElement.setAttribute("data-theme", t);
}

// ── go to a date ────────────────────────────────────────
function openGoto() {
  $("goto-error").textContent = "";
  $("g-date").value = state.selected;
  const day = state.days.get(state.selected);
  $("h-day").value = "";
  $("h-year").value = day ? day.hebrew.year : "";
  $("h-month").value = day ? day.hebrew.month : "Tishrei";
  $("goto").showModal();
}

$("go").addEventListener("click", async (e) => {
  e.preventDefault();
  try {
    let target = $("g-date").value;
    if ($("h-day").value) {
      const r = await call("hebrew_to_gregorian", { year: Number($("h-year").value), month: $("h-month").value, day: Number($("h-day").value) });
      target = r.date;
    }
    if (!/^\d{4}-\d{2}-\d{2}$/.test(target || "")) throw new Error("Enter a date.");
    $("goto").close();
    showMonth(firstOfMonth(target), target);
  } catch (err) {
    $("goto-error").textContent = String(err.message || err);
  }
});

window.addEventListener("hashchange", () => {
  const m = /^#(\d{4}-\d{2}-\d{2})$/.exec(location.hash);
  if (m && m[1] !== state.selected) select(m[1]);
});

// ── misc ────────────────────────────────────────────────
let toastTimer;
function toast(message) {
  const t = $("toast");
  t.textContent = message;
  t.classList.add("show");
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => t.classList.remove("show"), 4500);
}

document.addEventListener("keydown", (e) => {
  if (document.querySelector("dialog[open]") || e.altKey || e.ctrlKey || e.metaKey) return;
  if (["INPUT", "SELECT", "TEXTAREA"].includes(e.target.tagName)) return;
  const move = { ArrowLeft: -1, ArrowRight: 1, ArrowUp: -7, ArrowDown: 7 }[e.key];
  if (move) { e.preventDefault(); select(addDays(state.selected, move)); focusSelected(); return; }
  if (e.key === "PageUp") { e.preventDefault(); showMonth(addMonths(state.month, -1)); }
  else if (e.key === "PageDown") { e.preventDefault(); showMonth(addMonths(state.month, 1)); }
  else if (e.key === "t" || e.key === "T") goToday();
  else if (e.key === "g" || e.key === "G") { e.preventDefault(); openGoto(); }
});

function focusSelected() {
  requestAnimationFrame(() => {
    const c = document.querySelector(`.day[data-date="${state.selected}"]`);
    if (c) c.focus({ preventScroll: true });
  });
}

function goToday() {
  const t = todayIso();
  showMonth(firstOfMonth(t), t);
}

$("prev").addEventListener("click", () => showMonth(addMonths(state.month, -1)));
$("next").addEventListener("click", () => showMonth(addMonths(state.month, 1)));
$("today").addEventListener("click", goToday);
$("place-btn").addEventListener("click", openSettings);
$("goto-btn").addEventListener("click", openGoto);
$("h-month").replaceChildren(...HEBREW_MONTHS.map(([v, t]) => el("option", { value: v, text: t })));

fillCities();
applyTheme();
const linked = /^#(\d{4}-\d{2}-\d{2})$/.exec(location.hash);
if (linked) showMonth(firstOfMonth(linked[1]), linked[1]);
else goToday();
