# MissionDesk

MissionDesk is a planning and briefing tool for unmanned aircraft (UAS)
operations in the EASA **Specific category**. It walks a remote pilot or a
small operator through a SORA 2.5 assessment and collects the result, together
with weather, airspace and contingency information, in an operations briefing
and an operational flight plan.

It runs as a Windows desktop application (built with Tauri) and as a web
application in the browser. Both are the same code.

> **Disclaimer.** MissionDesk is a planning aid. It is **not** certified,
> approved or endorsed by EASA or by any national aviation authority, and its
> output is **not** an operational authorisation or evidence that one would be
> granted. It does not provide legal or regulatory advice, its calculations and
> built-in reference data may contain errors or be out of date, and it comes
> with **no warranty** of any kind (see [LICENSE](LICENSE)). The UAS operator
> and the remote pilot remain solely responsible for the risk assessment, for
> complying with all applicable regulations and for the safe conduct of the
> flight. Always check the current rules and aeronautical information at the
> source.

## What it does

- **Step 1 - Detailed operational information.** Operator, aircraft, crew and
  the operation itself, with a built-in list of common UAS models.
- **Step 2 - Volume planner.** Flight geography, contingency volume and ground
  risk buffer drawn on a map.
- **Steps 2-7 - iGRC, GRC, ARC and SAIL.** Intrinsic and final ground risk
  class, air risk class and the resulting SAIL.
- **Step 8 - Containment** and **Step 9 - OSO**, the 17 operational safety
  objectives.
- **Weather and airspace.** METAR, TAF and NOTAM lookups, forecast data and an
  airspace overlay on the maps.
- **Emergency and contingency plan**, and **threat and error management**.
- **Charts and annexes**, including PDF import.
- **Operations briefing** and **operational flight plan**, ready to print or
  save as PDF.
- **Plans as files.** A plan is saved to, and opened from, a `.dbrf` file on
  your own computer.

### Regulatory context

MissionDesk is designed around Commission Implementing Regulation
(EU) 2019/947 and the EASA Easy Access Rules for Unmanned Aircraft Systems, in
which **SORA 2.5** is AMC1 to Article 11. The application helps you work
through that method and document the result; it does not decide or guarantee
compliance.

### Status

The risk assessment shown in the application is calculated by `app/js/igrc.js`
(ground risk) and `app/js/arc.js` (air risk), together with logic in
`app/js/app.js`. The modules `category.js`, `containment.js`, `oso.js` and the
orchestrator `sora.js` are a newer, stand-alone implementation of the full
pipeline. They are part of the repository but are **not yet loaded** by the
user interface.

## Your data

MissionDesk has no server and no account. Everything you enter stays in your
browser or in the desktop application (local storage) and in the plan files
you save yourself.

The application contacts third-party services only to fetch map tiles,
weather and airspace data, and only for the coordinates and aerodromes you
are working with:

| Service | Used for | API key |
|---|---|---|
| Esri World Topographic Map | Base map | none |
| OpenAIP | Airspace overlay | your own (free account) |
| CheckWX | METAR, TAF, NOTAM | your own (free account) |
| Open-Meteo | Forecast data | none |
| LFV, Lantmäteriet, SCB (Sweden) | Aeronautical, topographic and population layers | none |
| ChartBundle | Aeronautical chart layer | none |
| OpenStreetMap | Embedded overview map | none |
| Google Fonts | Typefaces | none |

API keys are stored in local storage on your own machine and are sent only to
the service they belong to. Each service has its own terms of use, which you
are responsible for following.

## Running it

### Web version

The application is the static folder `app/`; there is no build step. Serve it
with any static web server, for example:

```bash
npx serve app
```

### Desktop application

You need Node.js 20, a stable Rust toolchain and the
[Tauri v1 prerequisites](https://v1.tauri.app/v1/guides/getting-started/prerequisites)
for your platform.

```bash
npm install
npx tauri dev      # run in a development window
npx tauri build    # build the installer
```

The version shown in the application is set in one place,
`app/js/version.js`. The build workflow in `.github/workflows/build.yml`
stamps the same version into the installer.

## Built-in reference data

`app/js/data.js` contains a list of UAS models with their characteristics,
and aerodrome names, positions, radio frequencies and telephone numbers.
This data was compiled by hand as a convenience. It is **not** an official
source and may be incomplete or out of date. Verify aircraft data against the
manufacturer's documentation and aerodrome data against the current AIP.

## Licence

MissionDesk is licensed under the **GNU Affero General Public License,
version 3 only** (`AGPL-3.0-only`). The full text is in [LICENSE](LICENSE).
Copyright (C) 2026 Lars Wallström.

Under section 13 of the licence, if you run a **modified** version and let
users interact with it over a network, you must offer those users the
corresponding source of your version. The application shows a
"SOURCE · AGPL-3.0" link in the header; set `window.MISSIONDESK_SOURCE_URL`
in `app/js/version.js` so that the link points at your own source.

If you cannot or do not wish to meet the obligations of the AGPL, a
commercial licence is available: see
[COMMERCIAL-LICENCE.md](COMMERCIAL-LICENCE.md).

Third-party components and their licences are listed in
[THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md). The libraries under
`app/vendor/` are unmodified upstream builds, each with its own licence file.

## Contributing, security and trademarks

- [CONTRIBUTING.md](CONTRIBUTING.md) - development setup, conventions and
  the contributor licence agreement ([CLA.md](CLA.md)).
- [SECURITY.md](SECURITY.md) - how to report a vulnerability privately.
- [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) - expected behaviour.
- [TRADEMARKS.md](TRADEMARKS.md) - use of the MissionDesk name and logo.
