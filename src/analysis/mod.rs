use {Circuit, Config, Error, Result};

#[cfg(test)]
mod tests;

/// Temperature analysis.
pub struct Analysis {
    config: Config,
    system: System,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
struct System {
    cores: usize,
    nodes: usize,
    U: Vec<f64>,
    L: Vec<f64>,
    D: Vec<f64>,
    E: Vec<f64>,
    F: Vec<f64>,
}

impl Analysis {
    /// Set up the analysis for a particular problem.
    #[allow(non_snake_case)]
    pub fn new(circuit: &Circuit, config: &Config) -> Result<Analysis> {
        use linear::multiply;
        use linear::symmetric_eigen;
        use matrix::Dense;

        let (nc, nn) = (circuit.cores, circuit.nodes);

        let mut D: Vec<_> = circuit.capacitance.clone().into();
        for i in 0..nn {
            D[i] = (1.0 / D[i]).sqrt();
        }

        let mut A: Vec<_> = Dense::from(&circuit.conductance).into();
        for i in 0..nn {
            for j in 0..nn {
                A[j * nn + i] = -1.0 * D[i] * D[j] * A[j * nn + i];
            }
        }

        let mut U = A; // recycle
        let mut L = vec![0.0; nn];
        if let Err(error) = symmetric_eigen(&mut U, &mut L) {
            return Err(Error(error.to_string()));
        }

        let dt = config.time_step;

        let mut temp1 = vec![0.0; nn];
        let mut temp2 = vec![0.0; nn * nn];

        for i in 0..nn {
            temp1[i] = (dt * L[i]).exp();
        }
        for i in 0..nn {
            for j in 0..nn {
                temp2[j * nn + i] = temp1[i] * U[i * nn + j];
            }
        }

        let mut E = vec![0.0; nn * nn];
        multiply(1.0, &U, &temp2, 1.0, &mut E, nn);

        for i in 0..nn {
            temp1[i] = (temp1[i] - 1.0) / L[i];
        }
        for i in 0..nn {
            for j in 0..nc {
                temp2[j * nn + i] = temp1[i] * U[i * nn + j] * D[j];
            }
        }

        let mut F = vec![0.0; nn * nc];
        multiply(1.0, &U, &temp2[..(nn * nc)], 1.0, &mut F, nn);

        Ok(Analysis {
            config: *config,
            system: System { cores: nc, nodes: nn, L: L, U: U, D: D, E: E, F: F },
        })
    }

    /// Perform transient temperature analysis.
    ///
    /// ## Arguments
    ///
    /// * `P` is an input power profile given as a `cores × steps` matrix;
    ///
    /// * `Q` is the corresponding output temperature profile given as a
    ///   `cores × steps` matrix;
    ///
    /// * `S` is an `nodes × steps` matrix for the internal usage; and
    ///
    /// * `steps` is the number of time steps; see `time_step` in `Config`.
    ///
    /// The structure of the arguments allows one to avoid repetitive memory
    /// allocation if the analysis is to be performed several times.
    #[allow(non_snake_case)]
    pub fn compute_transient(&self, P: &[f64], Q: &mut [f64], S: &mut [f64], steps: usize) {
        use linear::multiply;
        use std::mem::transmute_copy;

        let (nc, nn) = (self.system.cores, self.system.nodes);

        let D = &self.system.D;
        let E = &self.system.E;
        let F = &self.system.F;

        multiply(1.0, F, P, 1.0, S, nn);

        // In the loop below, we need to perform operations on certain slices of
        // `S` and overwrite them with new data. Let us be efficient.
        let Z: &mut [f64] = unsafe { transmute_copy(&S) };

        for i in 1..steps {
            let (j, k) = ((i - 1) * nn, i * nn);
            multiply(1.0, E, &S[j..k], 1.0, &mut Z[k..(k + nn)], nn);
        }

        let ambience = self.config.ambience;
        for i in 0..nc {
            for j in 0..steps {
                Q[nc * j + i] = D[i] * S[nn * j + i] + ambience;
            }
        }
    }
}
