//! # Wire Harmony
//!
//! Wire harmonics for signal transmission analysis.
//! Models wire media, wave propagation, resonance, harmonic frequencies,
//! and constructive/destructive wave interference.

use std::f64::consts::PI;

// ── wire ────────────────────────────────────────────────────────────────────

/// A wire medium with physical properties for signal transmission.
#[derive(Debug, Clone)]
pub struct Wire {
    pub length: f64,
    pub tension: f64,
    pub linear_density: f64,
    pub damping: f64,
}

impl Wire {
    pub fn new(length: f64, tension: f64, linear_density: f64, damping: f64) -> Self {
        Self { length, tension, linear_density, damping }
    }

    pub fn wave_speed(&self) -> f64 {
        (self.tension / self.linear_density).sqrt()
    }

    pub fn fundamental_frequency(&self) -> f64 {
        self.wave_speed() / (2.0 * self.length)
    }

    pub fn wavelength(&self, frequency: f64) -> f64 {
        if frequency == 0.0 { return f64::INFINITY; }
        self.wave_speed() / frequency
    }

    pub fn impedance(&self) -> f64 {
        (self.tension * self.linear_density).sqrt()
    }

    pub fn travel_time(&self) -> f64 {
        let speed = self.wave_speed();
        if speed == 0.0 { return f64::INFINITY; }
        self.length / speed
    }

    pub fn energy(&self, amplitude: f64) -> f64 {
        0.5 * self.linear_density * self.length * (2.0 * PI * self.fundamental_frequency() * amplitude).powi(2)
    }

    pub fn with_tension(&self, tension: f64) -> Wire {
        Wire { tension, ..self.clone() }
    }

    pub fn with_damping(&self, damping: f64) -> Wire {
        Wire { damping, ..self.clone() }
    }

    pub fn quality_factor(&self) -> f64 {
        if self.damping == 0.0 { return f64::INFINITY; }
        1.0 / self.damping
    }
}

// ── wave ────────────────────────────────────────────────────────────────────

/// A propagating signal on a wire.
#[derive(Debug, Clone)]
pub struct Wave {
    pub amplitude: f64,
    pub frequency: f64,
    pub phase: f64,
    pub direction: f64, // +1.0 or -1.0
}

impl Wave {
    pub fn new(amplitude: f64, frequency: f64, phase: f64) -> Self {
        Self { amplitude, frequency, phase, direction: 1.0 }
    }

    pub fn forward(amplitude: f64, frequency: f64) -> Self {
        Self { amplitude, frequency, phase: 0.0, direction: 1.0 }
    }

    pub fn backward(amplitude: f64, frequency: f64) -> Self {
        Self { amplitude, frequency, phase: 0.0, direction: -1.0 }
    }

    pub fn displacement(&self, x: f64, t: f64, speed: f64) -> f64 {
        let k = 2.0 * PI * self.frequency / speed;
        let omega = 2.0 * PI * self.frequency;
        self.amplitude * (k * x * self.direction - omega * t + self.phase).sin()
    }

    pub fn velocity(&self, x: f64, t: f64, speed: f64) -> f64 {
        let k = 2.0 * PI * self.frequency / speed;
        let omega = 2.0 * PI * self.frequency;
        -self.amplitude * omega * (k * x * self.direction - omega * t + self.phase).cos()
    }

    pub fn wavelength(&self, speed: f64) -> f64 {
        if self.frequency == 0.0 { return f64::INFINITY; }
        speed / self.frequency
    }

    pub fn period(&self) -> f64 {
        if self.frequency == 0.0 { return f64::INFINITY; }
        1.0 / self.frequency
    }

    pub fn energy_density(&self, linear_density: f64) -> f64 {
        0.5 * linear_density * (2.0 * PI * self.frequency * self.amplitude).powi(2)
    }

    pub fn power(&self, linear_density: f64, speed: f64) -> f64 {
        0.5 * linear_density * speed * (2.0 * PI * self.frequency * self.amplitude).powi(2)
    }

    pub fn with_phase(&self, phase: f64) -> Wave {
        Wave { phase, ..self.clone() }
    }

