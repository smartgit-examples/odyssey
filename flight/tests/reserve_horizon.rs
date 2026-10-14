use odyssey_flight::power::reserve::ReserveSchedule;

#[test]
fn extended_coast_window_includes_the_eighteen_hour_boundary() {
    let schedule = ReserveSchedule {
        capacity_wh: 1000.0,
        fraction: 0.8,
        reserve_fraction: 0.2,
    };
    assert!((schedule.maximum_draw_w(43_200.0).unwrap() - 50.0).abs() < 1e-9);
    assert!((schedule.maximum_draw_w(64_800.0).unwrap() - 100.0 / 3.0).abs() < 1e-9);
}

#[test]
fn horizons_above_eighteen_hours_require_a_new_estimate() {
    let schedule = ReserveSchedule {
        capacity_wh: 1000.0,
        fraction: 0.8,
        reserve_fraction: 0.2,
    };
    for horizon_s in [64_800.001, 86_400.0] {
        assert!(schedule.maximum_draw_w(horizon_s).is_err());
    }
}
