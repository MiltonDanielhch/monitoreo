# 🛠️ Auditoría de Software — Lab 3030

> Generado: `2026-06-08 11:43`

## Resumen

| Métrica | Valor |
| :--- | :--- |
| **Proyecto** | `monitoreo` |
| **Líneas de Código (Netas)** | 56877 LoC |
| **Peso Total del Proyecto** | 4.05MB |
| **Timestamp** | 2026-06-08 11:43 |
| **Estado** | Activa |

## Breakdown por Capa

| Capa / Archivo | LoC | Peso | % LoC |
| :--- | ---: | ---: | ---: |
| `apps` | 39005 | 3.10MB | 68.6% █████████████ |
| `crates` | 10695 | 414.23KB | 18.8% ███ |
| `guia` | 5743 | 388.50KB | 10.1% ██ |
| `pnpm-lock.yaml` | 605 | 24.75KB | 1.1%  |
| `data` | 513 | 34.65KB | 0.9%  |
| `audit.py` | 146 | 5.43KB | 0.3%  |
| `tests` | 91 | 3.73KB | 0.2%  |
| `infrastructure` | 37 | 2.88KB | 0.1%  |
| `Cargo.toml` | 30 | 1.01KB | 0.1%  |
| `.mise.toml` | 7 | 183.00B | 0.0%  |
| `package.json` | 5 | 69.00B | 0.0%  |
| `.env.local` | 0 | 575.00B | 0.0%  |
| `.env.production.example` | 0 | 497.00B | 0.0%  |
| `Cargo.lock` | 0 | 95.59KB | 0.0%  |
| `justfile` | 0 | 786.00B | 0.0%  |
| **TOTAL** | **56877** | **4.05MB** | 100% |

## Mapa de Arquitectura

