// used for now since this is in development
#[allow(dead_code)]
pub fn dct(_waveform: &Vec<u8>) -> Vec<f64> {
    let amp = vec![0.0];
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
    amp
}
