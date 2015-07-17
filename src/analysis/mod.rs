#![allow(non_snake_case)]

use linear;
use matrix::{Conventional, Size};
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
    C: Vec<f64>,
    E: Vec<f64>,
    F: Vec<f64>,
    S: Vec<f64>,
}

impl Analysis {
    /// Set up the analysis.
    pub fn new(circuit: &Circuit, config: &Config) -> Result<Analysis> {
        let &Circuit {
            ref capacitance, ref conductance, ref distribution, ref aggregation,
        } = circuit;

        let ((nodes, cores), spots) = (distribution.dimensions(), aggregation.rows());
        debug_assert_eq!(aggregation.columns(), nodes);

        let mut D: Vec<_> = capacitance.clone().into();
        for i in 0..nodes {
            D[i] = (1.0 / D[i]).sqrt();
        }

        let mut A: Vec<_> = Conventional::from(conductance).into();
        for i in 0..nodes {
            for j in 0..nodes {
                A[j * nodes + i] = -D[i] * D[j] * A[j * nodes + i];
            }
        }

        let mut U = A;
        let mut L = vec![0.0; nodes];
        ok!(linear::symmetric_eigen(&mut U, &mut L));

        let mut T1 = vec![0.0; nodes * nodes];
        let mut T2 = vec![0.0; nodes * nodes];

        for i in 0..nodes {
            let factor = ((config.time_step * L[i]).exp() - 1.0) / L[i];
            for j in 0..nodes {
                T1[j * nodes + i] = factor * U[i * nodes + j] * D[j];
            }
        }

        let mut F: Vec<_> = Conventional::from(distribution).into();
        linear::multiply(1.0, &T1, &F, 0.0, &mut T2[..(nodes * cores)], nodes);
        linear::multiply(1.0, &U, &T2[..(nodes * cores)], 0.0, &mut F, nodes);

        for i in 0..nodes {
            let factor = (config.time_step * L[i]).exp();
            for j in 0..nodes {
                T1[j * nodes + i] = factor * U[i * nodes + j];
            }
        }

        let mut E = T2;
        linear::multiply(1.0, &U, &T1, 0.0, &mut E, nodes);

        let mut C = aggregation.clone();
        for (_, j, value) in C.iter_mut() {
            *value *= D[j];
        }
        let C: Vec<_> = Conventional::from(C).into();

        Ok(Analysis {
            config: *config,
            system: System {
                cores: cores, nodes: nodes, spots: spots,
                C: C, E: E, F: F, S: vec![0.0; 2 * nodes],
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

        unsafe {
            let current = S.len();
            let required = (steps + 1) * nodes;
            debug_assert!(current >= nodes && current % nodes == 0);

            if S.capacity() < required {
                let mut T = vec![0.0; required];
                ptr::copy_nonoverlapping(&S[current - nodes], T.as_mut_ptr(), nodes);
                mem::replace(S, T);
            } else {
                ptr::copy_nonoverlapping(&S[current - nodes], S.as_mut_ptr(), nodes);
                ptr::write_bytes(&mut S[nodes], 0, required - nodes);
                S.set_len(required);
            }
        }

        linear::multiply(1.0, F, P, 1.0, &mut S[nodes..], nodes);

        for i in 0..steps {
            let (from, into) = S[(i * nodes)..((i + 2) * nodes)].split_at_mut(nodes);
            linear::multiply(1.0, E, from, 1.0, into, nodes);
        }

        linear::multiply(1.0, C, &S[nodes..], 0.0, Q, cores);
        for value in Q.iter_mut() {
            *value += ambience;
        }
    }
}
