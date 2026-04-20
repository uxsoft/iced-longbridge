//! Linear and band scales for mapping data values into pixel coordinates.

#[derive(Debug, Clone, Copy)]
pub struct Linear {
    pub domain: (f32, f32),
    pub range: (f32, f32),
}

impl Linear {
    pub fn new(domain: (f32, f32), range: (f32, f32)) -> Self {
        Self { domain, range }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_pixel(&self, value: f32) -> f32 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        if (d1 - d0).abs() < f32::EPSILON {
            return (r0 + r1) * 0.5;
        }
        let t = (value - d0) / (d1 - d0);
        r0 + t * (r1 - r0)
    }

    #[allow(dead_code, clippy::wrong_self_convention)]
    pub fn from_pixel(&self, pixel: f32) -> f32 {
        let (d0, d1) = self.domain;
        let (r0, r1) = self.range;
        if (r1 - r0).abs() < f32::EPSILON {
            return d0;
        }
        let t = (pixel - r0) / (r1 - r0);
        d0 + t * (d1 - d0)
    }

    /// Pick ~`count` round tick values over the domain.
    pub fn ticks(&self, count: usize) -> Vec<f32> {
        let (mut d0, mut d1) = self.domain;
        if d0 > d1 {
            std::mem::swap(&mut d0, &mut d1);
        }
        let n = count.max(2);
        let raw_step = (d1 - d0) / (n as f32 - 1.0);
        if raw_step <= 0.0 {
            return vec![d0];
        }
        let exp = raw_step.log10().floor();
        let base = 10f32.powf(exp);
        let mantissa = raw_step / base;
        let nice = if mantissa < 1.5 {
            1.0
        } else if mantissa < 3.0 {
            2.0
        } else if mantissa < 7.0 {
            5.0
        } else {
            10.0
        };
        let step = nice * base;
        let start = (d0 / step).ceil() * step;
        let mut out = Vec::new();
        let mut v = start;
        while v <= d1 + step * 0.001 {
            out.push(v);
            v += step;
        }
        out
    }
}

/// Categorical (band) scale — maps an index into a contiguous slot.
#[derive(Debug, Clone, Copy)]
pub struct Band {
    pub count: usize,
    pub range: (f32, f32),
    pub padding: f32,
}

impl Band {
    pub fn new(count: usize, range: (f32, f32)) -> Self {
        Self {
            count,
            range,
            padding: 0.15,
        }
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p.clamp(0.0, 0.9);
        self
    }

    pub fn step(&self) -> f32 {
        let (r0, r1) = self.range;
        if self.count == 0 {
            return 0.0;
        }
        (r1 - r0) / self.count as f32
    }

    pub fn bandwidth(&self) -> f32 {
        self.step() * (1.0 - self.padding)
    }

    /// Pixel centre of band `i`.
    pub fn center(&self, i: usize) -> f32 {
        let (r0, _) = self.range;
        r0 + self.step() * (i as f32 + 0.5)
    }

    /// Pixel left edge of band `i` (accounting for padding).
    pub fn left(&self, i: usize) -> f32 {
        self.center(i) - self.bandwidth() / 2.0
    }
}

/// Compute a tight (min, max) domain over samples with a small padding.
pub fn padded_domain(values: impl Iterator<Item = f32>) -> (f32, f32) {
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for v in values {
        if v.is_finite() {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
    }
    if !min.is_finite() || !max.is_finite() {
        return (0.0, 1.0);
    }
    if (max - min).abs() < f32::EPSILON {
        let pad = (min.abs() * 0.1).max(1.0);
        return (min - pad, max + pad);
    }
    let pad = (max - min) * 0.08;
    (min - pad, max + pad)
}
