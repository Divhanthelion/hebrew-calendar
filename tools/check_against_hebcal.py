"""Compare hebrew_core with Hebcal, day by day.

    python tools/check_against_hebcal.py 1950 2080

Fetches Hebcal's calendar for each Gregorian year (diaspora and Israel),
caches it in .hebcal-cache/, runs the `dump` example for the same days, and
compares: the Hebrew date of every day, every holiday, fast, special Shabbat
and Rosh Chodesh, the weekly Torah reading of every Shabbat, the Daf Yomi of
every day, and the molad announced on every Shabbat Mevarchim
(outside Israel; Hebcal's Israel calendar does not list it).

Exits non-zero if anything differs. Hebcal (https://www.hebcal.com) is free;
the first run makes two requests per year, half a second apart.
"""

import collections
import json
import os
import re
import subprocess
import sys
import time
import urllib.request

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CACHE = os.path.join(ROOT, ".hebcal-cache")
URL = ("https://www.hebcal.com/hebcal?v=1&cfg=json&maj=on&min=on&mod=on&nx=on&mf=on"
       "&ss=on&s=on&F=on&i={i}&start={y}-01-01&end={y}-12-31")


def hebcal_year(year, israel):
    os.makedirs(CACHE, exist_ok=True)
    path = os.path.join(CACHE, f"{'il' if israel else 'diaspora'}-{year}.json")
    if not os.path.exists(path):
        url = URL.format(i="on" if israel else "off", y=year)
        req = urllib.request.Request(url, headers={"User-Agent": "hebrew-calendar-check/1.0"})
        with urllib.request.urlopen(req, timeout=60) as r:
            data = r.read()
        with open(path, "wb") as f:
            f.write(data)
        time.sleep(0.5)
    with open(path, encoding="utf-8") as f:
        return json.load(f)["items"]


def key(name):
    """Fold spelling differences between the two calendars."""
    n = name.replace("’", "'").lower()
    n = re.sub(r"\(.*?\)", "", n)
    n = re.sub(r"[^a-z0-9 ]", "", n)
    for a, b in [("hashanah", "hashana"), ("shemini", "shmini"), ("iyyar", "iyar"),
                 ("tevet", "teves"), ("shvat", "shevat"), ("tamuz", "tammuz")]:
        n = re.sub(rf"\b{a}\b", b, n)
    return " ".join(n.split())


def hebcal_holiday(title, israel):
    """Hebcal title -> the calendar's name, folded; None for titles not compared."""
    t = title.replace("’", "'")
    if re.fullmatch(r"Rosh Hashana \d+", t):
        return key("Rosh Hashanah I")
    if t == "Sukkot VII (Hoshana Raba)":
        return key("Hoshana Rabbah")
    if israel and t == "Shavuot":
        return key("Shavuot I")
    if t.startswith("Chanukah:"):
        return None  # compared by candle count below
    return key(t)


PARSHA_ALIASES = {"behaalotecha": "behaalotcha", "shemini": "shmini", "shelach": "shlach",
                  "lech lecha": "lechlecha", "lechlecha": "lechlecha"}


def parsha_key(name):
    n = name.replace("Parashat ", "").replace("’", "").replace("'", "").lower()
    parts = [re.sub(r"[^a-z ]", "", p).strip() for p in n.replace("lech-lecha", "lech lecha").split("-")]
    return "-".join(PARSHA_ALIASES.get(p, p).replace(" ", "") for p in parts)


TRACTATES = {"Berachos": "Berachot", "Shabbos": "Shabbat", "Taanis": "Taanit", "Megilah": "Megillah",
             "Sukah": "Sukkah", "Rosh Hashanah": "Rosh Hashana", "Yevamos": "Yevamot",
             "Kesuvos": "Ketubot", "Kidushin": "Kiddushin", "Bava Kama": "Baba Kamma",
             "Bava Metzia": "Baba Metzia", "Bava Basra": "Baba Batra", "Makos": "Makkot",
             "Shevuos": "Shevuot", "Horayos": "Horayot", "Menachos": "Menachot", "Chulin": "Chullin",
             "Bechoros": "Bechorot", "Erchin": "Arachin", "Kerisus": "Keritot", "Me'ilah": "Meilah",
             "Kinim": "Kinnim", "Midos": "Midot", "Nidah": "Niddah"}

MONTHS = {"Iyyar": "Iyar", "Tamuz": "Tammuz", "Tevet": "Teves", "Sh'vat": "Shevat", "Sh’vat": "Shevat"}


