use optimization_engine::{SolverError, constraints, panoc::PANOCCache, alm::{AlmFactory, NO_MAPPING, NO_JACOBIAN_MAPPING, AlmProblem, AlmCache, AlmOptimizer}};

fn main() {
    let tolerance = 1e-2;
    let n = 2;
    let lbfgs_memory = 20;
    let mut u = [20.4, 3.0];
    let vx_bot = 3.0*1.744;
    let vy_bot = -1.0;
    let d_target = 10.0;
    let t_wanted = 3.0;

    let df = |u: &[f64], grad: &mut [f64]| -> Result<(), SolverError> {
        let t = u[1];
        grad[0] = 0.0;
        grad[1] = 2.0 * (t - t_wanted);
        Ok(())
    };
    let f = |u: &[f64], c: &mut f64| -> Result<(), SolverError> {
        let t = u[1];
        *c = (t-t_wanted)*(t-t_wanted);
        Ok(())
    };
    let f2 = |u: &[f64], res: &mut [f64]| -> Result<(), SolverError> {
        let d = u[0];
        let t = u[1];
        let err = d - f64::sqrt((vx_bot * vx_bot * t * t) + (d_target - vy_bot * t) * (d_target - vy_bot * t));
        res[0] = err;
        Ok(())
    };
    let jf2t = |u: &[f64], d_mul: &[f64], res: &mut [f64]| -> Result<(), SolverError> {
        let t = u[1];
        res[0] = d_mul[0];
        res[1] = -1.0 * (((vx_bot * vx_bot + vy_bot * vy_bot) * t - d_target * vy_bot) / f64::sqrt((vx_bot * vx_bot * t * t) + (d_target - vy_bot * t) * (d_target - vy_bot * t))) * d_mul[0];
        Ok(())
    };

    // define the constraints
    let t_max_r = 10.0;
    let t_max = if vy_bot > 0.0 { f64::min(d_target / vy_bot, t_max_r) } else { t_max_r };
    if t_max < 1.0 { println!("no solution"); return; }
    let upper = [30.0, t_max];
    let bounds = constraints::Rectangle::new(Some(&[0.5, 1.0]), Some(&upper));
    let set_c = constraints::Rectangle::new(Some(&[-0.005]), Some(&[0.005]));
    // TODO: what does set_y do?
    let set_y = constraints::Rectangle::new(Some(&[-0.01]), Some(&[0.01]));

    let factory = AlmFactory::new(
        f,
        df,
        Some(f2),
        Some(jf2t),
        NO_MAPPING,
        NO_JACOBIAN_MAPPING,
        Some(set_c),
        0,
    );

    let alm_problem = AlmProblem::new(
        bounds,
        Some(set_c),
        Some(set_y),
        |u: &[f64], xi: &[f64], cost: &mut f64| -> Result<(), SolverError> {
            factory.psi(u, xi, cost)
        },
        |u: &[f64], xi: &[f64], grad: &mut [f64]| -> Result<(), SolverError> {
            factory.d_psi(u, xi, grad)
        },
        Some(f2),
        NO_MAPPING,
        1,
        0,
    );
    let panoc_cache = PANOCCache::new(n, tolerance, lbfgs_memory);
    let mut alm_cache = AlmCache::new(panoc_cache, 1, 0);

    let mut alm_optimizer = AlmOptimizer::new(&mut alm_cache, alm_problem)
        .with_delta_tolerance(1e-4)
        .with_epsilon_tolerance(1e-4)
        .with_max_outer_iterations(20)
        .with_max_inner_iterations(10)
        .with_initial_penalty(1.0)
        .with_penalty_update_factor(2.0)
        .with_sufficient_decrease_coefficient(0.2);

    let status = alm_optimizer.solve(&mut u);
    dbg!(status);
    dbg!(u);
    let mut err = [0.0];
    f2(&u, &mut err).unwrap();
    dbg!(err);
    f(&u, &mut err[0]).unwrap();
    dbg!(err[0].sqrt());
}
