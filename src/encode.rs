use std::f32::consts::PI;

pub fn dct(waveform: &Vec<f64>) -> Vec<f64> {
    let arr_len = waveform.len();
    let mut amp = vec![0.0; arr_len];
    /*
        Discrete cosine transform:

        Given some waveform, and assuming it is constructed from the sum of
        cosines of a discrete set of frequencies, we are able to determine
        the amplitudes of that set of frequencies wherein:

        f: frequency list
        a: amplitude list
        w: waveform

        f*a = w

        This can be calculated in O(n^2) if done simply and as quickly as
        O(nlog(n)) using a *fast DCT algorithm

        Note:
        This is used instead of Discrete Fourier Transform (DFT) since it
        has been shown to be measurably faster to compute and more useful
        in lossy data compression (e.g. MP3 audio compression standard)
            - DFT follows the same idea however instead of real-valued
            cosine functions it uses a set of "harmonically-related complex
            exponential functions"
            - DFT is more often used for general spectral analysis tools

    */
    let factor: f64 = (PI / (arr_len as f32)) as f64;
    println!("Discrete Cosine Transform Factor: {factor}");

    for i in 0..arr_len {
        let mut sum = 0.0;
        for j in 0..arr_len {
            sum += waveform[i] * (((j as f64) + 0.5) * (i as f64) * factor).cos();
        }
        amp[i] = sum;
    }
    amp
}