def main():
    first, last = int(sys.argv[1]), int(sys.argv[2])
    failures = collections.Counter()
    examples = collections.defaultdict(list)
    counts = collections.Counter()

    def fail(kind, *detail):
        failures[kind] += 1
        if len(examples[kind]) < 5:
            examples[kind].append(detail)

    for observance in ("diaspora", "israel"):
        israel = observance == "israel"
        dump = subprocess.run(
            ["cargo", "run", "-q", "--release", "-p", "hebrew_core", "--example", "dump", "--",
             f"{first}-01-01", f"{last}-12-31", observance],
            cwd=ROOT, check=True, capture_output=True, text=True, encoding="utf-8").stdout
        ours = {d["g"]: d for d in map(json.loads, dump.splitlines())}
        theirs = collections.defaultdict(list)
        for year in range(first, last + 1):
            for item in hebcal_year(year, israel):
                theirs[item["date"][:10]].append(item)

        for g, day in ours.items():
            items = theirs.get(g, [])
            # Hebrew date, from any event's hdate.
            for it in items:
                if it.get("hdate"):
                    d, *month, y = it["hdate"].split()
                    month = " ".join(month)
                    want = (int(d), MONTHS.get(month, month), int(y))
                    got = (day["hd"], day["hm"], day["hy"])
                    counts["hebrew date"] += 1
                    if want != got:
                        fail("hebrew date", observance, g, want, got)
                    break
            # Holidays.
            want = {hebcal_holiday(it["title"], israel) for it in items
                    if it["category"] in ("holiday", "roshchodesh")} - {None}
            got = {key(h) for h in day["hol"] if not h.startswith("Chanukah")}
            if israel:
                got.discard(key("Simchat Torah")) if key("Shemini Atzeret") in got else None
            counts["holiday days"] += 1
            if want != got:
                fail("holidays", observance, g, sorted(want - got), sorted(got - want))
            # Chanukah candles lit that evening.
            want = [int(re.search(r"\d+", it["title"]).group()) for it in items
                    if re.match(r"Chanukah: \d+ Candles?", it["title"])]
            got = [day["candles"]] if day["candles"] else []
            if want != got:
                fail("chanukah candles", observance, g, want, got)
            # Weekly reading.
            if day["parsha"] is not None:
                want = [parsha_key(it["title"]) for it in items if it["category"] == "parashat"]
                got = [parsha_key(day["parsha"])] if day["parsha"] else []
                counts["shabbatot"] += 1
                if want != got:
                    fail("parsha", observance, g, want, got)
            # Daf Yomi.
            want = [it["title"] for it in items if it["category"] == "dafyomi"]
            if want or day["daf"]:
                got = []
                if day["daf"]:
                    t, n = day["daf"].rsplit(" ", 1)
                    got = [f"{TRACTATES.get(t, t)} {n}"]
                counts["daf yomi"] += 1
                if want != got:
                    fail("daf yomi", observance, g, want, got)
            # Molad on Shabbat Mevarchim (Hebcal lists it outside Israel only).
            if israel:
                continue
            want = []
            for it in items:
                m = re.search(r"Molad ([\w’' ]+?): (\w+), (\d+):(\d+)(am|pm)(?: and (\d+) chalak)?",
                              it.get("memo", "")) if it["category"] == "mevarchim" else None
                if m:
                    month, wd, hh, mm, ap, ch = m.groups()
                    h24 = int(hh) % 12 + (12 if ap == "pm" else 0)
                    want = [(MONTHS.get(month, month).replace("Adar I", "AdarI"), wd[:3], h24, int(mm), int(ch or 0))]
            got = []
            if day["mevarchim"]:
                mv = day["mevarchim"]
                got = [(mv["month"].replace("Adar I", "AdarI"), mv["wd"], mv["h"], mv["min"], mv["ch"])]
            if want or got:
                counts["molad announcements"] += 1
                if want != got:
                    fail("molad", observance, g, want, got)

    print(f"Compared {first}-{last}, diaspora and Israel:")
    for kind, n in counts.items():
        print(f"  {kind}: {n}")
    if not failures:
        print("No differences.")
        return 0
    for kind, n in failures.items():
        print(f"{kind}: {n} differences")
        for e in examples[kind]:
            print("   ", *e)
    return 1


if __name__ == "__main__":
    sys.exit(main())
