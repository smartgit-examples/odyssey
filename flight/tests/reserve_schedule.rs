use odyssey_flight::power::reserve::ReserveSchedule;
#[test]
fn reserve_is_preserved_over_a_one_hour_coast() {
    let schedule = ReserveSchedule {
        capacity_wh: 1000.0,
        fraction: 0.8,
        reserve_fraction: 0.2,
    };
    assert!((schedule.maximum_draw_w(3600.0).unwrap() - 600.0).abs() < 1e-9);
    assert!((schedule.maximum_draw_w(1800.0).unwrap() - 1200.0).abs() < 1e-9);
}

#[test]
fn below_reserve_allows_no_discretionary_draw() {
    let mut schedule = ReserveSchedule {
        capacity_wh: 1000.0,
        fraction: 0.1,
        reserve_fraction: 0.2,
    };
    assert_eq!(schedule.maximum_draw_w(3600.0).unwrap(), 0.0);
    schedule.fraction = schedule.reserve_fraction;
    assert_eq!(schedule.maximum_draw_w(3600.0).unwrap(), 0.0);
}
#[test]
fn zero_horizon_is_rejected() {
    let schedule = ReserveSchedule {
        capacity_wh: 1000.0,
        fraction: 0.8,
        reserve_fraction: 0.2,
    };
    assert!(schedule.maximum_draw_w(0.0).is_err());
}

#[test]
fn invalid_horizons_cannot_authorize_draw() {
    let schedule = ReserveSchedule {
        capacity_wh: 1000.0,
        fraction: 0.8,
        reserve_fraction: 0.2,
    };
    for horizon_s in [
        -1.0,
        0.5,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        86_401.0,
    ] {
        assert!(schedule.maximum_draw_w(horizon_s).is_err());
    }
}