    pub fn attenuated(&self, factor: f64) -> Wave {
        Wave { amplitude: self.amplitude * factor, ..self.clone() }
    }

    pub fn is_standing(&self, other: &Wave) -> bool {
        (self.frequency - other.frequency).abs() < 1e-10 &&
        self.direction != other.direction &&
        (self.amplitude - other.amplitude).abs() < 1e-10
    }
}

// ── resonance ───────────────────────────────────────────────────────────────

/// Natural frequency matching and resonance behavior.
#[derive(Debug, Clone)]
pub struct Resonance {
    pub natural_freq: f64,
    pub damping_ratio: f64,
}

impl Resonance {
    pub fn new(natural_freq: f64, damping_ratio: f64) -> Self {
        Self { natural_freq, damping_ratio }
    }

    pub fn from_wire(wire: &Wire) -> Self {
        Self { natural_freq: wire.fundamental_frequency(), damping_ratio: wire.damping }
    }

    pub fn resonance_freq(&self) -> f64 {
        if self.damping_ratio >= 1.0 { return 0.0; }
        self.natural_freq * (1.0 - 2.0 * self.damping_ratio.powi(2)).sqrt()
    }

    pub fn response_amplitude(&self, driving_freq: f64) -> f64 {
        let r = driving_freq / self.natural_freq;
        let denom = ((1.0 - r * r).powi(2) + (2.0 * self.damping_ratio * r).powi(2)).sqrt();
        if denom < 1e-10 { return f64::INFINITY; }
        1.0 / denom
    }

    pub fn bandwidth(&self) -> f64 {
        2.0 * self.damping_ratio * self.natural_freq
    }

    pub fn q_factor(&self) -> f64 {
        if self.damping_ratio == 0.0 { return f64::INFINITY; }
        1.0 / (2.0 * self.damping_ratio)
    }

    pub fn is_at_resonance(&self, freq: f64, tolerance: f64) -> bool {
        (freq - self.natural_freq).abs() / self.natural_freq < tolerance
    }

    pub fn phase_shift(&self, driving_freq: f64) -> f64 {
        let r = driving_freq / self.natural_freq;
        (2.0 * self.damping_ratio * r / (1.0 - r * r)).atan()
    }

    pub fn critical_damping() -> f64 { 1.0 }
    pub fn underdamped(damping: f64) -> bool { damping < 1.0 }
    pub fn overdamped(damping: f64) -> bool { damping > 1.0 }
}

// ── harmonic ────────────────────────────────────────────────────────────────

/// Integer frequency multiples of a fundamental.
#[derive(Debug, Clone)]
pub struct Harmonic {
    pub fundamental: f64,
    pub n: u32,
}

impl Harmonic {
    pub fn new(fundamental: f64, n: u32) -> Self {
        Self { fundamental, n }
    }

    pub fn fundamental(fundamental: f64) -> Self {
        Self { fundamental, n: 1 }
    }

    pub fn frequency(&self) -> f64 {
        self.fundamental * self.n as f64
    }

    pub fn wavelength(&self, speed: f64) -> f64 {
        let freq = self.frequency();
        if freq == 0.0 { return f64::INFINITY; }
        speed / freq
    }

    pub fn nodes(&self) -> u32 { self.n + 1 }

    pub fn antinodes(&self) -> u32 { self.n }

    pub fn wavelength_on_wire(&self, length: f64) -> f64 {
        2.0 * length / self.n as f64
    }

    pub fn overtone_number(&self) -> u32 {
        if self.n == 1 { 0 } else { self.n - 1 }
    }

    pub fn is_even(&self) -> bool { self.n.is_multiple_of(2) }

    pub fn is_odd(&self) -> bool { self.n % 2 == 1 }

    pub fn series(fundamental: f64, count: u32) -> Vec<Harmonic> {
        (1..=count).map(|n| Harmonic::new(fundamental, n)).collect()
    }

    pub fn energy_ratio(&self) -> f64 {
        1.0 / self.n.pow(2) as f64
    }

    pub fn displacement_at(&self, x: f64, length: f64, amplitude: f64, t: f64) -> f64 {
        let omega = 2.0 * PI * self.frequency();
        amplitude / self.n as f64 * (self.n as f64 * PI * x / length).sin() * (omega * t).cos()
    }
}

