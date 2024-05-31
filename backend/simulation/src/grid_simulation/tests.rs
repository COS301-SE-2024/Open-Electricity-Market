#[cfg(test)]
mod tests {
    use crate::grid_simulation;
    use crate::grid_simulation::GridElement;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[test]
    fn distribution_line_produce_150() {
        let d = grid_simulation::DistributionLine {
            resistance: 0.2,
            amps: AtomicU64::new(0),
            to: grid_simulation::GridPiece::Nil,
        };
        d.produce(150);
        assert_eq!(
            d.amps.load(Ordering::Relaxed),
            150,
            "Testing distribution line produce function with 150"
        );
    }

    #[test]
    fn distribution_line_produce_0() {
        let d = grid_simulation::DistributionLine {
            resistance: 0.2,
            amps: AtomicU64::new(0),
            to: grid_simulation::GridPiece::Nil,
        };
        d.produce(0);
        assert_eq!(
            d.amps.load(Ordering::Relaxed),
            0,
            "Testing distribution line produce function with 0"
        );
    }

    #[test]
    fn distribution_line_consume_150_when_full() {
        let d = grid_simulation::DistributionLine {
            resistance: 0.2,
            amps: AtomicU64::new(150),
            to: grid_simulation::GridPiece::Nil,
        };
        let consumed = d.consume(150);
        assert_eq!(
            consumed, 150,
            "Testing distribution line consume function with 150 when full"
        );
    }

    #[test]
    fn distribution_line_consume_150_when_half_full() {
        let d = grid_simulation::DistributionLine {
            resistance: 0.2,
            amps: AtomicU64::new(75),
            to: grid_simulation::GridPiece::Nil,
        };
        let consumed = d.consume(150);
        assert_eq!(
            consumed, 75,
            "Testing distribution line consume function with 150 when half full"
        );
    }

    #[test]
    fn distribution_line_consume_150_when_empty() {
        let d = grid_simulation::DistributionLine {
            resistance: 0.2,
            amps: AtomicU64::new(0),
            to: grid_simulation::GridPiece::Nil,
        };
        let consumed = d.consume(150);
        assert_eq!(
            consumed, 0,
            "Testing distribution line consume function with 150 when empty"
        );
    }

    #[test]
    fn distribution_line_get_avg_distribution_line_voltage_single() {
        let d = grid_simulation::DistributionLine {
            resistance: 0.2,
            amps: AtomicU64::new(1118),
            to: grid_simulation::GridPiece::Nil,
        };
        let voltage = d.get_avg_distribution_line_voltage();
        assert_eq!(voltage,223.6,"Testing distribution line get average distribution line function without any connected pieces")
    }
}
