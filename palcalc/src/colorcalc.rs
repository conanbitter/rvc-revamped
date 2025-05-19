use std::sync::Mutex;

use anyhow::Result;
use image::RgbImage;
use rand::Rng;
use rayon::prelude::*;

use crate::{
    colors::{FloatColor, IntColor},
    interface::{StatusCalculating, Tui},
    palette::Palette,
};

pub struct ColorData(Vec<Vec<Vec<u64>>>);

impl ColorData {
    pub fn new() -> ColorData {
        ColorData(vec![vec![vec![0u64; 256]; 256]; 256])
    }

    pub fn add(&mut self, image: &RgbImage) {
        for pixel in image.enumerate_pixels() {
            let color = pixel.2;
            self.0[color[0] as usize][color[1] as usize][color[2] as usize] += 1;
        }
    }
}

struct ColorPoint {
    color: FloatColor,
    segment: i32,
    count: u64,
    distance: f64,
}

impl ColorPoint {
    fn distance_squared(&mut self, c: FloatColor) -> f64 {
        let dist = self.color.distance_squared(c);
        if dist < self.distance {
            self.distance = dist;
            return dist;
        }
        self.distance
    }
}

pub struct ColorCalc {
    points: Vec<ColorPoint>,
    centroids: Vec<FloatColor>,

    colors: u32,
    point_count: u64,

    total_distance: f64,
    points_changed: u64,

    status: StatusCalculating,
    max_attempts: u32,
    max_steps: u32,

    best_error: f64,
    best_palette: Palette,
}

impl ColorCalc {
    pub fn new(
        color_count: u32,
        colors: ColorData,
        tui: &mut Tui,
        max_attempts: u32,
        max_steps: u32,
    ) -> Result<ColorCalc> {
        let mut total_colors = {
            if color_count > 256 {
                256u64
            } else if color_count < 1 {
                1u64
            } else {
                color_count as u64
            }
        };

        let mut points = vec![];

        for r in 0..256 {
            for g in 0..256 {
                for b in 0..256 {
                    if colors.0[r][g][b] > 0 {
                        points.push(ColorPoint {
                            color: FloatColor::new(r as i32, g as i32, b as i32),
                            segment: 0,
                            count: colors.0[r][g][b],
                            distance: f64::MAX,
                        })
                    }
                }
            }
        }

        let unique_colors = points.len() as u64;
        if total_colors > unique_colors {
            total_colors = unique_colors;
        }

        Ok(ColorCalc {
            colors: color_count,
            points,
            centroids: vec![FloatColor::BLACK; total_colors as usize],
            point_count: unique_colors,
            total_distance: 0.0,
            points_changed: 0,
            max_attempts,
            max_steps,
            status: StatusCalculating::new(tui, max_attempts, max_steps, total_colors as u32)?,
            best_error: 0.0,
            best_palette: Palette::new(),
        })
    }

    fn init_centroids(&mut self) {
        let mut rng = rand::rng();
        self.points.swap(0, rng.random_range(0..self.point_count) as usize);
        for cent_ind in 1..(self.colors - 1) as usize {
            let mut sum = 0.0;
            let cent_color = self.points[cent_ind - 1].color;
            for i in cent_ind - 1..self.point_count as usize {
                sum += self.points[i].distance_squared(cent_color);
            }

            let rnd = sum * rng.random::<f64>();
            sum = 0.0;
            let mut next = self.point_count as usize - 1;
            for i in cent_ind + 1..self.point_count as usize {
                sum += self.points[i].distance;
                if sum > rnd {
                    next = i;
                    break;
                }
            }
            self.points.swap(cent_ind, next);
        }
        for i in 0..self.colors {
            self.centroids[i as usize] = self.points[i as usize].color;
        }
    }

    fn calc_centroids(&mut self) {
        let mut new_centroids = vec![FloatColor::BLACK; self.colors as usize];
        let mut counts = vec![0u64; self.colors as usize];
        for point in &self.points {
            counts[point.segment as usize] += point.count;
            let c = &mut new_centroids[point.segment as usize];
            c.r += point.color.r * (point.count as f64);
            c.g += point.color.g * (point.count as f64);
            c.b += point.color.b * (point.count as f64);
        }

        self.total_distance = 0.0;

        for (i, c) in self.centroids.iter_mut().enumerate() {
            if counts[i] == 0 {
                continue;
            }

            let count = counts[i] as f64;
            new_centroids[i].r /= count;
            new_centroids[i].g /= count;
            new_centroids[i].b /= count;
            self.total_distance += new_centroids[i].distance(*c);
            *c = new_centroids[i];
        }
    }

    fn calc_segments(&mut self) {
        let points_changed = Mutex::new(0u64);

        self.points.par_iter_mut().for_each(|point| {
            let old_seg = point.segment;
            let mut new_seg = old_seg;
            let mut min_dist = point.color.distance(self.centroids[old_seg as usize]);
            for (i, c) in self.centroids.iter().enumerate() {
                let dist = point.color.distance(*c);
                if min_dist > dist {
                    min_dist = dist;
                    new_seg = i as i32;
                }
            }
            if new_seg != old_seg {
                point.segment = new_seg;
                let mut changed = points_changed.lock().unwrap();
                *changed += 1;
            }
        });

        self.points_changed = *points_changed.lock().unwrap();
    }

    fn update_stats(&mut self, tui: &mut Tui, attempt: u32, step: u32, passed: u32) -> Result<()> {
        let step_current;
        let steps_total;
        if attempt == 0 {
            step_current = step;
            steps_total = self.max_attempts * self.max_steps;
        } else {
            step_current = passed + step;
            steps_total = passed + passed / attempt * (self.max_attempts - attempt);
        }

        self.status.update(
            tui,
            attempt,
            step,
            self.points_changed,
            self.total_distance * 100.0,
            step_current,
            steps_total,
        )?;
        Ok(())
    }

    fn calc_error(&self) -> f64 {
        let mut score = 0.0;
        for point in &self.points {
            score += (point.color.distance(self.centroids[point.segment as usize]) * (point.count as f64)).sqrt();
        }
        score
    }

    fn generate_palette(&self) -> Palette {
        let mut result = Palette::new();
        for cent in &self.centroids {
            result.add(IntColor::from(cent));
        }
        result
    }

    pub fn run(&mut self, tui: &mut Tui) -> Result<Palette> {
        let mut steps_passed = 0;
        for a in 0..self.max_attempts {
            self.init_centroids();
            for s in 0..self.max_steps {
                self.calc_segments();
                if self.points_changed == 0 {
                    self.update_stats(tui, a, s, steps_passed)?;
                    steps_passed += s;
                    break;
                }
                self.calc_centroids();
                if self.status.timer.needs_update() || s == self.max_steps - 1 {
                    self.update_stats(tui, a, s, steps_passed)?;
                }
                if s == self.max_steps - 1 {
                    steps_passed += s;
                }
            }
            self.calc_segments();
            let error = self.calc_error();
            if a == 0 || error < self.best_error {
                self.best_error = error;
                self.best_palette = self.generate_palette();
            }
        }

        self.best_palette.sort();
        Ok(self.best_palette.clone())
    }
}