// ── interference ────────────────────────────────────────────────────────────

/// Constructive and destructive wave combination.
pub struct Interference;

impl Interference {
    pub fn superpose(waves: &[Wave], x: f64, t: f64, speed: f64) -> f64 {
        waves.iter().map(|w| w.displacement(x, t, speed)).sum()
    }

    pub fn constructive(a: &Wave, b: &Wave) -> bool {
        (a.frequency - b.frequency).abs() < 1e-10 && (a.phase - b.phase).abs() < 0.1
    }

    pub fn destructive(a: &Wave, b: &Wave) -> bool {
        (a.frequency - b.frequency).abs() < 1e-10 && ((a.phase - b.phase).abs() - PI).abs() < 0.1
    }

    pub fn beat_frequency(a: &Wave, b: &Wave) -> f64 {
        (a.frequency - b.frequency).abs()
    }

    pub fn combine_amplitudes(a: f64, b: f64, phase_diff: f64) -> f64 {
        (a * a + b * b + 2.0 * a * b * phase_diff.cos()).sqrt()
    }

    pub fn standing_wave_pattern(amplitude: f64, n: u32, x: f64, length: f64) -> f64 {
        amplitude * (n as f64 * PI * x / length).sin()
    }

    pub fn interference_intensity(a_amp: f64, b_amp: f64, phase_diff: f64) -> f64 {
        a_amp.powi(2) + b_amp.powi(2) + 2.0 * a_amp * b_amp * phase_diff.cos()
    }

    pub fn coherence(waves: &[Wave]) -> f64 {
        if waves.len() <= 1 { return 1.0; }
        let base_freq = waves[0].frequency;
        let in_phase = waves.iter().filter(|w| (w.frequency - base_freq).abs() < 1e-10).count();
        in_phase as f64 / waves.len() as f64
    }
}

// ── tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod wire_tests {
    use super::*;

    #[test]
    fn test_new() {
        let w = Wire::new(1.0, 100.0, 0.01, 0.01);
        assert!((w.length - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_wave_speed() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.0);
        assert!((w.wave_speed() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_fundamental_frequency() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.0);
        assert!((w.fundamental_frequency() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_wavelength() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.0);
        assert!((w.wavelength(5.0) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_wavelength_zero_freq() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.0);
        assert!(w.wavelength(0.0).is_infinite());
    }

    #[test]
    fn test_impedance() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.0);
        assert!((w.impedance() - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_travel_time() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.0);
        assert!((w.travel_time() - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_energy() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.0);
        let e = w.energy(0.1);
        assert!(e > 0.0);
    }

    #[test]
    fn test_with_tension() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.0);
        let w2 = w.with_tension(400.0);
        assert!((w2.tension - 400.0).abs() < 1e-10);
        assert!((w.tension - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_quality_factor() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.01);
        assert!((w.quality_factor() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_quality_factor_zero_damping() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.0);
        assert!(w.quality_factor().is_infinite());
    }
}

#[cfg(test)]
mod wave_tests {
    use super::*;

    #[test]
    fn test_new() {
        let w = Wave::new(1.0, 440.0, 0.0);
        assert!((w.amplitude - 1.0).abs() < 1e-10);
        assert!((w.frequency - 440.0).abs() < 1e-10);
    }

    #[test]
    fn test_forward_backward() {
        let f = Wave::forward(1.0, 440.0);
        let b = Wave::backward(1.0, 440.0);
        assert!((f.direction - 1.0).abs() < 1e-10);
        assert!((b.direction + 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_displacement_zero_time() {
        let w = Wave::new(1.0, 1.0, 0.0);
        let d = w.displacement(0.0, 0.0, 1.0);
        assert!(d.abs() < 1e-10);
    }

    #[test]
    fn test_displacement_peak() {
        let w = Wave::new(1.0, 1.0, PI / 2.0);
        let d = w.displacement(0.0, 0.0, 1.0);
        assert!((d - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_wavelength() {
        let w = Wave::new(1.0, 1.0, 0.0);
        assert!((w.wavelength(10.0) - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_period() {
        let w = Wave::new(1.0, 2.0, 0.0);
        assert!((w.period() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_energy_density() {
        let w = Wave::new(1.0, 1.0, 0.0);
        let e = w.energy_density(1.0);
        assert!(e > 0.0);
    }

    #[test]
    fn test_power() {
        let w = Wave::new(1.0, 1.0, 0.0);
        let p = w.power(1.0, 10.0);
        assert!(p > 0.0);
    }

    #[test]
    fn test_attenuated() {
        let w = Wave::new(1.0, 1.0, 0.0);
        let a = w.attenuated(0.5);
        assert!((a.amplitude - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_is_standing() {
        let f = Wave::forward(1.0, 1.0);
        let b = Wave::backward(1.0, 1.0);
        assert!(f.is_standing(&b));
    }

    #[test]
    fn test_not_standing() {
        let f = Wave::forward(1.0, 1.0);
        let f2 = Wave::forward(1.0, 1.0);
        assert!(!f.is_standing(&f2));
    }

    #[test]
    fn test_velocity() {
        let w = Wave::new(1.0, 1.0, 0.0);
        let v = w.velocity(0.0, 0.0, 1.0);
        assert!(v.abs() > 0.0);
    }
}

#[cfg(test)]
mod resonance_tests {
    use super::*;

    #[test]
    fn test_new() {
        let r = Resonance::new(440.0, 0.01);
        assert!((r.natural_freq - 440.0).abs() < 1e-10);
    }

    #[test]
    fn test_from_wire() {
        let w = Wire::new(1.0, 100.0, 1.0, 0.05);
        let r = Resonance::from_wire(&w);
        assert!((r.natural_freq - w.fundamental_frequency()).abs() < 1e-10);
    }

    #[test]
    fn test_resonance_freq() {
        let r = Resonance::new(100.0, 0.1);
        let f = r.resonance_freq();
        assert!(f < r.natural_freq);
    }

    #[test]
    fn test_response_at_resonance() {
        let r = Resonance::new(100.0, 0.01);
        let resp = r.response_amplitude(100.0);
        assert!(resp > 10.0); // high Q
    }

    #[test]
    fn test_response_off_resonance() {
        let r = Resonance::new(100.0, 0.1);
        let resp = r.response_amplitude(1000.0);
        assert!(resp < 1.0);
    }

    #[test]
    fn test_bandwidth() {
        let r = Resonance::new(100.0, 0.1);
        assert!((r.bandwidth() - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_q_factor() {
        let r = Resonance::new(100.0, 0.01);
        assert!((r.q_factor() - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_at_resonance() {
        let r = Resonance::new(100.0, 0.1);
        assert!(r.is_at_resonance(100.0, 0.1));
        assert!(!r.is_at_resonance(200.0, 0.1));
    }

    #[test]
    fn test_phase_shift() {
        let r = Resonance::new(100.0, 0.1);
        let ps = r.phase_shift(100.0);
        assert!((ps - PI / 2.0).abs() < 0.1);
    }

    #[test]
    fn test_underdamped() {
        assert!(Resonance::underdamped(0.5));
        assert!(!Resonance::underdamped(1.5));
    }

    #[test]
    fn test_overdamped() {
        assert!(Resonance::overdamped(1.5));
        assert!(!Resonance::overdamped(0.5));
    }

    #[test]
    fn test_critical_damping() {
        assert!((Resonance::critical_damping() - 1.0).abs() < 1e-10);
    }
}

#[cfg(test)]
mod harmonic_tests {
    use super::*;

    #[test]
    fn test_fundamental() {
        let h = Harmonic::fundamental(440.0);
        assert!((h.frequency() - 440.0).abs() < 1e-10);
    }

    #[test]
    fn test_second_harmonic() {
        let h = Harmonic::new(440.0, 2);
        assert!((h.frequency() - 880.0).abs() < 1e-10);
    }

    #[test]
    fn test_third_harmonic() {
        let h = Harmonic::new(440.0, 3);
        assert!((h.frequency() - 1320.0).abs() < 1e-10);
    }

    #[test]
    fn test_wavelength() {
        let h = Harmonic::new(440.0, 2);
        let wl = h.wavelength(340.0);
        let expected = 340.0 / (440.0 * 2.0);
        assert!((wl - expected).abs() < 1e-10);
    }

    #[test]
    fn test_nodes() {
        assert_eq!(Harmonic::new(440.0, 3).nodes(), 4);
    }

    #[test]
    fn test_antinodes() {
        assert_eq!(Harmonic::new(440.0, 3).antinodes(), 3);
    }

    #[test]
    fn test_wavelength_on_wire() {
        let h = Harmonic::new(440.0, 2);
        assert!((h.wavelength_on_wire(1.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_overtone_number() {
        assert_eq!(Harmonic::new(440.0, 1).overtone_number(), 0);
        assert_eq!(Harmonic::new(440.0, 2).overtone_number(), 1);
    }

    #[test]
    fn test_is_even_odd() {
        assert!(Harmonic::new(440.0, 2).is_even());
        assert!(Harmonic::new(440.0, 3).is_odd());
        assert!(!Harmonic::new(440.0, 2).is_odd());
    }

    #[test]
    fn test_series() {
        let series = Harmonic::series(440.0, 5);
        assert_eq!(series.len(), 5);
        assert!((series[0].frequency() - 440.0).abs() < 1e-10);
        assert!((series[4].frequency() - 2200.0).abs() < 1e-10);
    }

    #[test]
    fn test_energy_ratio() {
        let h = Harmonic::new(440.0, 2);
        assert!((h.energy_ratio() - 0.25).abs() < 1e-10);
    }

    #[test]
    fn test_displacement_at_node() {
        let h = Harmonic::new(440.0, 2);
        let d = h.displacement_at(0.5, 1.0, 1.0, 0.0);
        assert!(d.abs() < 1e-10);
    }
}

#[cfg(test)]
mod interference_tests {
    use super::*;

    #[test]
    fn test_superpose_single() {
        let w = Wave::new(1.0, 1.0, 0.0);
        let s = Interference::superpose(&[w], 0.25, 0.0, 1.0);
        assert!((s - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_superpose_empty() {
        let s = Interference::superpose(&[], 0.0, 0.0, 1.0);
        assert_eq!(s, 0.0);
    }

    #[test]
    fn test_constructive() {
        let a = Wave::new(1.0, 1.0, 0.0);
        let b = Wave::new(1.0, 1.0, 0.0);
        assert!(Interference::constructive(&a, &b));
    }

    #[test]
    fn test_not_constructive() {
        let a = Wave::new(1.0, 1.0, 0.0);
        let b = Wave::new(1.0, 1.0, PI);
        assert!(!Interference::constructive(&a, &b));
    }

    #[test]
    fn test_destructive() {
        let a = Wave::new(1.0, 1.0, 0.0);
        let b = Wave::new(1.0, 1.0, PI);
        assert!(Interference::destructive(&a, &b));
    }

    #[test]
    fn test_beat_frequency() {
        let a = Wave::new(1.0, 440.0, 0.0);
        let b = Wave::new(1.0, 442.0, 0.0);
        assert!((Interference::beat_frequency(&a, &b) - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_combine_amplitudes_in_phase() {
        let combined = Interference::combine_amplitudes(1.0, 1.0, 0.0);
        assert!((combined - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_combine_amplitudes_antiphase() {
        let combined = Interference::combine_amplitudes(1.0, 1.0, PI);
        assert!(combined.abs() < 1e-10);
    }

    #[test]
    fn test_standing_wave_pattern() {
        let p = Interference::standing_wave_pattern(1.0, 1, 0.0, 1.0);
        assert!(p.abs() < 1e-10);
    }

    #[test]
    fn test_interference_intensity() {
        let i = Interference::interference_intensity(1.0, 1.0, 0.0);
        assert!((i - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_coherence() {
        let waves = vec![Wave::new(1.0, 440.0, 0.0), Wave::new(1.0, 440.0, 0.0)];
        assert!((Interference::coherence(&waves) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_coherence_mixed() {
        let waves = vec![Wave::new(1.0, 440.0, 0.0), Wave::new(1.0, 880.0, 0.0)];
        let c = Interference::coherence(&waves);
        assert!(c < 1.0);
    }
}
