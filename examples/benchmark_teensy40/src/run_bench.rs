macro_rules! run_bench {
    ($time_us:expr, $uart:expr, $f:ident) => {{
        write!($uart, "{:<10}", core::stringify!($f));

        run_bench!(@run_bench $time_us, $uart, |num| <f32 as micromath::F32Ext>::$f(num));

        writeln!($uart);
    }};

    ($time_us:expr, $uart:expr, $f:ident, $flibm:ident) => {{
        write!($uart, "{:<10}", core::stringify!($f));

        run_bench!(@run_bench $time_us, $uart, |num| <f32 as micromath::F32Ext>::$f(num));
        run_bench!(@run_bench $time_us, $uart, |num| libm::$flibm(num));

        writeln!($uart);
    }};

    (@run_bench $time_us:expr, $uart:expr, $f:expr) => {{
        const COUNT: u64 = 4096;

        const VALUE: f32 = 0.12345;

        if !$f(VALUE).is_finite() {
            writeln!($uart, "  ERROR: {} did not produce a finite value!", core::stringify!($f));
        }

        let t_start = $time_us();
        for _ in 0..COUNT {
            run_bench!(@unroll_32 {
                run_bench!(@iteration VALUE, $f);
            })
        }
        let t_end = $time_us();

        let iterations: u64 = COUNT * 32;

        let duration_us = u64::from(t_end - t_start);
        let duration_ps = duration_us * 1_000_000;
        let ps_per_iter = duration_ps / iterations;

        let rounding_corrected_ps_per_iter = ps_per_iter + 50;
        let ns_per_iter_full = rounding_corrected_ps_per_iter / 1000;
        let ns_per_iter_rest = (rounding_corrected_ps_per_iter % 1000) / 100;

        // writeln!($uart, "    {} iterations in {} us", iterations, duration_us);
        // writeln!($uart, "    {} ps/iter", ps_per_iter);
        write!($uart, "{:>8}.{}", ns_per_iter_full, ns_per_iter_rest);
    }};

    (@unroll_64 $b:block) => {{
        run_bench!(@unroll_32 $b);
        run_bench!(@unroll_32 $b);
    }};
    (@unroll_32 $b:block) => {{
        run_bench!(@unroll_16 $b);
        run_bench!(@unroll_16 $b);
    }};
    (@unroll_16 $b:block) => {{
        run_bench!(@unroll_8 $b);
        run_bench!(@unroll_8 $b);
    }};
    (@unroll_8 $b:block) => {{
        run_bench!(@unroll_4 $b);
        run_bench!(@unroll_4 $b);
    }};
    (@unroll_4 $b:block) => {{
        run_bench!(@unroll_2 $b);
        run_bench!(@unroll_2 $b);
    }};
    (@unroll_2 $b:block) => {{
        {$b}{$b}
    }};

    (@iteration $val:expr, $f:expr) => {{
        const NUM_CONST: f32 = $val;
        let num = core::intrinsics::black_box(NUM_CONST);
        let result = $f(num);
        core::intrinsics::black_box(result);
    }};
}

pub(crate) use run_bench;
