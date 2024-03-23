use rustfft::{FftPlanner, num_complex::Complex};

pub fn perform_fft(data: &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(data.len());

    let mut buffer: Vec<Complex<f32>> = data.iter().map(|&f| Complex::new(f, 0.0)).collect();
    
    fft.process(&mut buffer);

    buffer.iter().map(|c| c.norm()).collect()
}