```text
monitoreo/
├── .env.local (0 LoC | 575.00B)
├── .env.production.example (0 LoC | 497.00B)
├── .mise.toml (7 LoC | 183.00B)
├── Cargo.lock (0 LoC | 95.59KB)
├── Cargo.toml (30 LoC | 1.01KB)
├── apps/ [3.10MB]
│   ├── api/ [3.93KB]
│   │   ├── Cargo.toml (17 LoC | 573.00B)
│   │   └── src/ [3.37KB]
│   │       └── main.rs (70 LoC | 3.37KB)
│   └── web/ [3.09MB]
│       ├── .npmrc (0 LoC | 19.00B)
│       ├── .prettierignore (0 LoC | 107.00B)
│       ├── .prettierrc (0 LoC | 230.00B)
│       ├── .svelte-kit/ [2.07MB]
│       │   ├── ambient.d.ts (388 LoC | 15.84KB)
│       │   ├── generated/ [14.78KB]
│       │   │   ├── client/ [4.36KB]
│       │   │   │   ├── app.js (61 LoC | 1.97KB)
│       │   │   │   ├── matchers.js (1 LoC | 27.00B)
│       │   │   │   └── nodes/ [2.37KB]
│       │   │   │       ├── 0.js (3 LoC | 163.00B)
│       │   │   │       ├── 1.js (1 LoC | 203.00B)
│       │   │   │       ├── 10.js (1 LoC | 100.00B)
│       │   │   │       ├── 11.js (1 LoC | 100.00B)
│       │   │   │       ├── 12.js (1 LoC | 98.00B)
│       │   │   │       ├── 13.js (1 LoC | 100.00B)
│       │   │   │       ├── 14.js (1 LoC | 99.00B)
│       │   │   │       ├── 15.js (1 LoC | 94.00B)
│       │   │   │       ├── 16.js (1 LoC | 99.00B)
│       │   │   │       ├── 17.js (1 LoC | 106.00B)
│       │   │   │       ├── 18.js (1 LoC | 93.00B)
│       │   │   │       ├── 19.js (1 LoC | 94.00B)
│       │   │   │       ├── 2.js (1 LoC | 87.00B)
│       │   │   │       ├── 20.js (1 LoC | 91.00B)
│       │   │   │       ├── 21.js (1 LoC | 94.00B)
│       │   │   │       ├── 22.js (1 LoC | 93.00B)
│       │   │   │       ├── 23.js (1 LoC | 81.00B)
│       │   │   │       ├── 3.js (1 LoC | 75.00B)
│       │   │   │       ├── 4.js (1 LoC | 85.00B)
│       │   │   │       ├── 5.js (1 LoC | 92.00B)
│       │   │   │       ├── 6.js (1 LoC | 97.00B)
│       │   │   │       ├── 7.js (1 LoC | 93.00B)
│       │   │   │       ├── 8.js (1 LoC | 91.00B)
│       │   │   │       └── 9.js (1 LoC | 95.00B)
│       │   │   ├── client-optimized/ [4.36KB]
│       │   │   │   ├── app.js (61 LoC | 1.97KB)
│       │   │   │   ├── matchers.js (1 LoC | 27.00B)
│       │   │   │   └── nodes/ [2.37KB]
│       │   │   │       ├── 0.js (3 LoC | 163.00B)
│       │   │   │       ├── 1.js (1 LoC | 203.00B)
│       │   │   │       ├── 10.js (1 LoC | 100.00B)
│       │   │   │       ├── 11.js (1 LoC | 100.00B)
│       │   │   │       ├── 12.js (1 LoC | 98.00B)
│       │   │   │       ├── 13.js (1 LoC | 100.00B)
│       │   │   │       ├── 14.js (1 LoC | 99.00B)
│       │   │   │       ├── 15.js (1 LoC | 94.00B)
│       │   │   │       ├── 16.js (1 LoC | 99.00B)
│       │   │   │       ├── 17.js (1 LoC | 106.00B)
│       │   │   │       ├── 18.js (1 LoC | 93.00B)
│       │   │   │       ├── 19.js (1 LoC | 94.00B)
│       │   │   │       ├── 2.js (1 LoC | 87.00B)
│       │   │   │       ├── 20.js (1 LoC | 91.00B)
│       │   │   │       ├── 21.js (1 LoC | 94.00B)
│       │   │   │       ├── 22.js (1 LoC | 93.00B)
│       │   │   │       ├── 23.js (1 LoC | 81.00B)
│       │   │   │       ├── 3.js (1 LoC | 75.00B)
│       │   │   │       ├── 4.js (1 LoC | 85.00B)
│       │   │   │       ├── 5.js (1 LoC | 92.00B)
│       │   │   │       ├── 6.js (1 LoC | 97.00B)
│       │   │   │       ├── 7.js (1 LoC | 93.00B)
│       │   │   │       ├── 8.js (1 LoC | 91.00B)
│       │   │   │       └── 9.js (1 LoC | 95.00B)
│       │   │   ├── root.js (3 LoC | 122.00B)
│       │   │   ├── root.svelte (67 LoC | 2.41KB)
│       │   │   └── server/ [3.53KB]
│       │   │       └── internal.js (46 LoC | 3.53KB)
│       │   ├── non-ambient.d.ts (58 LoC | 3.15KB)
│       │   ├── output/ [2.01MB]
│       │   │   ├── client/ [725.10KB]
│       │   │   │   ├── .vite/ [22.54KB]
│       │   │   │   │   └── manifest.json (834 LoC | 22.54KB)
│       │   │   │   ├── _app/ [702.51KB]
│       │   │   │   │   ├── immutable/ [702.48KB]
│       │   │   │   │   │   ├── assets/ [269.75KB]
│       │   │   │   │   │   │   ├── 0.CoP0xUc3.css (2 LoC | 51.23KB)
│       │   │   │   │   │   │   ├── 14.B6HipyJ3.css (1 LoC | 2.29KB)
│       │   │   │   │   │   │   ├── 22.B6iX8CsZ.css (1 LoC | 2.85KB)
│       │   │   │   │   │   │   ├── inter-cyrillic-ext-wght-normal.BOeWTOD4.woff2 (0 LoC | 25.35KB)
│       │   │   │   │   │   │   ├── inter-cyrillic-wght-normal.DqGufNeO.woff2 (0 LoC | 18.31KB)
│       │   │   │   │   │   │   ├── inter-greek-ext-wght-normal.DlzME5K_.woff2 (0 LoC | 10.97KB)
│       │   │   │   │   │   │   ├── inter-greek-wght-normal.CkhJZR-_.woff2 (0 LoC | 18.55KB)
│       │   │   │   │   │   │   ├── inter-latin-ext-wght-normal.DO1Apj_S.woff2 (0 LoC | 83.07KB)
│       │   │   │   │   │   │   ├── inter-latin-wght-normal.Dx4kXJAl.woff2 (0 LoC | 47.12KB)
│       │   │   │   │   │   │   └── inter-vietnamese-wght-normal.CBcvBZtf.woff2 (0 LoC | 10.01KB)
│       │   │   │   │   │   ├── chunks/ [236.41KB]
│       │   │   │   │   │   │   ├── 5LAFKw29.js (1 LoC | 469.00B)
│       │   │   │   │   │   │   ├── B0B7inlp2.js (1 LoC | 420.00B)
│       │   │   │   │   │   │   ├── B4qpqDyc.js (1 LoC | 497.00B)
│       │   │   │   │   │   │   ├── BGXUTgQ3.js (1 LoC | 596.00B)
│       │   │   │   │   │   │   ├── BPcuV6Ja.js (1 LoC | 765.00B)
│       │   │   │   │   │   │   ├── BQXjXzaX.js (1 LoC | 736.00B)
│       │   │   │   │   │   │   ├── BWIeeRrz2.js (1 LoC | 403.00B)
│       │   │   │   │   │   │   ├── BX8Ttaqz.js (1 LoC | 415.00B)
│       │   │   │   │   │   │   ├── BcPwO7ir.js (1 LoC | 2.12KB)
│       │   │   │   │   │   │   ├── BnXcjEjl2.js (1 LoC | 502.00B)
│       │   │   │   │   │   │   ├── BuFlayix.js (1 LoC | 644.00B)
│       │   │   │   │   │   │   ├── Bur_MeiI.js (1 LoC | 35.98KB)
│       │   │   │   │   │   │   ├── C4I-84-d2.js (1 LoC | 467.00B)
│       │   │   │   │   │   │   ├── CG7Ff-WH2.js (1 LoC | 737.00B)
│       │   │   │   │   │   │   ├── CTwHpbHg2.js (1 LoC | 496.00B)
│       │   │   │   │   │   │   ├── CX1zBDsF.js (57 LoC | 62.63KB)
│       │   │   │   │   │   │   ├── Csfxa9a5.js (1 LoC | 3.34KB)
│       │   │   │   │   │   │   ├── D-Z4MMCM.js (1 LoC | 479.00B)
│       │   │   │   │   │   │   ├── D19obVww.js (1 LoC | 421.00B)
│       │   │   │   │   │   │   ├── D9-4UDs1.js (1 LoC | 958.00B)
│       │   │   │   │   │   │   ├── DAJ2ODRE2.js (1 LoC | 562.00B)
│       │   │   │   │   │   │   ├── DAS_HEAu2.js (1 LoC | 424.00B)
│       │   │   │   │   │   │   ├── DdNYMZxg.js (1 LoC | 26.81KB)
│       │   │   │   │   │   │   ├── DiWbKoNB.js (1 LoC | 22.00B)
│       │   │   │   │   │   │   ├── DoGB7tIJ.js (1 LoC | 487.00B)
│       │   │   │   │   │   │   ├── DrEgQiYg.js (1 LoC | 419.00B)
│       │   │   │   │   │   │   ├── DvbA9ipE.js (1 LoC | 1.40KB)
│       │   │   │   │   │   │   ├── DwS73oe9.js (1 LoC | 14.67KB)
│       │   │   │   │   │   │   ├── Dwemfg6S.js (1 LoC | 9.48KB)
│       │   │   │   │   │   │   ├── Dy_jyaQX.js (1 LoC | 393.00B)
│       │   │   │   │   │   │   ├── KzL6abX32.js (1 LoC | 535.00B)
│       │   │   │   │   │   │   ├── VNnweyVz.js (1 LoC | 463.00B)
│       │   │   │   │   │   │   ├── XrGsGetf.js (3 LoC | 56.32KB)
│       │   │   │   │   │   │   ├── _yrx6Rqw.js (1 LoC | 659.00B)
│       │   │   │   │   │   │   ├── dt2e9V9u.js (1 LoC | 444.00B)
│       │   │   │   │   │   │   ├── favrNDRa.js (1 LoC | 3.29KB)
│       │   │   │   │   │   │   ├── gPedstIv.js (1 LoC | 393.00B)
│       │   │   │   │   │   │   ├── iRSPQALs.js (1 LoC | 594.00B)
│       │   │   │   │   │   │   ├── kNaey6uv.js (1 LoC | 1.18KB)
│       │   │   │   │   │   │   ├── vXl1VgzM.js (1 LoC | 1.94KB)
│       │   │   │   │   │   │   ├── x0slt7CE.js (1 LoC | 2.63KB)
│       │   │   │   │   │   │   ├── xihTtKlq.js (1 LoC | 65.00B)
│       │   │   │   │   │   │   └── zHgyInin2.js (1 LoC | 504.00B)
│       │   │   │   │   │   ├── entry/ [7.62KB]
│       │   │   │   │   │   │   ├── app.HUZOUv0x.js (2 LoC | 7.54KB)
│       │   │   │   │   │   │   └── start.DPaTCSzB.js (1 LoC | 82.00B)
│       │   │   │   │   │   └── nodes/ [188.70KB]
│       │   │   │   │   │       ├── 0.CdB3tpRI.js (1 LoC | 9.47KB)
│       │   │   │   │   │       ├── 1.BigpouPx.js (1 LoC | 376.00B)
│       │   │   │   │   │       ├── 10.AAoJBPB0.js (5 LoC | 10.35KB)
│       │   │   │   │   │       ├── 11.DPE8LU06.js (1 LoC | 10.43KB)
│       │   │   │   │   │       ├── 12.f_m-GOOF.js (1 LoC | 4.47KB)
│       │   │   │   │   │       ├── 13.2RLt7NWr.js (1 LoC | 14.40KB)
│       │   │   │   │   │       ├── 14.C1wMmjhx.js (1 LoC | 5.87KB)
│       │   │   │   │   │       ├── 15.OmBodNEb.js (12 LoC | 9.95KB)
│       │   │   │   │   │       ├── 16.t9pXtLen.js (1 LoC | 7.19KB)
│       │   │   │   │   │       ├── 17.Bl-wMTTv.js (1 LoC | 8.22KB)
│       │   │   │   │   │       ├── 18.C2LAqmlZ.js (1 LoC | 5.74KB)
│       │   │   │   │   │       ├── 19.B53lrLyp.js (1 LoC | 11.86KB)
│       │   │   │   │   │       ├── 2.CqMo-n5C.js (1 LoC | 8.93KB)
│       │   │   │   │   │       ├── 20.DAqZkOdq.js (1 LoC | 4.89KB)
│       │   │   │   │   │       ├── 21.Bgrz7NF2.js (1 LoC | 7.01KB)
│       │   │   │   │   │       ├── 22.Db7toAR7.js (1 LoC | 5.53KB)
│       │   │   │   │   │       ├── 23.CyjdodST.js (1 LoC | 3.46KB)
│       │   │   │   │   │       ├── 3.DLw0arR5.js (1 LoC | 2.31KB)
│       │   │   │   │   │       ├── 4.BXlOpT8R.js (1 LoC | 8.85KB)
│       │   │   │   │   │       ├── 5.9-aU3vnV.js (1 LoC | 8.25KB)
│       │   │   │   │   │       ├── 6.YGYMoF6a.js (12 LoC | 10.42KB)
│       │   │   │   │   │       ├── 7.DdRgk0hK.js (1 LoC | 8.04KB)
│       │   │   │   │   │       ├── 8.BX-AyfJV.js (1 LoC | 9.13KB)
│       │   │   │   │   │       └── 9.Cs-W_Zej.js (1 LoC | 13.58KB)
│       │   │   │   │   └── version.json (1 LoC | 27.00B)
│       │   │   │   └── robots.txt (0 LoC | 63.00B)
│       │   │   ├── prerendered/ [19.00B]
│       │   │   │   └── dependencies/ [19.00B]
│       │   │   │       └── _app/ [19.00B]
│       │   │   │           └── env.js (1 LoC | 19.00B)
│       │   │   └── server/ [1.30MB]
│       │   │       ├── .vite/ [21.08KB]
│       │   │       │   └── manifest.json (816 LoC | 21.08KB)
│       │   │       ├── _app/ [269.78KB]
│       │   │       │   └── immutable/ [269.78KB]
│       │   │       │       └── assets/ [269.78KB]
│       │   │       │           ├── _layout.CcBO_7B7.css (2 LoC | 51.25KB)
│       │   │       │           ├── _page.B6HipyJ3.css (1 LoC | 2.29KB)
│       │   │       │           ├── _page.B6iX8CsZ.css (1 LoC | 2.85KB)
│       │   │       │           ├── inter-cyrillic-ext-wght-normal.BOeWTOD4.woff2 (0 LoC | 25.35KB)
│       │   │       │           ├── inter-cyrillic-wght-normal.DqGufNeO.woff2 (0 LoC | 18.31KB)
│       │   │       │           ├── inter-greek-ext-wght-normal.DlzME5K_.woff2 (0 LoC | 10.97KB)
│       │   │       │           ├── inter-greek-wght-normal.CkhJZR-_.woff2 (0 LoC | 18.55KB)
│       │   │       │           ├── inter-latin-ext-wght-normal.DO1Apj_S.woff2 (0 LoC | 83.07KB)
│       │   │       │           ├── inter-latin-wght-normal.Dx4kXJAl.woff2 (0 LoC | 47.12KB)
│       │   │       │           └── inter-vietnamese-wght-normal.CBcvBZtf.woff2 (0 LoC | 10.01KB)
│       │   │       ├── chunks/ [532.39KB]
│       │   │       │   ├── Icon.js (231 LoC | 13.00KB)
│       │   │       │   ├── activity.js (79 LoC | 4.99KB)
│       │   │       │   ├── arrow-left.js (79 LoC | 4.82KB)
│       │   │       │   ├── auth.svelte.js (113 LoC | 3.07KB)
│       │   │       │   ├── badge.js (117 LoC | 5.18KB)
│       │   │       │   ├── building-2.js (85 LoC | 5.27KB)
│       │   │       │   ├── button.js (71 LoC | 3.71KB)
│       │   │       │   ├── chevron-left.js (79 LoC | 4.77KB)
│       │   │       │   ├── chevron-right.js (79 LoC | 4.77KB)
│       │   │       │   ├── circle-alert.js (97 LoC | 5.11KB)
│       │   │       │   ├── circle-check-big.js (79 LoC | 4.90KB)
│       │   │       │   ├── client.js (1471 LoC | 46.34KB)
│       │   │       │   ├── clock.js (83 LoC | 4.83KB)
│       │   │       │   ├── context.js (1061 LoC | 32.36KB)
│       │   │       │   ├── cpu.js (106 LoC | 5.75KB)
│       │   │       │   ├── createQuery.js (456 LoC | 18.29KB)
│       │   │       │   ├── dev.js (4757 LoC | 147.04KB)
│       │   │       │   ├── download.js (83 LoC | 4.96KB)
│       │   │       │   ├── environment.js (45 LoC | 1.27KB)
│       │   │       │   ├── exports.js (370 LoC | 11.53KB)
│       │   │       │   ├── file-text.js (85 LoC | 5.27KB)
│       │   │       │   ├── globe.js (87 LoC | 5.00KB)
│       │   │       │   ├── index-server.js (145 LoC | 4.78KB)
│       │   │       │   ├── internal.js (624 LoC | 21.33KB)
│       │   │       │   ├── layers.js (83 LoC | 5.38KB)
│       │   │       │   ├── mail.js (85 LoC | 4.95KB)
│       │   │       │   ├── monitor.js (99 LoC | 5.13KB)
│       │   │       │   ├── mutation.js (216 LoC | 5.60KB)
│       │   │       │   ├── navigation.js (2 LoC | 33.00B)
│       │   │       │   ├── plus.js (79 LoC | 4.77KB)
│       │   │       │   ├── search.js (83 LoC | 4.85KB)
│       │   │       │   ├── server.js (108 LoC | 5.35KB)
│       │   │       │   ├── settings.js (83 LoC | 5.57KB)
│       │   │       │   ├── shared.js (711 LoC | 24.34KB)
│       │   │       │   ├── shield.js (79 LoC | 5.07KB)
│       │   │       │   ├── signal.js (166 LoC | 10.17KB)
│       │   │       │   ├── skeleton.js (16 LoC | 575.00B)
│       │   │       │   ├── smartphone.js (425 LoC | 25.32KB)
│       │   │       │   ├── state.js (98 LoC | 3.23KB)
│       │   │       │   ├── table.js (108 LoC | 3.91KB)
│       │   │       │   ├── thermometer.js (79 LoC | 4.83KB)
│       │   │       │   ├── triangle-alert.js (83 LoC | 5.05KB)
│       │   │       │   ├── upload.js (83 LoC | 4.95KB)
│       │   │       │   ├── utils.js (863 LoC | 29.94KB)
│       │   │       │   └── wifi.js (84 LoC | 5.03KB)
│       │   │       ├── entries/ [293.20KB]
│       │   │       │   ├── fallbacks/ [498.00B]
│       │   │       │   │   └── error.svelte.js (10 LoC | 498.00B)
│       │   │       │   └── pages/ [292.71KB]
│       │   │       │       ├── _layout.svelte.js (483 LoC | 16.69KB)
│       │   │       │       ├── _layout.ts.js (9 LoC | 183.00B)
│       │   │       │       ├── _page.svelte.js (45 LoC | 2.54KB)
│       │   │       │       ├── dashboard/ [271.28KB]
│       │   │       │       │   ├── _layout.svelte.js (716 LoC | 37.93KB)
│       │   │       │       │   ├── _page.svelte.js (249 LoC | 8.01KB)
│       │   │       │       │   ├── agents/ [29.04KB]
│       │   │       │       │   │   ├── _id_/ [16.75KB]
│       │   │       │       │   │   │   └── _page.svelte.js (449 LoC | 16.75KB)
│       │   │       │       │   │   └── _page.svelte.js (330 LoC | 12.29KB)
│       │   │       │       │   ├── alertas/ [12.05KB]
│       │   │       │       │   │   └── _page.svelte.js (341 LoC | 12.05KB)
│       │   │       │       │   ├── audit/ [17.95KB]
│       │   │       │       │   │   └── _page.svelte.js (394 LoC | 17.95KB)
│       │   │       │       │   ├── discovery/ [60.03KB]
│       │   │       │       │   │   ├── _id_/ [8.41KB]
│       │   │       │       │   │   │   └── _page.svelte.js (144 LoC | 8.41KB)
│       │   │       │       │   │   ├── _page.svelte.js (717 LoC | 24.78KB)
│       │   │       │       │   │   └── scan/ [26.84KB]
│       │   │       │       │   │       └── _page.svelte.js (516 LoC | 26.84KB)
│       │   │       │       │   ├── dispositivos/ [5.79KB]
│       │   │       │       │   │   └── _page.svelte.js (119 LoC | 5.79KB)
│       │   │       │       │   ├── infrastructure/ [31.81KB]
│       │   │       │       │   │   └── _page.svelte.js (741 LoC | 31.81KB)
│       │   │       │       │   ├── jobs/ [3.88KB]
│       │   │       │       │   │   └── settings/ [3.88KB]
│       │   │       │       │   │       └── _page.svelte.js (25 LoC | 3.88KB)
│       │   │       │       │   ├── metricas/ [14.35KB]
│       │   │       │       │   │   └── _page.svelte.js (373 LoC | 14.35KB)
│       │   │       │       │   ├── notifications/ [28.50KB]
│       │   │       │       │   │   ├── _page.svelte.js (418 LoC | 17.46KB)
│       │   │       │       │   │   └── config/ [11.04KB]
│       │   │       │       │   │       └── _page.svelte.js (148 LoC | 11.04KB)
│       │   │       │       │   ├── reports/ [6.07KB]
│       │   │       │       │   │   └── _page.svelte.js (144 LoC | 6.07KB)
│       │   │       │       │   ├── security/ [3.49KB]
│       │   │       │       │   │   └── _page.svelte.js (61 LoC | 3.49KB)
│       │   │       │       │   ├── sedes/ [5.79KB]
│       │   │       │       │   │   └── _page.svelte.js (115 LoC | 5.79KB)
│       │   │       │       │   ├── settings/ [5.84KB]
│       │   │       │       │   │   └── _page.svelte.js (69 LoC | 5.84KB)
│       │   │       │       │   └── workers/ [758.00B]
│       │   │       │       │       └── _page.svelte.js (13 LoC | 758.00B)
│       │   │       │       └── login/ [2.02KB]
│       │   │       │           └── _page.svelte.js (19 LoC | 2.02KB)
│       │   │       ├── index.js (3813 LoC | 131.53KB)
│       │   │       ├── internal.js (3 LoC | 409.00B)
│       │   │       ├── manifest-full.js (197 LoC | 5.53KB)
│       │   │       ├── manifest.js (197 LoC | 5.53KB)
│       │   │       ├── nodes/ [15.56KB]
│       │   │       │   ├── 0.js (12 LoC | 1.08KB)
│       │   │       │   ├── 1.js (6 LoC | 467.00B)
│       │   │       │   ├── 10.js (6 LoC | 708.00B)
│       │   │       │   ├── 11.js (6 LoC | 998.00B)
│       │   │       │   ├── 12.js (6 LoC | 598.00B)
│       │   │       │   ├── 13.js (6 LoC | 707.00B)
│       │   │       │   ├── 14.js (6 LoC | 420.00B)
│       │   │       │   ├── 15.js (6 LoC | 700.00B)
│       │   │       │   ├── 16.js (6 LoC | 777.00B)
│       │   │       │   ├── 17.js (6 LoC | 533.00B)
│       │   │       │   ├── 18.js (6 LoC | 592.00B)
│       │   │       │   ├── 19.js (6 LoC | 376.00B)
│       │   │       │   ├── 2.js (6 LoC | 984.00B)
│       │   │       │   ├── 20.js (6 LoC | 626.00B)
│       │   │       │   ├── 21.js (6 LoC | 412.00B)
│       │   │       │   ├── 22.js (6 LoC | 414.00B)
│       │   │       │   ├── 23.js (6 LoC | 507.00B)
│       │   │       │   ├── 3.js (6 LoC | 355.00B)
│       │   │       │   ├── 4.js (6 LoC | 692.00B)
│       │   │       │   ├── 5.js (6 LoC | 807.00B)
│       │   │       │   ├── 6.js (6 LoC | 846.00B)
│       │   │       │   ├── 7.js (6 LoC | 662.00B)
│       │   │       │   ├── 8.js (6 LoC | 807.00B)
│       │   │       │   └── 9.js (6 LoC | 848.00B)
│       │   │       ├── remote-entry.js (1506 LoC | 53.85KB)
│       │   │       └── stylesheets/ [0.00B]
│       │   ├── tsconfig.json (58 LoC | 1.00KB)
│       │   └── types/ [28.75KB]
│       │       ├── route_meta_data.json (67 LoC | 1.20KB)
│       │       └── src/ [27.55KB]
│       │           └── routes/ [27.55KB]
│       │               ├── $types.d.ts (23 LoC | 2.30KB)
│       │               ├── dashboard/ [23.30KB]
│       │               │   ├── $types.d.ts (21 LoC | 1.94KB)
│       │               │   ├── agents/ [2.45KB]
│       │               │   │   ├── $types.d.ts (15 LoC | 1.17KB)
│       │               │   │   └── [id]/ [1.28KB]
│       │               │   │       └── $types.d.ts (16 LoC | 1.28KB)
│       │               │   ├── alertas/ [1.17KB]
│       │               │   │   └── $types.d.ts (15 LoC | 1.17KB)
│       │               │   ├── audit/ [1.17KB]
│       │               │   │   └── $types.d.ts (15 LoC | 1.17KB)
│       │               │   ├── discovery/ [3.64KB]
│       │               │   │   ├── $types.d.ts (15 LoC | 1.17KB)
│       │               │   │   ├── [id]/ [1.28KB]
│       │               │   │   │   └── $types.d.ts (16 LoC | 1.28KB)
│       │               │   │   └── scan/ [1.19KB]
│       │               │   │       └── $types.d.ts (15 LoC | 1.19KB)
│       │               │   ├── dispositivos/ [1.18KB]
│       │               │   │   └── $types.d.ts (15 LoC | 1.18KB)
│       │               │   ├── infrastructure/ [1.18KB]
│       │               │   │   └── $types.d.ts (15 LoC | 1.18KB)
│       │               │   ├── jobs/ [1.19KB]
│       │               │   │   └── settings/ [1.19KB]
│       │               │   │       └── $types.d.ts (15 LoC | 1.19KB)
│       │               │   ├── metricas/ [1.17KB]
│       │               │   │   └── $types.d.ts (15 LoC | 1.17KB)
│       │               │   ├── notifications/ [2.37KB]
│       │               │   │   ├── $types.d.ts (15 LoC | 1.18KB)
│       │               │   │   └── config/ [1.19KB]
│       │               │   │       └── $types.d.ts (15 LoC | 1.19KB)
│       │               │   ├── reports/ [1.17KB]
│       │               │   │   └── $types.d.ts (15 LoC | 1.17KB)
│       │               │   ├── security/ [1.17KB]
│       │               │   │   └── $types.d.ts (15 LoC | 1.17KB)
│       │               │   ├── sedes/ [1.17KB]
│       │               │   │   └── $types.d.ts (15 LoC | 1.17KB)
│       │               │   ├── settings/ [1.17KB]
│       │               │   │   └── $types.d.ts (15 LoC | 1.17KB)
│       │               │   └── workers/ [1.17KB]
│       │               │       └── $types.d.ts (15 LoC | 1.17KB)
│       │               ├── login/ [1.06KB]
│       │               │   └── $types.d.ts (15 LoC | 1.06KB)
│       │               └── proxy+layout.ts (23 LoC | 913.00B)
│       ├── README.md (27 LoC | 997.00B)
│       ├── build/ [703.99KB]
│       │   ├── _app/ [702.52KB]
│       │   │   ├── env.js (1 LoC | 19.00B)
│       │   │   ├── immutable/ [702.48KB]
│       │   │   │   ├── assets/ [269.75KB]
│       │   │   │   │   ├── 0.CoP0xUc3.css (2 LoC | 51.23KB)
│       │   │   │   │   ├── 14.B6HipyJ3.css (1 LoC | 2.29KB)
│       │   │   │   │   ├── 22.B6iX8CsZ.css (1 LoC | 2.85KB)
│       │   │   │   │   ├── inter-cyrillic-ext-wght-normal.BOeWTOD4.woff2 (0 LoC | 25.35KB)
│       │   │   │   │   ├── inter-cyrillic-wght-normal.DqGufNeO.woff2 (0 LoC | 18.31KB)
│       │   │   │   │   ├── inter-greek-ext-wght-normal.DlzME5K_.woff2 (0 LoC | 10.97KB)
│       │   │   │   │   ├── inter-greek-wght-normal.CkhJZR-_.woff2 (0 LoC | 18.55KB)
│       │   │   │   │   ├── inter-latin-ext-wght-normal.DO1Apj_S.woff2 (0 LoC | 83.07KB)
│       │   │   │   │   ├── inter-latin-wght-normal.Dx4kXJAl.woff2 (0 LoC | 47.12KB)
│       │   │   │   │   └── inter-vietnamese-wght-normal.CBcvBZtf.woff2 (0 LoC | 10.01KB)
│       │   │   │   ├── chunks/ [236.41KB]
│       │   │   │   │   ├── 5LAFKw29.js (1 LoC | 469.00B)
│       │   │   │   │   ├── B0B7inlp2.js (1 LoC | 420.00B)
│       │   │   │   │   ├── B4qpqDyc.js (1 LoC | 497.00B)
│       │   │   │   │   ├── BGXUTgQ3.js (1 LoC | 596.00B)
│       │   │   │   │   ├── BPcuV6Ja.js (1 LoC | 765.00B)
│       │   │   │   │   ├── BQXjXzaX.js (1 LoC | 736.00B)
│       │   │   │   │   ├── BWIeeRrz2.js (1 LoC | 403.00B)
│       │   │   │   │   ├── BX8Ttaqz.js (1 LoC | 415.00B)
│       │   │   │   │   ├── BcPwO7ir.js (1 LoC | 2.12KB)
│       │   │   │   │   ├── BnXcjEjl2.js (1 LoC | 502.00B)
│       │   │   │   │   ├── BuFlayix.js (1 LoC | 644.00B)
│       │   │   │   │   ├── Bur_MeiI.js (1 LoC | 35.98KB)
│       │   │   │   │   ├── C4I-84-d2.js (1 LoC | 467.00B)
│       │   │   │   │   ├── CG7Ff-WH2.js (1 LoC | 737.00B)
│       │   │   │   │   ├── CTwHpbHg2.js (1 LoC | 496.00B)
│       │   │   │   │   ├── CX1zBDsF.js (57 LoC | 62.63KB)
│       │   │   │   │   ├── Csfxa9a5.js (1 LoC | 3.34KB)
│       │   │   │   │   ├── D-Z4MMCM.js (1 LoC | 479.00B)
│       │   │   │   │   ├── D19obVww.js (1 LoC | 421.00B)
│       │   │   │   │   ├── D9-4UDs1.js (1 LoC | 958.00B)
│       │   │   │   │   ├── DAJ2ODRE2.js (1 LoC | 562.00B)
│       │   │   │   │   ├── DAS_HEAu2.js (1 LoC | 424.00B)
│       │   │   │   │   ├── DdNYMZxg.js (1 LoC | 26.81KB)
│       │   │   │   │   ├── DiWbKoNB.js (1 LoC | 22.00B)
│       │   │   │   │   ├── DoGB7tIJ.js (1 LoC | 487.00B)
│       │   │   │   │   ├── DrEgQiYg.js (1 LoC | 419.00B)
│       │   │   │   │   ├── DvbA9ipE.js (1 LoC | 1.40KB)
│       │   │   │   │   ├── DwS73oe9.js (1 LoC | 14.67KB)
│       │   │   │   │   ├── Dwemfg6S.js (1 LoC | 9.48KB)
│       │   │   │   │   ├── Dy_jyaQX.js (1 LoC | 393.00B)
│       │   │   │   │   ├── KzL6abX32.js (1 LoC | 535.00B)
│       │   │   │   │   ├── VNnweyVz.js (1 LoC | 463.00B)
│       │   │   │   │   ├── XrGsGetf.js (3 LoC | 56.32KB)
│       │   │   │   │   ├── _yrx6Rqw.js (1 LoC | 659.00B)
│       │   │   │   │   ├── dt2e9V9u.js (1 LoC | 444.00B)
│       │   │   │   │   ├── favrNDRa.js (1 LoC | 3.29KB)
│       │   │   │   │   ├── gPedstIv.js (1 LoC | 393.00B)
│       │   │   │   │   ├── iRSPQALs.js (1 LoC | 594.00B)
│       │   │   │   │   ├── kNaey6uv.js (1 LoC | 1.18KB)
│       │   │   │   │   ├── vXl1VgzM.js (1 LoC | 1.94KB)
│       │   │   │   │   ├── x0slt7CE.js (1 LoC | 2.63KB)
│       │   │   │   │   ├── xihTtKlq.js (1 LoC | 65.00B)
│       │   │   │   │   └── zHgyInin2.js (1 LoC | 504.00B)
│       │   │   │   ├── entry/ [7.62KB]
│       │   │   │   │   ├── app.HUZOUv0x.js (2 LoC | 7.54KB)
│       │   │   │   │   └── start.DPaTCSzB.js (1 LoC | 82.00B)
│       │   │   │   └── nodes/ [188.70KB]
│       │   │   │       ├── 0.CdB3tpRI.js (1 LoC | 9.47KB)
│       │   │   │       ├── 1.BigpouPx.js (1 LoC | 376.00B)
│       │   │   │       ├── 10.AAoJBPB0.js (5 LoC | 10.35KB)
│       │   │   │       ├── 11.DPE8LU06.js (1 LoC | 10.43KB)
│       │   │   │       ├── 12.f_m-GOOF.js (1 LoC | 4.47KB)
│       │   │   │       ├── 13.2RLt7NWr.js (1 LoC | 14.40KB)
│       │   │   │       ├── 14.C1wMmjhx.js (1 LoC | 5.87KB)
│       │   │   │       ├── 15.OmBodNEb.js (12 LoC | 9.95KB)
│       │   │   │       ├── 16.t9pXtLen.js (1 LoC | 7.19KB)
│       │   │   │       ├── 17.Bl-wMTTv.js (1 LoC | 8.22KB)
│       │   │   │       ├── 18.C2LAqmlZ.js (1 LoC | 5.74KB)
│       │   │   │       ├── 19.B53lrLyp.js (1 LoC | 11.86KB)
│       │   │   │       ├── 2.CqMo-n5C.js (1 LoC | 8.93KB)
│       │   │   │       ├── 20.DAqZkOdq.js (1 LoC | 4.89KB)
│       │   │   │       ├── 21.Bgrz7NF2.js (1 LoC | 7.01KB)
│       │   │   │       ├── 22.Db7toAR7.js (1 LoC | 5.53KB)
│       │   │   │       ├── 23.CyjdodST.js (1 LoC | 3.46KB)
│       │   │   │       ├── 3.DLw0arR5.js (1 LoC | 2.31KB)
│       │   │   │       ├── 4.BXlOpT8R.js (1 LoC | 8.85KB)
│       │   │   │       ├── 5.9-aU3vnV.js (1 LoC | 8.25KB)
│       │   │   │       ├── 6.YGYMoF6a.js (12 LoC | 10.42KB)
│       │   │   │       ├── 7.DdRgk0hK.js (1 LoC | 8.04KB)
│       │   │   │       ├── 8.BX-AyfJV.js (1 LoC | 9.13KB)
│       │   │   │       └── 9.Cs-W_Zej.js (1 LoC | 13.58KB)
│       │   │   └── version.json (1 LoC | 27.00B)
│       │   ├── index.html (37 LoC | 1.41KB)
│       │   └── robots.txt (0 LoC | 63.00B)
│       ├── components.json (20 LoC | 440.00B)
│       ├── eslint.config.js (42 LoC | 1.33KB)
│       ├── nginx.conf (0 LoC | 958.00B)
│       ├── package.json (57 LoC | 1.59KB)
│       ├── pnpm-lock.yaml (2106 LoC | 91.45KB)
│       ├── src/ [248.03KB]
│       │   ├── app.css (124 LoC | 4.08KB)
│       │   ├── app.d.ts (12 LoC | 274.00B)
│       │   ├── app.html (12 LoC | 331.00B)
│       │   ├── lib/ [31.07KB]
│       │   │   ├── assets/ [1.53KB]
│       │   │   │   └── favicon.svg (0 LoC | 1.53KB)
│       │   │   ├── auth.svelte.ts (152 LoC | 5.36KB)
│       │   │   ├── components/ [22.55KB]
│       │   │   │   ├── layout/ [7.52KB]
│       │   │   │   │   └── Sidebar.svelte (188 LoC | 7.52KB)
│       │   │   │   └── ui/ [15.04KB]
│       │   │   │       ├── badge/ [2.01KB]
│       │   │   │       │   ├── badge.svelte (44 LoC | 1.90KB)
│       │   │   │       │   └── index.ts (2 LoC | 118.00B)
│       │   │   │       ├── button/ [3.99KB]
│       │   │   │       │   ├── button.svelte (77 LoC | 3.74KB)
│       │   │   │       │   └── index.ts (16 LoC | 260.00B)
│       │   │   │       ├── card/ [4.29KB]
│       │   │   │       │   ├── card-action.svelte (21 LoC | 494.00B)
│       │   │   │       │   ├── card-content.svelte (18 LoC | 446.00B)
│       │   │   │       │   ├── card-description.svelte (18 LoC | 446.00B)
│       │   │   │       │   ├── card-footer.svelte (18 LoC | 536.00B)
│       │   │   │       │   ├── card-header.svelte (21 LoC | 707.00B)
│       │   │   │       │   ├── card-title.svelte (18 LoC | 479.00B)
│       │   │   │       │   ├── card.svelte (20 LoC | 740.00B)
│       │   │   │       │   └── index.ts (24 LoC | 548.00B)
│       │   │   │       ├── skeleton/ [523.00B]
│       │   │   │       │   ├── index.ts (6 LoC | 81.00B)
│       │   │   │       │   └── skeleton.svelte (15 LoC | 442.00B)
│       │   │   │       └── table/ [4.23KB]
│       │   │   │           ├── index.ts (27 LoC | 574.00B)
│       │   │   │           ├── table-body.svelte (13 LoC | 443.00B)
│       │   │   │           ├── table-caption.svelte (18 LoC | 451.00B)
│       │   │   │           ├── table-cell.svelte (13 LoC | 454.00B)
│       │   │   │           ├── table-footer.svelte (18 LoC | 479.00B)
│       │   │   │           ├── table-head.svelte (13 LoC | 498.00B)
│       │   │   │           ├── table-header.svelte (18 LoC | 439.00B)
│       │   │   │           ├── table-row.svelte (13 LoC | 481.00B)
│       │   │   │           └── table.svelte (15 LoC | 510.00B)
│       │   │   ├── hooks/ [0.00B]
│       │   │   ├── index.ts (1 LoC | 75.00B)
│       │   │   ├── utils.ts (11 LoC | 631.00B)
│       │   │   └── vitest-examples/ [957.00B]
│       │   │       ├── Welcome.svelte (6 LoC | 158.00B)
│       │   │       ├── Welcome.svelte.spec.ts (11 LoC | 519.00B)
│       │   │       ├── greet.spec.ts (7 LoC | 200.00B)
│       │   │       └── greet.ts (3 LoC | 80.00B)
│       │   └── routes/ [212.29KB]
│       │       ├── +layout.svelte (20 LoC | 527.00B)
│       │       ├── +layout.ts (22 LoC | 883.00B)
│       │       ├── +page.svelte (67 LoC | 2.44KB)
│       │       ├── dashboard/ [203.48KB]
│       │       │   ├── +layout.svelte (12 LoC | 377.00B)
│       │       │   ├── +page.svelte (235 LoC | 7.92KB)
│       │       │   ├── agents/ [25.86KB]
│       │       │   │   ├── +page.svelte (267 LoC | 12.19KB)
│       │       │   │   └── [id]/ [13.67KB]
│       │       │   │       └── +page.svelte (322 LoC | 13.67KB)
│       │       │   ├── alertas/ [10.93KB]
│       │       │   │   └── +page.svelte (239 LoC | 10.93KB)
│       │       │   ├── audit/ [13.27KB]
│       │       │   │   └── +page.svelte (271 LoC | 13.27KB)
│       │       │   ├── discovery/ [33.18KB]
│       │       │   │   ├── +page.svelte (488 LoC | 13.45KB)
│       │       │   │   ├── [id]/ [9.45KB]
│       │       │   │   │   └── +page.svelte (350 LoC | 9.45KB)
│       │       │   │   └── scan/ [10.27KB]
│       │       │   │       └── +page.svelte (344 LoC | 10.27KB)
│       │       │   ├── dispositivos/ [5.45KB]
│       │       │   │   └── +page.svelte (126 LoC | 5.45KB)
│       │       │   ├── infrastructure/ [16.78KB]
│       │       │   │   └── +page.svelte (382 LoC | 16.78KB)
│       │       │   ├── jobs/ [10.07KB]
│       │       │   │   └── settings/ [10.07KB]
│       │       │   │       └── +page.svelte (372 LoC | 10.07KB)
│       │       │   ├── metricas/ [13.95KB]
│       │       │   │   └── +page.svelte (309 LoC | 13.95KB)
│       │       │   ├── notifications/ [21.37KB]
│       │       │   │   ├── +page.svelte (234 LoC | 10.33KB)
│       │       │   │   └── config/ [11.03KB]
│       │       │   │       └── +page.svelte (243 LoC | 11.03KB)
│       │       │   ├── reports/ [7.24KB]
│       │       │   │   └── +page.svelte (193 LoC | 7.24KB)
│       │       │   ├── security/ [15.94KB]
│       │       │   │   └── +page.svelte (425 LoC | 15.94KB)
│       │       │   ├── sedes/ [4.33KB]
│       │       │   │   └── +page.svelte (138 LoC | 4.33KB)
│       │       │   ├── settings/ [8.10KB]
│       │       │   │   └── +page.svelte (235 LoC | 8.10KB)
│       │       │   └── workers/ [8.74KB]
│       │       │       └── +page.svelte (326 LoC | 8.74KB)
│       │       └── login/ [5.00KB]
│       │           └── +page.svelte (114 LoC | 5.00KB)
│       ├── static/ [63.00B]
│       │   └── robots.txt (0 LoC | 63.00B)
│       ├── svelte.config.js (19 LoC | 772.00B)
│       ├── tsconfig.json (20 LoC | 692.00B)
│       └── vite.config.ts (43 LoC | 986.00B)
├── audit.py (146 LoC | 5.43KB)
├── crates/ [414.23KB]
│   ├── database/ [104.63KB]
│   │   ├── Cargo.toml (18 LoC | 531.00B)
│   │   └── src/ [104.11KB]
│   │       ├── entities/ [18.44KB]
│   │       │   ├── active_alert_entity.rs (37 LoC | 1.43KB)
│   │       │   ├── agent_metrics_entity.rs (25 LoC | 989.00B)
│   │       │   ├── audit_entity.rs (24 LoC | 910.00B)
│   │       │   ├── device_entity.rs (30 LoC | 1.08KB)
│   │       │   ├── discovered_device_entity.rs (31 LoC | 1.14KB)
│   │       │   ├── location_entity.rs (24 LoC | 844.00B)
│   │       │   ├── mod.rs (39 LoC | 1.60KB)
│   │       │   ├── network_file_entity.rs (23 LoC | 867.00B)
│   │       │   ├── network_scan_entity.rs (24 LoC | 849.00B)
│   │       │   ├── notification_channel_entity.rs (22 LoC | 929.00B)
│   │       │   ├── notification_log_entity.rs (39 LoC | 1.40KB)
│   │       │   ├── notification_template_entity.rs (23 LoC | 920.00B)
│   │       │   ├── remote_agent_entity.rs (22 LoC | 778.00B)
│   │       │   ├── role_entity.rs (17 LoC | 642.00B)
│   │       │   ├── security_event_entity.rs (28 LoC | 1013.00B)
│   │       │   ├── system_setting_entity.rs (25 LoC | 883.00B)
│   │       │   ├── used_refresh_token_entity.rs (18 LoC | 691.00B)
│   │       │   ├── user_entity.rs (27 LoC | 887.00B)
│   │       │   └── user_session_entity.rs (26 LoC | 874.00B)
│   │       ├── lib.rs (32 LoC | 1.42KB)
│   │       └── repositories/ [84.25KB]
│   │           ├── audit_repository.rs (133 LoC | 5.55KB)
│   │           ├── auth_repository.rs (149 LoC | 7.17KB)
│   │           ├── dashboard_repository.rs (149 LoC | 5.59KB)
│   │           ├── discovery_repository.rs (343 LoC | 15.23KB)
│   │           ├── mod.rs (23 LoC | 988.00B)
│   │           ├── network_file_repository.rs (150 LoC | 6.27KB)
│   │           ├── notification_repository.rs (215 LoC | 9.75KB)
│   │           ├── report_repository.rs (229 LoC | 8.48KB)
│   │           ├── security_repository.rs (195 LoC | 9.10KB)
│   │           ├── settings_repository.rs (121 LoC | 5.77KB)
│   │           └── telemetry_repository.rs (244 LoC | 10.37KB)
│   ├── domain/ [68.88KB]
│   │   ├── Cargo.toml (14 LoC | 451.00B)
│   │   └── src/ [68.44KB]
│   │       ├── errors.rs (59 LoC | 2.30KB)
│   │       ├── lib.rs (13 LoC | 780.00B)
│   │       └── models/ [65.37KB]
│   │           ├── audit.rs (178 LoC | 6.26KB)
│   │           ├── discovery.rs (308 LoC | 11.17KB)
│   │           ├── infrastructure_file.rs (146 LoC | 5.36KB)
│   │           ├── mod.rs (32 LoC | 1.36KB)
│   │           ├── notification.rs (270 LoC | 10.68KB)
│   │           ├── report.rs (167 LoC | 5.06KB)
│   │           ├── security.rs (262 LoC | 9.08KB)
│   │           ├── session.rs (42 LoC | 1.32KB)
│   │           ├── settings.rs (147 LoC | 4.65KB)
│   │           ├── telemetry.rs (256 LoC | 8.82KB)
│   │           └── user.rs (53 LoC | 1.61KB)
│   ├── infrastructure/ [238.42KB]
│   │   ├── Cargo.toml (34 LoC | 915.00B)
│   │   ├── src/ [205.23KB]
│   │   │   ├── background/ [0.00B]
│   │   │   ├── bin/ [852.00B]
│   │   │   │   └── generate_hash.rs (25 LoC | 852.00B)
│   │   │   ├── config/ [2.74KB]
│   │   │   │   ├── mod.rs (5 LoC | 251.00B)
│   │   │   │   └── runtime_config.rs (69 LoC | 2.50KB)
│   │   │   ├── crypto/ [3.85KB]
│   │   │   │   ├── jwt.rs (56 LoC | 1.82KB)
│   │   │   │   ├── mod.rs (8 LoC | 309.00B)
│   │   │   │   ├── opaque.rs (18 LoC | 626.00B)
│   │   │   │   └── password.rs (24 LoC | 1.12KB)
│   │   │   ├── discovery/ [44.37KB]
│   │   │   │   ├── mod.rs (7 LoC | 333.00B)
│   │   │   │   ├── oui_lookup.rs (558 LoC | 26.07KB)
│   │   │   │   └── scan_engine.rs (429 LoC | 17.97KB)
│   │   │   ├── handlers/ [81.21KB]
│   │   │   │   ├── audit_handler.rs (161 LoC | 5.26KB)
│   │   │   │   ├── auth_handler.rs (156 LoC | 6.65KB)
│   │   │   │   ├── dashboard_handler.rs (51 LoC | 1.86KB)
│   │   │   │   ├── devices_handler.rs (23 LoC | 871.00B)
│   │   │   │   ├── discovery_handler.rs (420 LoC | 13.71KB)
│   │   │   │   ├── infrastructure_file_handler.rs (275 LoC | 9.38KB)
│   │   │   │   ├── locations_handler.rs (41 LoC | 1.42KB)
│   │   │   │   ├── mod.rs (38 LoC | 1.63KB)
│   │   │   │   ├── notification_handler.rs (156 LoC | 5.28KB)
│   │   │   │   ├── report_handler.rs (166 LoC | 6.29KB)
│   │   │   │   ├── security_handler.rs (240 LoC | 8.83KB)
│   │   │   │   ├── settings_handler.rs (85 LoC | 3.94KB)
│   │   │   │   ├── telemetry_handler.rs (216 LoC | 7.02KB)
│   │   │   │   ├── worker_config_handler.rs (99 LoC | 3.29KB)
│   │   │   │   └── worker_stats_handler.rs (161 LoC | 5.80KB)
│   │   │   ├── lib.rs (105 LoC | 5.86KB)
│   │   │   ├── middleware/ [7.90KB]
│   │   │   │   ├── audit_middleware.rs (123 LoC | 3.82KB)
│   │   │   │   ├── mod.rs (9 LoC | 391.00B)
│   │   │   │   ├── rate_limit.rs (38 LoC | 1.54KB)
│   │   │   │   └── rbac.rs (51 LoC | 2.16KB)
│   │   │   ├── notifications/ [10.34KB]
│   │   │   │   ├── mod.rs (8 LoC | 346.00B)
│   │   │   │   ├── smtp_adapter.rs (146 LoC | 5.07KB)
│   │   │   │   └── worker.rs (123 LoC | 4.93KB)
│   │   │   ├── reports/ [6.72KB]
│   │   │   │   ├── crypto_signer.rs (84 LoC | 3.04KB)
│   │   │   │   ├── mod.rs (7 LoC | 289.00B)
│   │   │   │   └── pdf_renderer.rs (102 LoC | 3.39KB)
│   │   │   ├── security/ [14.96KB]
│   │   │   │   ├── detection_engine.rs (312 LoC | 14.67KB)
│   │   │   │   └── mod.rs (8 LoC | 296.00B)
│   │   │   ├── storage/ [5.97KB]
│   │   │   │   ├── mod.rs (5 LoC | 236.00B)
│   │   │   │   └── regional_storage.rs (154 LoC | 5.74KB)
│   │   │   ├── telemetry/ [5.36KB]
│   │   │   │   ├── ingestion_engine.rs (138 LoC | 5.14KB)
│   │   │   │   └── mod.rs (5 LoC | 221.00B)
│   │   │   └── workers/ [15.11KB]
│   │   │       ├── mod.rs (245 LoC | 9.78KB)
│   │   │       ├── scheduler.rs (83 LoC | 3.76KB)
│   │   │       └── session_cleanup.rs (38 LoC | 1.58KB)
│   │   └── tests/ [32.30KB]
│   │       ├── discovery_tests.rs (519 LoC | 19.01KB)
│   │       ├── security_tests.rs (178 LoC | 7.08KB)
│   │       └── worker_resilience_tests.rs (171 LoC | 6.21KB)
│   └── shared_types/ [2.30KB]
│       ├── Cargo.toml (13 LoC | 427.00B)
│       └── src/ [1.88KB]
│           └── lib.rs (57 LoC | 1.88KB)
├── data/ [34.65KB]
│   ├── migrations/ [28.53KB]
│   │   ├── 0001_init_auth.sql (34 LoC | 2.12KB)
│   │   ├── 0002_change_sessions_to_datetime.sql (18 LoC | 1.07KB)
│   │   ├── 0003_rtr_replay_protection.sql (12 LoC | 737.00B)
│   │   ├── 0004_locations_and_settings.sql (33 LoC | 2.10KB)
│   │   ├── 0004_notification_engine.sql (74 LoC | 5.55KB)
│   │   ├── 0005_devices_and_alerts.sql (47 LoC | 2.87KB)
│   │   ├── 0005_infrastructure_files.sql (24 LoC | 1.59KB)
│   │   ├── 0006_audit_trail.sql (22 LoC | 1.33KB)
│   │   ├── 0006_locations_hierarchy.sql (9 LoC | 516.00B)
│   │   ├── 0007_agent_telemetry.sql (38 LoC | 2.42KB)
│   │   ├── 0009_sla_aggregation_queries.sql (40 LoC | 1.92KB)
│   │   ├── 0011_security_events.sql (33 LoC | 2.60KB)
│   │   └── 0012_network_discovery.sql (49 LoC | 3.74KB)
│   └── seeds/ [6.11KB]
│       ├── 000_seed_roles.sql (10 LoC | 352.00B)
│       ├── 001_seed_admin.sql (11 LoC | 599.00B)
│       ├── 002_seed_locations.sql (9 LoC | 729.00B)
│       ├── 003_seed_devices_and_alerts.sql (24 LoC | 2.39KB)
│       └── 004_seed_locations_hierarchy.sql (26 LoC | 2.09KB)
├── guia/ [388.50KB]
│   ├── PROMPT_MAESTRO.md (116 LoC | 9.45KB)
│   ├── adr/ [171.49KB]
│   │   ├── ADR-0001-arquitectura-hexagonal.md (94 LoC | 8.49KB)
│   │   ├── ADR-0002-configuracion-tipeada-secretos.md (48 LoC | 5.18KB)
│   │   ├── ADR-0003-stack-backend-rust-axum.md (59 LoC | 6.26KB)
│   │   ├── ADR-0004-persistencia-mysql-seaorm-docker.md (35 LoC | 4.38KB)
│   │   ├── ADR-0005-migraciones-seeding.md (73 LoC | 6.22KB)
│   │   ├── ADR-0006-rbac-sessions-audit.md (44 LoC | 6.06KB)
│   │   ├── ADR-0007-manejo-errores.md (80 LoC | 7.77KB)
│   │   ├── ADR-0008-seguridad-auth-paseto.md (38 LoC | 4.83KB)
│   │   ├── ADR-0009-rate-limiting.md (111 LoC | 6.66KB)
│   │   ├── ADR-0010-testing-calidad.md (102 LoC | 9.86KB)
│   │   ├── ADR-0011-estandares-desarrollo.md (103 LoC | 10.73KB)
│   │   ├── ADR-0012-herramientas-desarrollo.md (127 LoC | 8.86KB)
│   │   ├── ADR-0013-infraestructura-docker-compose.md (83 LoC | 6.85KB)
│   │   ├── ADR-0014-monitoreo-tareas-criticas.md (67 LoC | 6.25KB)
│   │   ├── ADR-0015-tokio-jobs.md (99 LoC | 8.03KB)
│   │   ├── ADR-0016-documentacion-openapi-utoipa.md (70 LoC | 5.45KB)
│   │   ├── ADR-0017-frontend-sveltekit-svelte5.md (108 LoC | 9.50KB)
│   │   ├── ADR-0018-sintonia-cli.md (88 LoC | 8.89KB)
│   │   ├── ADR-0019-coolify-deploy.md (112 LoC | 8.46KB)
│   │   ├── ADR-0020-monitoreo-infraestructura-regional.md (116 LoC | 8.40KB)
│   │   ├── ADR-0021-local-first-sync-offline.md (140 LoC | 7.80KB)
│   │   ├── ADR-0022-agentes-monitoreo-distribuidos.md (148 LoC | 10.08KB)
│   │   ├── comando.md (12 LoC | 890.00B)
│   │   └── justifile.md (112 LoC | 5.62KB)
│   ├── docs/ [57.23KB]
│   │   ├── MODULO_DESCUBRIMIENTO.md (247 LoC | 10.13KB)
│   │   ├── MODULO_MONITOREO.md (267 LoC | 10.10KB)
│   │   ├── MODULO_REPORTES.md (221 LoC | 8.82KB)
│   │   ├── MODULO_SEGURIDAD.md (227 LoC | 10.15KB)
│   │   ├── MODULO_SISTEMA.md (268 LoC | 10.39KB)
│   │   └── README.md (189 LoC | 7.64KB)
│   └── roadmap/ [150.34KB]
│       ├── ROADMAP_MODULO_0_SETUP.md (414 LoC | 18.23KB)
│       ├── ROADMAP_MODULO_10_DESPLIEGUE.md (103 LoC | 10.81KB)
│       ├── ROADMAP_MODULO_11_INTRUSIONES_SEGURIDAD.md (377 LoC | 20.19KB)
│       ├── ROADMAP_MODULO_12_DESCUBRIMIENTO_RED.md (464 LoC | 24.40KB)
│       ├── ROADMAP_MODULO_1_AUTH.md (146 LoC | 14.30KB)
│       ├── ROADMAP_MODULO_2_CONFI_SISTEMA.md (59 LoC | 6.17KB)
│       ├── ROADMAP_MODULO_3_DASHBOARD.md (76 LoC | 6.83KB)
│       ├── ROADMAP_MODULO_4_NOTIFICACIONES.md (89 LoC | 9.03KB)
│       ├── ROADMAP_MODULO_5_ARCHIVO_INFRA_TOPOLOGIA.md (82 LoC | 7.04KB)
│       ├── ROADMAP_MODULO_6_AUDITORIA_DINAMICA_INMUTABLE.md (80 LoC | 8.22KB)
│       ├── ROADMAP_MODULO_7_API_TELEMATRIA.md (79 LoC | 7.91KB)
│       ├── ROADMAP_MODULO_8_TAREA SEGUNDO_PLANO_AUTOMATIZACION.md (93 LoC | 8.74KB)
│       └── ROADMAP_MODULO_9_REPORTES.md (77 LoC | 8.46KB)
├── infrastructure/ [2.88KB]
│   └── docker/ [2.88KB]
│       ├── docker-compose.yaml (37 LoC | 1.02KB)
│       ├── production.Dockerfile (0 LoC | 859.00B)
│       └── production.Dockerfile.web (0 LoC | 1.02KB)
├── justfile (0 LoC | 786.00B)
├── package.json (5 LoC | 69.00B)
├── pnpm-lock.yaml (605 LoC | 24.75KB)
└── tests/ [3.73KB]
    └── settings_thresholds_test.rs (91 LoC | 3.73KB)
```
