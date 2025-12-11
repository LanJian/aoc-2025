use criterion::criterion_main;

use aoc_benchmarking::aoc_benches;
use cafeteria::Cafeteria;
use factory::Factory;
use gift_shop::GiftShop;
use laboratories::Laboratories;
use lobby::Lobby;
use playground::Playground;
use printing_department::PrintingDepartment;
use reactor::Reactor;
use secret_entrance::SecretEntrance;
use trash_compactor::TrashCompactor;
// import_marker

criterion_main! {
    benches
}

aoc_benches! {
    5,
    (
        day_001,
        "../day-001-secret-entrance/input.txt",
        SecretEntrance,
        "Part 1",
        "Part 2"
    ),
    (
        day_002,
        "../day-002-gift-shop/input.txt",
        GiftShop,
        "Part 1",
        "Part 2"
    ),
    (
        day_003,
        "../day-003-lobby/input.txt",
        Lobby,
        "Combined (including parsing)"
    ),
    (
        day_004,
        "../day-004-printing-department/input.txt",
        PrintingDepartment,
        "Combined (including parsing)"
    ),
    (
        day_005,
        "../day-005-cafeteria/input.txt",
        Cafeteria,
        "Part 1",
        "Part 2"
    ),
    (
        day_006,
        "../day-006-trash-compactor/input.txt",
        TrashCompactor,
        "Combined (including parsing)"
    ),
    (
        day_007,
        "../day-007-laboratories/input.txt",
        Laboratories,
        "Part 1",
        "Part 2"
    ),
    (
        day_008,
        "../day-008-playground/input.txt",
        Playground,
        "Combined (including parsing)"
    ),
    (
        day_010,
        "../day-010-factory/input.txt",
        Factory,
        "Part 1",
        "Part 2"
    ),
    (
        day_011,
        "../day-011-reactor/input.txt",
        Reactor,
        "Part 1",
        "Part 2"
    ),
    // bench_marker
}
