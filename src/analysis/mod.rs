#![allow(non_snake_case)]

use linear;
use matrix;
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
    D: Vec<f64>,
    E: Vec<f64>,
    F: Vec<f64>,
    S: Vec<f64>,
}

impl Analysis {
    /// Set up the analysis for a particular problem.
    pub fn new(circuit: &Circuit, config: &Config) -> Result<Analysis> {
        let &Circuit { cores, nodes, ref capacitance, ref conductance, .. } = circuit;

        let mut D: Vec<_> = capacitance.clone().into();
        for i in 0..nodes {
            D[i] = (1.0 / D[i]).sqrt();
        }

        let mut A: Vec<_> = matrix::Dense::from(conductance).into();
        for i in 0..nodes {
            for j in 0..nodes {
                A[j * nodes + i] = -D[i] * D[j] * A[j * nodes + i];
            }
        }

        let mut U = A;
        let mut L = vec![0.0; nodes];
        ok!(linear::symmetric_eigen(&mut U, &mut L));

        let mut T = vec![0.0; nodes * nodes];

        for i in 0..nodes {
            let factor = (config.time_step * L[i]).exp();
            for j in 0..nodes {
                T[j * nodes + i] = factor * U[i * nodes + j];
            }
        }

        let mut E = vec![0.0; nodes * nodes];
        linear::multiply(1.0, &U, &T, 1.0, &mut E, nodes);

        for i in 0..nodes {
            let factor = ((config.time_step * L[i]).exp() - 1.0) / L[i];
            for j in 0..cores {
                T[j * nodes + i] = factor * U[i * nodes + j] * D[j];
            }
        }

        let mut F: Vec<_> = vec![0.0; nodes * cores];
        linear::multiply(1.0, &U, &T[..(nodes * cores)], 1.0, &mut F, nodes);

        Ok(Analysis {
            config: *config,
            system: System {
                cores: cores, nodes: nodes,
                D: D, E: E, F: F,
                S: vec![0.0; 2 * nodes],
            },
        })
    }

    /// Perform temperature analysis.
    pub fn step(&mut self, P: &[f64], Q: &mut [f64]) {
        let Config { ambience, .. } = self.config;
        let System { cores, nodes, ref D, ref E, ref F, ref mut S, .. } = self.system;

        debug_assert!(P.len() % cores == 0);
        debug_assert!(Q.len() % cores == 0);

        let steps = P.len() / cores;
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

        for i in 0..cores {
            for j in 0..steps {
                Q[j * cores + i] = D[i] * S[(j + 1) * nodes + i] + ambience;
            }
        }
    }
}
