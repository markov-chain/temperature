#![allow(non_snake_case)]

use matrix::format::{Compressed, Conventional, Diagonal};
use matrix::operation::{Multiply, MultiplyInto, SymmetricEigen};
use matrix::{Matrix, Size};
use std::ops::{Deref, DerefMut};
use std::{mem, ptr};

use {Circuit, Config, Result};

#[cfg(test)]
mod tests;

/// Temperature analysis.
pub struct Analysis {
    config: Config,
    system: System,
}

struct System {
    cores: usize,
    nodes: usize,
    spots: usize,
    C: Compressed<f64>,
    E: Conventional<f64>,
    F: Conventional<f64>,
    S: State,
}

struct State(Vec<f64>);

impl Analysis {
    /// Set up the analysis.
    pub fn new(circuit: &Circuit, config: &Config) -> Result<Analysis> {
        let &Circuit {
            ref capacitance, ref conductance, ref distribution, ref aggregation,
        } = circuit;

        let ((nodes, cores), spots) = (distribution.dimensions(), aggregation.rows());
        debug_assert_eq!(aggregation.columns(), nodes);

        let mut D = capacitance.clone();
        for value in D.iter_mut() {
            *value = (1.0 / *value).sqrt();
        }

        let mut A = conductance.clone();
        for (i, j, value) in A.iter_mut() {
            *value *= -D[i] * D[j];
        }

        let mut U = Conventional::from(A);
        let mut L = Diagonal::zero(nodes);
        ok!(SymmetricEigen::decompose(&mut (&mut *U, &mut *L)));

        let mut T1 = Conventional::zero(nodes);
        let mut T2 = Conventional::zero(nodes);

        for i in 0..nodes {
            let factor = ((config.time_step * L[i]).exp() - 1.0) / L[i];
            for j in 0..nodes {
                T1[(i, j)] = factor * U[(j, i)] * D[j];
            }
        }

        T1.multiply_into(distribution, &mut T2.values[..(nodes * cores)]);
        let F = U.multiply(&T2.values[..(nodes * cores)]);

        for i in 0..nodes {
            let factor = (config.time_step * L[i]).exp();
            for j in 0..nodes {
                T1[(i, j)] = factor * U[(j, i)];
            }
        }

        let mut E = T2;
        unsafe { E.erase() };
        U.multiply_into(&T1, &mut E);

        let C = aggregation.multiply(&D);

        Ok(Analysis {
            config: *config,
            system: System {
                cores: cores, nodes: nodes, spots: spots,
                C: C, E: E, F: F, S: State::new(nodes),
            },
        })
    }

    /// Perform the analysis.
    pub fn step(&mut self, P: &[f64], Q: &mut [f64]) {
        let Config { ambience, .. } = self.config;
        let System { cores, nodes, spots, ref C, ref E, ref F, ref mut S } = self.system;

        debug_assert_eq!(P.len() % cores, 0);
        debug_assert_eq!(Q.len() % spots, 0);

        let steps = P.len() / cores;
        debug_assert_eq!(steps, Q.len() / spots);
        debug_assert!(steps > 0);

        S.next(nodes, steps);
        F.multiply_into(P, &mut S[nodes..]);

        for i in 0..steps {
            let (from, into) = S[(i * nodes)..((i + 2) * nodes)].split_at_mut(nodes);
            E.multiply_into(from, into);
        }

        for value in Q.iter_mut() {
            *value = ambience;
        }
        C.multiply_into(&S[nodes..], Q);
    }
}

impl State {
    fn new(nodes: usize) -> State {
        State(vec![0.0; 2 * nodes])
    }

    fn next(&mut self, nodes: usize, steps: usize) {
        let buffer = &mut self.0;
        let current = buffer.len();
        let required = (steps + 1) * nodes;
        debug_assert!(current >= nodes && current % nodes == 0);

        unsafe {
            if buffer.capacity() < required {
                let mut new = vec![0.0; nodes * (steps + 1)];
                ptr::copy_nonoverlapping(&buffer[current - nodes], new.as_mut_ptr(), nodes);
                mem::replace(buffer, new);
            } else {
                ptr::copy_nonoverlapping(&buffer[current - nodes], buffer.as_mut_ptr(), nodes);
                ptr::write_bytes(&mut buffer[nodes], 0, required - nodes);
                buffer.set_len(required);
            }
        }
    }
}

impl Deref for State {
    type Target = [f64];

    #[inline(always)]
    fn deref(&self) -> &[f64] {
        &self.0
    }
}

impl DerefMut for State {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut [f64] {
        &mut self.0
    }
}
