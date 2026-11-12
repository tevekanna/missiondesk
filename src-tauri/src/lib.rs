// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2026 Lars Wallström

pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}